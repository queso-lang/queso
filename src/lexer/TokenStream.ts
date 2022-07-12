import { Lexer } from './Lexer';
import { Token, TokenType } from './Token';

export class TokenStream {
  // private cur: Token;

  private tokenList: Token[] = [];
  private idx = 0;

  constructor(private lexer: Lexer) {
    const cur = lexer.lexNext();
    this.tokenList.push(cur);
  }

  next = () => {
    const last = this.tokenList[this.idx];
    this.idx += 1;

    // if the element at that index doesn't exist,
    // request another token from the lexer
    if (this.idx > this.tokenList.length - 1) {
      const cur = this.lexer.lexNext();
      this.tokenList.push(cur);
    }

    return last;
  };

  peek = () => {
    return this.tokenList[this.idx];
  };

  nextIf = (t: TokenType) => {
    if (this.peek().type === t) {
      this.next();
      return true;
    }
    return false;
  };

  backtrackPoint = () => {
    const idxNow = this.idx;
    return () => (this.idx = idxNow);
  };
}
