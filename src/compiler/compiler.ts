import binaryen from 'binaryen';
import { match, P } from 'ts-pattern';
import { Expr, Stmt } from '../parser/AST.js';
import { insertMemoryRuntime } from './runtime/memory.js';
import { insertStd } from './runtime/std.js';

const i32_to_i8 = (num: number) => {
  return [
    0xff & num,
    0xff & (num >> 8),
    0xff & (num >> 16),
    0xff & (num >> 24),
  ];
};

export class Compiler {
  constructor(private m: binaryen.Module) {}

  data: number[] = [];

  private compileStmts = (stmts: Stmt[]) => {
    return this.m.block(
      null,
      stmts.map((stmt, i) =>
        i < stmts.length - 1
          ? this.m.drop(this.compile(stmt))
          : this.compile(stmt),
      ),
    );
  };

  compile = (node: Expr | Stmt): number => {
    return match(node)
      .with(['Expr', P._], ([, [expr]]) => this.compile(expr))
      .with(['ResolvedFn', P._], ([, { params, localCount, body }]) =>
        this.m.addFunction(
          (Math.random() + 1).toString(36).substring(7),
          binaryen.createType(Array(params.length).fill(binaryen.i32)),
          binaryen.i32,
          Array(localCount).fill(binaryen.i32),
          this.compile(body),
        ),
      )
      .with(['ResolvedAccess', P._], ([, { resolution }]) =>
        this.m.local.get((resolution as { local: number }).local, binaryen.i32),
      )
      .with(['ResolvedMutDecl', P._], ([, { expr, id, token }]) =>
        this.m.local.tee(id, this.compile(expr), binaryen.i32),
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
          const charCodes = withoutQuotes.split('').map((x) => x.charCodeAt(0));

          const pos = this.data.length;
          const len = charCodes.length;

          this.data = [...this.data, ...charCodes];
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

        this.m.setMemory(2, 2, 'memory', [
          {
            data: new Uint8Array(new Uint32Array(this.data).buffer),
            offset: this.m.i32.const(0),
          },
        ]);

        const program = this.m.addFunction(
          '~program',
          binaryen.createType([]),
          binaryen.i32,
          Array(localCount).fill(binaryen.i32),
          this.compileStmts(body),
        );

        // const increment = () => {
        //   const val = m.i32.load8_u(0, 1, 10);
        //   const one = m.i32.const(1);
        //   const add = m.i32.add(val, one);
        //   return add;
        //   // const sto = m.i32.store8(0, 1, 10, add);
        //   // return sto;
        // };

        this.m.addFunction(
          '_start',
          binaryen.createType([]),
          binaryen.none,
          [],
          this.m.block(null, [
            this.m.call(
              '~rt/print',
              [this.m.call('~program', [], binaryen.i32)],
              binaryen.none,
            ),
          ]),
        );

        this.m.addFunctionExport('_start', '_start');

        return program;
      })
      .otherwise((val) => {
        console.log(val);
        throw new Error('not implemented');
      });
  };
}
