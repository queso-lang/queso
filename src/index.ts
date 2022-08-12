// entry point

import binaryen from 'binaryen';
import { fstat, readFileSync, writeFileSync } from 'fs';
import { Compiler } from './compiler/compiler.js';
import { ErrorReporter } from './error/ErrorReporter.js';
import { Lexer } from './lexer/Lexer.js';
import { TokenStream } from './lexer/TokenStream.js';
import { Expr, Program, Stmt } from './parser/AST.js';
import { Parser } from './parser/Parser.js';
import { Resolver } from './resolver/Resolver.js';
import { insertMemoryRuntime } from './compiler/runtime/memory.js';
import { init, WASI } from '@wasmer/wasi';

const fileContents = readFileSync('./test.queso', 'utf8');
const errorReporter = new ErrorReporter(fileContents);
const lexer = new Lexer(fileContents);
const tokenStream = new TokenStream(lexer);
const parser = new Parser(tokenStream, errorReporter);

// console.dir(, { depth: null });

const displayAST = (node: Expr | Stmt): string => {
  if (node[0] === 'ResolvedProgram') {
    return `
    PROGRAM (${node[1].localCount} locals)
    ${node[1].body.map((stmt) => displayAST(stmt)).join(';\n')}`;
  }
  if (node[0] === 'Expr') {
    return displayAST(node[1][0]);
  }
  if (node[0] === 'MutDecl') {
    return node[1][1][0] === 'NullLiteral'
      ? `mut ${node[1][0].val}`
      : `mut ${node[1][0].val} = ${displayAST(node[1][1])}`;
  }
  if (node[0] === 'ResolvedMutDecl') {
    return node[1].expr[0] === 'NullLiteral'
      ? `mut #${node[1].id}_${node[1].token.val}`
      : `mut #${node[1].id}_${node[1].token.val} = ${displayAST(node[1].expr)}`;
  }
  if (node[0] === 'Access') {
    return node[1][0].val;
  }
  if (node[0] === 'ResolvedAccess') {
    if ('local' in node[1].resolution) {
      return `#${node[1].resolution.local}_${node[1].token.val}`;
    } else {
      return `^${node[1].resolution.upvalue}_${node[1].token.val}`;
    }
  }
  if (node[0] === 'Binary') {
    return `(${displayAST(node[1][0])} ${node[1][1].val} ${displayAST(
      node[1][2],
    )})`;
  }
  if (node[0] === 'Unary') {
    return `(${node[1][0].val} ${displayAST(node[1][1])})`;
  }
  if (['Constant', 'FalseLiteral', 'TrueLiteral'].includes(node[0])) {
    return (node as any)[1][0].val;
  }
  if (node[0] === 'Fn') {
    return `(${node[1][0].map((x) => x.val).join(', ')}) -> ${displayAST(
      node[1][1],
    )}`;
  }
  if (node[0] === 'ResolvedFn') {
    // console.log(node[1].captured);
    return `[${node[1].upvalues
      .map((x, i) => `^${i} = ${x.isLocal ? 'local ' : ''}${x.id}`)
      .join(', ')}][${node[1].localCount} locals](${node[1].params
      .map((x) => x.val)
      .join(', ')}) -> ${displayAST(node[1].body)}`;
  }
  if (node[0] === 'IfElse') {
    return `if ${displayAST(node[1][0])} then {${displayAST(
      node[1][1],
    )}} else {${displayAST(node[1][2])}}`;
  }
  if (node[0] === 'NullLiteral') {
    return `null`;
  }
  if (node[0] === 'Error') {
    return 'ERR';
  }
  if (node[0] === 'FnCall') {
    return `( ${displayAST(node[1][0])} )(${node[1][1].map((x) =>
      displayAST(x),
    )})`;
  }
  if (node[0] === 'Block') {
    return `( ${node[1][0].map((stmt) => displayAST(stmt)).join('; ')} )`;
  }
  // console.dir({ node }, { depth: null });

  return JSON.stringify(node);
};
// while (true) {
//   const token = lexer.lexNext();
//   const posString = `[${token.pos.from[0]}:${token.pos.from[1]}-${token.pos.to[0]}:${token.pos.to[1]}]`;
//   console.log(`${posString.padEnd(15)} ${token.type.padEnd(12)} ${token.val}`);

//   if (token.type === 'EOF') break;
// }

const parsed = parser.parse();

const resolver = new Resolver(errorReporter);

const parsedResolved = resolver.resolve(parsed);

console.log(displayAST(parsedResolved));

// for (const stmt of parsedResolved) {
//   console.log(displayAST(stmt)   + '\n');
// }

const m = new binaryen.Module();

try {
  const compiler = new Compiler(m);
  const compiled = compiler.compile(parsedResolved);

  writeFileSync('out.wat', m.emitText(), { encoding: 'utf-8' });
  // writeFileSync('out.wasm', m.emitBinary());

  // await init();

  // let wasi = new WASI({
  //   env: {},
  //   args: [],
  // });

  // const buf = readFileSync('out2.wasm');

  // const _module = await WebAssembly.compile(new Uint8Array(buf));
  // await wasi.instantiate(_module, {});

  // let exitCode = wasi.start();
  // let stdout = wasi.getStdoutString();

  // console.log(`${stdout}(exit code: ${exitCode})`);
} catch (err) {
  console.log(err);
}
