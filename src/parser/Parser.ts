import { TokenType, Token } from '../lexer/Token';
import { TokenStream } from '../lexer/TokenStream';
import { noop } from '../utils';
import { createASTExpr, createASTStmt, Expr, Program, Stmt } from './AST';

enum BP {
  Zero = 0,
  KeywordExpr,
  Assignment,
  Or,
  And,
  Equality,
  Comparison,
  Addition,
  Multiplication,
  Exponentiation,
  Unary,
  FnCall,
  Atom,
}

type PrefixFn = () => Expr;
type InfixFn = (expr: Expr) => Expr;

type ParserRule = {
  prefix?: PrefixFn;
  infix?: InfixFn;
  bp: BP;
  assoc?: 'left' | 'right' | null;
};

export class Parser {
  hadError = false;
  private panic = false;

  constructor(public tokenStream: TokenStream) {}

  parse = () => this.program();

  private consume = (type: TokenType, msg: string) => {
    const cur = this.tokenStream.peek();
    if (cur.type === type) {
      this.tokenStream.next();
      return true;
    }
    this.error(cur, msg);
    return false;
  };

  private error = (token: Token, msg: string) => {
    if (this.panic) return;
    this.hadError = true;
    this.panic = true;
    console.log(
      `[${token.pos.from[0]}:${token.pos.from[1]}-${token.pos.to[0]}:${token.pos.to[1]}] ${msg}`,
    );
  };

  private sync = () => {
    if (this.panic) {
      this.panic = false;
      while (this.tokenStream.peek().type !== 'EOF') {
        if (this.tokenStream.next().type === 'Semi') return;
      }
    }
  };

  private parseBp = (bp: BP): Expr => {
    const cur = this.tokenStream.peek();
    const prefixFn = this.getRule(cur.type).prefix;

    if (prefixFn) {
      let expr = prefixFn();

      while (true) {
        const cur = this.tokenStream.peek();
        if (bp > this.getRule(cur.type).bp) break;

        const infixFn = this.getRule(cur.type).infix;
        expr = infixFn!(expr);
      }

      return expr;
    }

    this.error(cur, 'Expected an expression');
    return createASTExpr('Error');
  };

  private expr = (): Expr => {
    return this.parseBp(BP.Assignment);
  };

  private unary = (): Expr => {
    const op = this.tokenStream.next();
    const expr = this.parseBp(BP.Unary);
    return createASTExpr('Unary', [op, expr]);
  };

  private unaryKeyword = (): Expr => {
    const op = this.tokenStream.next();
    const expr = this.parseBp(BP.KeywordExpr);
    return createASTExpr('Unary', [op, expr]);
  };

  private binary = (left: Expr): Expr => {
    const op = this.tokenStream.next();

    const right = this.parseBp(this.getRule(op.type).bp + 1);

    return createASTExpr('Binary', [left, op, right]);
  };

  private literal = (): Expr => {
    const tok = this.tokenStream.next();

    const expr = noop<{ [key in TokenType]?: Expr }>({
      Number: createASTExpr('Constant', [tok]),
    })[tok.type];

    if (!expr) {
      throw new Error('This is an error with the parser itself!');
    }

    return expr;
  };

  private grouping = (): Expr => {
    this.tokenStream.next();

    const expr = this.expr();

    this.consume('RightParen', 'Unmatched (');

    return expr;
  };

  private access = (): Expr => {
    const cur = this.tokenStream.next();

    return createASTExpr('Access', [cur]);
  };

  private program = (): Program => {
    const stmts: Program = [];
    let isFirst = true;
    while (this.tokenStream.peek().type !== 'EOF') {
      stmts.push(this.stmt(isFirst));
      isFirst = false;
    }

    return stmts;
  };

  private stmt = (isFirst: boolean): Stmt => {
    if (!isFirst) {
      if (this.consume('Semi', 'Expected a ; after expression')) {
        this.panic = false;
      } else {
        this.sync();
      }
    }

    return this.exprStmt();
  };

  private exprStmt = (): Stmt => {
    return createASTStmt('Expr', [this.expr()]);
  };

  private rules: { [key in TokenType]?: ParserRule } = {
    LeftParen: { prefix: this.grouping, infix: undefined, bp: BP.FnCall },
    Plus: { prefix: this.unary, infix: this.binary, bp: BP.Addition },
    Minus: { prefix: this.unary, infix: this.binary, bp: BP.Addition },
    Slash: { prefix: undefined, infix: this.binary, bp: BP.Multiplication },
    Star: { prefix: undefined, infix: this.binary, bp: BP.Multiplication },
    Number: { prefix: this.literal, infix: undefined, bp: BP.Zero },
    EOF: { prefix: undefined, infix: undefined, bp: BP.Zero },
  };

  private getRule = (type: TokenType) => {
    const rule = this.rules[type];

    if (!rule) {
      console.log(type);
      throw new Error('This is a problem with the parser itself');
    }

    return rule;
  };
}
