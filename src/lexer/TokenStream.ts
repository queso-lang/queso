import { Lexer } from './Lexer';
import { Token, TokenType } from './Token';

export class TokenStream {
  private cur: Token;

  constructor(private lexer: Lexer) {
    const next = lexer.lexNext();
    this.cur = next;
  }

  next = () => {
    const last = this.cur;
    this.cur = this.lexer.lexNext();
    return last;
  }

  peek = () => {
    return this.cur;
  }

  nextIf = (t: TokenType) => {
    if (this.peek().type === t) {
      this.next();
      return true;
    }
    return false;
  }
}
