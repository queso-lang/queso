// entry point

import { readFileSync } from 'fs';
import { ErrorReporter } from './error/ErrorReporter';
import { Lexer } from './lexer/Lexer';
import { TokenStream } from './lexer/TokenStream';
import { Expr, Program, Stmt } from './parser/AST';
import { Parser } from './parser/Parser';

const fileContents = readFileSync('./test.queso', 'utf8');
const lexer = new Lexer(fileContents);
const tokenStream = new TokenStream(lexer);
const parser = new Parser(tokenStream, new ErrorReporter(fileContents));

// console.dir(, { depth: null });

const displayAST = (node: Expr | Stmt): string => {
  if (node[0] === 'Expr') {
    return displayAST(node[1][0]);
  }
  if (node[0] === 'MutDecl') {
    return node[1][1][0] === 'NullLiteral'
      ? `mut ${node[1][0].val}`
      : `mut ${node[1][0].val} = ${displayAST(node[1][1])}`;
  }
  if (node[0] === 'Access') {
    return node[1][0].val;
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
  console.log({ node });

  return JSON.stringify(node);
};
// while (true) {
//   const token = lexer.lexNext();
//   const posString = `[${token.pos.from[0]}:${token.pos.from[1]}-${token.pos.to[0]}:${token.pos.to[1]}]`;
//   console.log(`${posString.padEnd(15)} ${token.type.padEnd(12)} ${token.val}`);

//   if (token.type === 'EOF') break;
// }

// parser.parse();
for (const stmt of parser.parse()) {
  console.log(displayAST(stmt));
}
