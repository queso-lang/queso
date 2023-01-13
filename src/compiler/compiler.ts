import binaryen from 'binaryen';
import { match, P } from 'ts-pattern';
import { Expr, Stmt } from '../parser/AST.js';
import { insertMemoryRuntime } from './runtime/memory.js';
import { insertStd } from './runtime/std.js';

const i32_to_i8x4 = (num: number) => {
  return [
    0xff & num,
    0xff & (num >> 8),
    0xff & (num >> 16),
    0xff & (num >> 24),
  ] as const;
};

const i32_to_i8x4_trunc = (num: number) => {
  const arr = i32_to_i8x4(num);
  for (let i = 3; i >= 0; i--) {
    if (arr[i] !== 0) return arr.slice(0, i + 1);
  }
  return arr;
};

function toUTF8Array(str) {
  var utf8: number[] = [];
  for (var i = 0; i < str.length; i++) {
    var charcode = str.charCodeAt(i);
    if (charcode < 0x80) utf8.push(charcode);
    else if (charcode < 0x800) {
      utf8.push(0xc0 | (charcode >> 6), 0x80 | (charcode & 0x3f));
    } else if (charcode < 0xd800 || charcode >= 0xe000) {
      utf8.push(
        0xe0 | (charcode >> 12),
        0x80 | ((charcode >> 6) & 0x3f),
        0x80 | (charcode & 0x3f),
      );
    }
    // surrogate pair
    else {
      i++;
      // UTF-16 encodes 0x10000-0x10FFFF by
      // subtracting 0x10000 and splitting the
      // 20 bits of 0x0-0xFFFFF into two halves
      charcode =
        0x10000 + (((charcode & 0x3ff) << 10) | (str.charCodeAt(i) & 0x3ff));
      utf8.push(
        0xf0 | (charcode >> 18),
        0x80 | ((charcode >> 12) & 0x3f),
        0x80 | ((charcode >> 6) & 0x3f),
        0x80 | (charcode & 0x3f),
      );
    }
  }
  return utf8;
}

export class Compiler {
  constructor(private m: binaryen.Module) {}

  data: number[] = [];
  tableFuncs: string[] = [];
  private scratchIdx = -1;

  private compileStmts = (stmts: Stmt[]) => {
    return this.m.block(
      null,
      stmts.map((stmt, i) =>
        i < stmts.length - 1
          ? this.m.drop(this.compile(stmt))
          : this.compile(stmt),
      ),
      binaryen.i32,
    );
  };

  compile = (node: Expr | Stmt): number => {
    return (
      match(node)
        .with(['Expr', P._], ([, [expr]]) => this.compile(expr))
        .with(
          ['ResolvedFn', P._],
          ([, { params, localCount, body, upvalues }]) => {
            const funcName =
              'fn_' + (Math.random() + 1).toString(36).substring(7);

            const parentScratchIdx = this.scratchIdx;
            this.scratchIdx = params.length + localCount + 1;
            this.m.addFunction(
              funcName,
              binaryen.createType([
                /* env */ binaryen.i32,
                ...Array(params.length).fill(binaryen.i32),
              ]),
              binaryen.i32,
              [
                ...Array(localCount).fill(binaryen.i32),
                /* scratch */ binaryen.i32,
              ],
              this.compile(body),
            );
            this.scratchIdx = parentScratchIdx;

            const tableIdx = this.tableFuncs.length;
            this.tableFuncs.push(funcName);

            return this.m.block(null, [
              this.m.local.set(
                parentScratchIdx,
                this.m.call(
                  '~rt/createValue/closure',
                  [
                    this.m.i32.const(tableIdx),
                    this.m.i32.const(upvalues.length),
                  ],
                  binaryen.i32,
                ),
              ),
              ...upvalues
                .map((upvalue) =>
                  upvalue.isLocal
                    ? this.m.local.get(upvalue.id + 1, binaryen.i32)
                    : this.m.i32.load(
                        upvalue.id,
                        0,
                        this.m.local.get(0, binaryen.i32),
                      ),
                )
                .map((x, i) =>
                  this.m.i32.store(
                    8 + i * 4,
                    0,
                    this.m.i32.const(parentScratchIdx),
                    x,
                  ),
                ),
              this.m.return(this.m.local.get(parentScratchIdx, binaryen.i32)),
            ]);
          },
        )
        .with(['ResolvedAccess', P._], ([, { resolution }]) =>
          'local' in resolution
            ? this.m.local.get(resolution.local + 1, binaryen.i32)
            : this.m.i32.add(
                this.m.i32.const(resolution.upvalue * 4),
                this.m.local.get(0, binaryen.i32),
              ),
        )
        .with(['ResolvedMutDecl', P._], ([, { expr, id, token }]) =>
          this.m.local.tee(id + 1, this.compile(expr), binaryen.i32),
        )
        .with(['NullLiteral', P._], () =>
          this.m.call('~rt/createValue/null', [], binaryen.i32),
        )
        .with(['Constant', P._], ([, [token]]) => {
          if (token.type === 'Number') {
            return this.m.call(
              '~rt/createValue/number',
              [this.m.i32.const(+token.val)],
              binaryen.i32,
            );
          } else if (token.type === 'String') {
            const withoutQuotes = token.val.slice(1, -1);
            // const charCodes = withoutQuotes.split('').map((x) => x.charCodeAt(0));

            const utf8Arr = toUTF8Array(withoutQuotes);

            const pos = this.data.length;
            const len = utf8Arr.length;

            // console.log(utf8Arr);
            this.data = [...this.data, ...utf8Arr];
            return this.m.call(
              '~rt/createValue/string',
              [this.m.i32.const(len), this.m.i32.const(pos)],
              binaryen.i32,
            );
          } else throw new Error('constant expr not implemented');
        })
        .with(['Block', P._], ([, [stmts]]) => this.compileStmts(stmts))
        .with(['ResolvedProgram', P._], ([, { body, localCount }]) => {
          insertMemoryRuntime(this.m);
          insertStd(this.m);

          // (import "wasi_unstable" "fd_write" (func $fd_write (param i32 i32 i32 i32) (result i32)))
          this.m.addFunctionImport(
            '~rt/fd_write',
            'wasi_unstable',
            'fd_write',
            binaryen.createType([
              binaryen.i32,
              binaryen.i32,
              binaryen.i32,
              binaryen.i32,
            ]),
            binaryen.i32,
          );

          this.scratchIdx = localCount + 1;
          const program = this.m.addFunction(
            '~program',
            binaryen.createType([/* dummy env */ binaryen.i32]),
            binaryen.i32,
            [
              ...Array(localCount).fill(binaryen.i32),
              /* scratch */ binaryen.i32,
            ],
            this.compileStmts(body),
          );

          this.m.setMemory(2, 2, 'memory', [
            {
              data: new Uint8Array(this.data),
              offset: this.m.i32.const(0),
            },
          ]);
          // this.m.i32.load 
          // this.m.call_indirect()
          // console.log((binaryen as any)._BinaryenAddTable);
          // (binaryen as any)._BinaryenAddActiveElementSegment
          this.m.addTable(
            '0',
            this.tableFuncs.length,
            this.tableFuncs.length,
            binaryen.funcref,
          );
          // (binaryen as any)._BinaryenAddTable(
          //   this.m.ref,
          //   this.m.i32.const(0),
          //   this.tableFuncs.length,
          //   this.tableFuncs.length,
          //   binaryen.funcref,
          // );
          console.log((this.m as any).addActiveElementSegment.toString());
          (this.m as any).addActiveElementSegment('0', '0', this.tableFuncs);
          // (binaryen as any)._BinaryenAddActiveElementSegment(
          //   this.m.ref,
          //   this.m.i32.const(0),
          //   this.m.i32.const(0),
          //   this.tableFuncs,
          //   this.tableFuncs,
          //   this.m.i32.const(0),
          // );
          // for (const [i, f] of this.tableFuncs.entries()) {
          //   this.m.table.set('0', i, this.m.getFunction(f));
          // }
          // this.m.table;
          // this.m.setFunctionTable(
          //   this.tableFuncs.length,
          //   this.tableFuncs.length,
          //   this.tableFuncs as any,
          //   0,
          // );

          // const increment = () => {
          //   const val = m.i32.load8_u(0, 1, 10);
          //   const one = m.i32.const(1);
          //   const add = m.i32.add(val, one);
          //   return add;
          //   // const sto = m.i32.store8(0, 1, 10, add);
          //   // return sto;
          // };

          // this.m.setStart(

          this.m.addFunction(
            '_start',
            binaryen.createType([]),
            binaryen.none,
            [],
            this.m.block(null, [
              this.m.call(
                '~rt/print',
                [this.m.call('~program', [this.m.i32.const(-1)], binaryen.i32)],
                binaryen.none,
              ),
            ]),
          ),
            // );

            this.m.addFunctionExport('_start', '_start');

          return program;
        })
        // .with(['ResolvedAccess', P._], ([, { resolution }]) => {
        //   if ('local' in resolution) {
        //     return this.m.i32.load(
        //       0,
        //       0,
        //       this.m.local.get(resolution.local + 1, binaryen.i32),
        //     );
        //   } else {
        //     return this.m.i32.load(
        //       resolution.upvalue * 4,
        //       0,
        //       this.m.local.get(0, binaryen.i32),
        //     );
        //   }
        // })
        .with(['FnCall', P._], ([, [callee, args]]) => {
          // console.log(callee, args);
          return this.m.block(
            null,
            [
              this.m.local.set(this.scratchIdx, this.compile(callee)),
              this.m.call_indirect(
                this.m.i32.const(0),
                [],
                binaryen.createType([]),
                binaryen.createType([]),
              ),
            ],
            binaryen.i32,
          );
          return null!;
        })
        .otherwise((val) => {
          console.log(val);
          throw new Error('not implemented');
        })
    );
  };
}
