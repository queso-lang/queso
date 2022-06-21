import { Token, TokenType } from './token';

const isDigit = (c: string) => /^\d$/.test(c);
const isLetterOrUnderscore = (c: string) => /^[a-zA-Z_]$/.test(c);

export class Lexer {
  private src: string;

  // position in the string
  private from = 0;
  private to = 0;

  // position in the file
  private fileFrom = [1, 1] as [number, number];
  private fileTo = [1, 1] as [number, number];

  constructor(src: string) {
    this.src = src;
  }

  private next = () => {
    const previous = this.src[this.to];
    this.to += 1;
    this.fileTo[1] += 1;
    return previous;
  };

  private peek = (ahead: number) => {
    if (this.to + ahead >= this.src.length) {
      return null;
    }
    return this.src[this.to + ahead];
  };

  private matchToken = (expect: string) => {
    if (this.isEOF() || this.peek(0) !== expect) return false;

    this.to += 1;
    this.fileTo[1] += 1;
  };

  private isEOF = () => this.to >= this.src.length;

  private getCurrentSubstring = () => {
    return this.src.slice(this.from, this.to);
  };

  private createToken = (
    type: TokenType,
    val = this.getCurrentSubstring(),
  ): Token => {
    return {
      pos: {
        from: this.fileFrom,
        to: this.fileTo,
      },
      type,
      val,
    };
  };

  private makeNumber = () => {
    while (isDigit(this.peek(0) ?? '')) {
      this.next();
    }
    if (this.peek(0) !== '.' && isDigit(this.peek(0) ?? '')) {
      this.next();
      while (isDigit(this.peek(0) ?? '')) {
        this.next();
      }
    }

    return this.createToken('Number');
  };

  private makeIdentifier = () => {
    while (isLetterOrUnderscore(this.peek(0) ?? '')) {
      this.next();
    }
    const val = this.getCurrentSubstring();
    return this.createToken(
      [
        'let',
        'mut',
        'in',
        'loop',
        'break',
        'continue',
        'return',
        'catch',
      ].includes(val)
        ? (val.toUpperCase() as any)
        : 'Identifier',
    );
  };

  lexNext = () => {
    this.from = this.to;
    this.fileFrom[1] = this.fileTo[1];

    if (this.isEOF()) {
      return this.createToken('EOF');
    }

    const c = this.next();

    const tokenFn = {
      '(': () => this.createToken('LeftParen'),
      ')': () => this.createToken('RightParen'),
      '[': () => this.createToken('LeftBracket'),
      ']': () => this.createToken('RightBracket'),
      '{': () => this.createToken('LeftBrace'),
      '}': () => this.createToken('RightBrace'),
      '+': () => this.createToken('Plus'),
      '*': () => this.createToken(this.matchToken('*') ? 'StarStar' : 'Star'),
      '-': () => this.createToken(this.matchToken('>') ? 'SlimArrow' : 'Minus'),
      '!': () => this.createToken(this.matchToken('=') ? 'BangEqual' : 'Bang'),
      '<': () => this.createToken(this.matchToken('=') ? 'LessEqual' : 'Less'),
      '>': () =>
        this.createToken(this.matchToken('=') ? 'GreaterEqual' : 'Greater'),
      '|': () =>
        this.createToken(
          this.matchToken('>')
            ? 'Pipe'
            : this.matchToken('|')
            ? 'Or'
            : 'Invalid',
        ),
      '&': () => this.createToken(this.matchToken('&') ? 'And' : 'Invalid'),

      '/': () => {
        if (this.matchToken('/')) {
          while (this.peek(0) !== '\n' && !this.isEOF()) {
            this.next();
          }
          // return this.lexNext();
        }
      },
    }[c];

    const token = (
      tokenFn ??
      (() => {
        if (isDigit(c)) {
          return this.makeNumber();
        }
      })
    )();

    return token;
  };
}

export const tokenize = (src: string) => {};
