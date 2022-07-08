import { Token, TokenType } from './Token';

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

  private matchNext = (expect: string) => {
    if (this.isEOF() || this.peek(0) !== expect) return false;

    this.to += 1;
    this.fileTo[1] += 1;

    return true;
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
    if (this.peek(0) === '.' && isDigit(this.peek(1) ?? '')) {
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
        'null',
        'true',
        'false',
      ].includes(val)
        ? ((val.slice(0, 1).toUpperCase() + val.slice(1)) as any)
        : 'Identifier',
    );
  };

  lexNext = () => {
    this.from = this.to;
    this.fileFrom = [...this.fileTo];

    // console.log('a', this.fileFrom, this.fileTo);

    if (this.isEOF()) {
      return this.createToken('EOF');
    }

    const c = this.next();

    // console.log('b', c, this.fileFrom, this.fileTo);

    const tokenFn = {
      '(': () => this.createToken('LeftParen'),
      ')': () => this.createToken('RightParen'),
      '[': () => this.createToken('LeftBracket'),
      ']': () => this.createToken('RightBracket'),
      '{': () => this.createToken('LeftBrace'),
      '}': () => this.createToken('RightBrace'),
      '+': () => this.createToken('Plus'),
      ',': () => this.createToken('Comma'),
      '.': () => this.createToken('Dot'),
      ';': () => this.createToken('Semi'),
      '*': () => this.createToken(this.matchNext('*') ? 'StarStar' : 'Star'),
      '-': () => this.createToken(this.matchNext('>') ? 'SlimArrow' : 'Minus'),
      '!': () => this.createToken(this.matchNext('=') ? 'BangEqual' : 'Bang'),
      '<': () => this.createToken(this.matchNext('=') ? 'LessEqual' : 'Less'),
      '>': () =>
        this.createToken(this.matchNext('=') ? 'GreaterEqual' : 'Greater'),
      '|': () =>
        this.createToken(
          this.matchNext('>')
            ? 'Pipe'
            : this.matchNext('|')
            ? 'Or'
            : 'Invalid',
        ),
      '&': () => this.createToken(this.matchNext('&') ? 'And' : 'Invalid'),
      '/': () => {
        if (this.matchNext('/')) {
          while (this.peek(0) !== '\n' && !this.isEOF()) {
            this.next();
          }
          return (this.lexNext as () => Token)();
        }
        return this.createToken('Slash');
      },
      '=': () =>
        this.createToken(
          this.matchNext('>')
            ? 'FatArrow'
            : this.matchNext('=')
            ? 'EqualEqual'
            : 'Equal',
        ),

      '\n': () => {
        this.fileTo = [this.fileTo[0] + 1, 1];
        this.fileFrom = [...this.fileTo];
        return (this.lexNext as () => Token)();
      },
    }[c];

    const token = (
      tokenFn ??
      (() => {
        if ([' ', '\t', '\r'].includes(c)) {
          return (this.lexNext as () => Token)();
        }

        if (isDigit(c)) {
          return this.makeNumber();
        }
        if (isLetterOrUnderscore(c)) {
          return this.makeIdentifier();
        }
        return this.createToken('Invalid');
      })
    )();

    return token;
  };
}

export const tokenize = (src: string) => {};
