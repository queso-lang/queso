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
  if (node[0] === 'Binary') {
    return `(${node[1][1].val} ${displayAST(node[1][0])} ${displayAST(
      node[1][2],
    )})`;
  }
  if (node[0] === 'Constant') {
    return node[1][0].val;
  }
  if (node[0] === 'Error') {
    return 'ERR';
  }
  return '?';
};
parser.parse();
// console.log(displayAST(parser.parse()[0]));

// while (true) {
//   const token = lexer.lexNext();
//   const posString = `[${token.pos.from[0]}:${token.pos.from[1]}-${token.pos.to[0]}:${token.pos.to[1]}]`;
//   console.log(`${posString.padEnd(15)} ${token.type.padEnd(12)} ${token.val}`);

//   if (token.type === 'EOF') break;
// }
