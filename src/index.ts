// entry point

import { readFileSync } from 'fs';
import { Lexer } from './lexer/Lexer';

const lexer = new Lexer(readFileSync('./test.queso', 'utf8'));

while (true) {
  const token = lexer.lexNext();
  const posString = `[${token.pos.from[0]}:${token.pos.from[1]}-${token.pos.to[0]}:${token.pos.to[1]}]`;
  console.log(`${posString.padEnd(15)} ${token.type.padEnd(12)} ${token.val}`);

  if (token.type === 'EOF') break;
}
