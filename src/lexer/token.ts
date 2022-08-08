import { PositionRange } from '../utils.js';

export type TokenType =
  | 'LeftParen' // open scope
  | 'RightParen' // close scope
  | 'LeftBracket' // open list
  | 'RightBracket' // close list
  | 'LeftBrace' // open object
  | 'RightBrace' // close object
  | 'Minus' // subtraction
  | 'Plus' // addition
  | 'Slash' // division
  | 'Star' // multiplication
  | 'StarStar' // exponentiation
  | 'Comma' // object key:value separator
  | 'Dot' // property access
  | 'Semi' // statement delimeter
  | 'Question'
  | 'Colon'
  | 'Pipe' // piping
  | 'SlimArrow' // function arrow
  | 'FatArrow' // control flow
  | 'Or' // logical or
  | 'And' // logical and
  | 'Bang' // logical negation
  | 'BangEqual' // inquality
  | 'Equal' // assignment
  | 'EqualEqual' // equality
  | 'Greater' // comparison
  | 'GreaterEqual' // comparison
  | 'Less' // comparison
  | 'LessEqual' // comparison
  | 'Identifier' // names
  | 'String' // string literal
  | 'Number' // number literal
  | 'Null' // null literal
  | 'True' // true literal
  | 'False' // false literal
  // keywords:
  | 'Let'
  | 'Mut'
  | 'In'
  | 'Loop'
  | 'Break'
  | 'Continue'
  | 'Return'
  | 'Catch'
  // other:
  | 'EOF'
  | 'Invalid';

export type Token = { type: TokenType; val: string; pos: PositionRange };

// export const createTokenStream = (lexer: Lexer) => {

// }
