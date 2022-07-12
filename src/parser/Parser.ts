import { ErrorReporter } from '../error/ErrorReporter';
import { TokenType, Token } from '../lexer/Token';
import { TokenStream } from '../lexer/TokenStream';
import { noop } from '../utils';
import { match } from 'ts-pattern';
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

class Backtrack extends Error {}

export class Parser {
  hadError = false;
  private panic = false;

  constructor(
    public tokenStream: TokenStream,
    public errorReporter: ErrorReporter,
  ) {}

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
    // console.log(
    //   `[${token.pos.from[0]}:${token.pos.from[1]}-${token.pos.to[0]}:${token.pos.to[1]}] ${msg}`,
    // );
    this.errorReporter.report(msg, token.pos);
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

    const expr = match(tok.type)
      .with('Number', () => createASTExpr('Constant', [tok]))
      .with('String', () => createASTExpr('Constant', [tok]))
      .with('True', () => createASTExpr('TrueLiteral', [tok]))
      .with('False', () => createASTExpr('FalseLiteral', [tok]))
      .with('Null', () => createASTExpr('NullLiteral', [tok]))
      .otherwise(() => {
        throw new Error('This is an error with the parser itself!');
      });

    return expr;
  };

  private grouping = (): Expr => {
    this.tokenStream.next();

    const backtrackPoint = this.tokenStream.backtrackPoint();
    try {
      const expr = this.fnFromLeftParen();
      return expr;
    } catch (err) {
      console.log(err);
      backtrackPoint();

      // console.log(this.tokenStream.peek());

      const expr = this.expr();

      this.consume('RightParen', 'Unmatched (');

      return expr;
    }
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

    const tok = this.tokenStream.peek();

    match(tok.type)
      .with('RightParen', () => {
        this.error(
          tok,
          'Unexpected semi. Either remove the semi, or explicitly return null',
        );
      })
      .with('EOF', () => {
        this.error(
          tok,
          'The last statement in your program must not end in a semi',
        );
      });

    return match(tok.type)
      .with('Mut', () => this.mutDecl())
      .otherwise(() => this.exprStmt());
  };

  private exprStmt = (): Stmt => {
    return createASTStmt('Expr', [this.expr()]);
  };

  private fnArgList = (endWith: 'RightParen' | 'SlimArrow'): Token[] => {
    let params: Token[] = [];
    if (this.tokenStream.peek().type !== endWith) {
      while (true) {
        params.push(this.tokenStream.peek());
        this.consume('Identifier', 'Expected parameter name');

        if (!this.tokenStream.nextIf('Comma')) {
          break;
        }
      }
    }
    return params;
  };

  private fnFromLeftParen = (): Expr => {
    const backtrackPoint = this.tokenStream.backtrackPoint();

    // in this loop we detect if this statement has a ") ->" somewhere
    // if so, we backtrack and parse the argument list and the whole lambda
    // otherwise we throw and backtrack to try and parse a grouping instead
    while (true) {
      if (['Semi', 'EOF'].includes(this.tokenStream.peek().type)) {
        throw new Backtrack();
      }
      if (this.tokenStream.peek().type === 'RightParen') {
        console.log('x');
        this.tokenStream.next();
        if (this.tokenStream.peek().type !== 'SlimArrow') {
          throw new Backtrack();
        }
        this.tokenStream.next();
        break;
      }
      this.tokenStream.next();
    }
    backtrackPoint();

    const params = this.fnArgList('RightParen');

    this.tokenStream.next(); // RightParen
    this.tokenStream.next(); // SlimArrow

    return createASTExpr('Fn', [params, this.expr()]);
  };

  private mutDecl = (): Stmt => {
    this.tokenStream.next();

    const name = this.tokenStream.peek();
    this.consume('Identifier', 'Expected a variable identifier');

    let val = createASTExpr('NullLiteral', [name]);
    if (this.tokenStream.nextIf('Equal')) {
      val = this.expr();
    }

    return createASTStmt('MutDecl', [name, val]);
  };

  private rules: { [key in TokenType]?: ParserRule } = {
    LeftParen: { prefix: this.grouping, infix: undefined, bp: BP.FnCall },

    Plus: { prefix: this.unary, infix: this.binary, bp: BP.Addition },
    Minus: { prefix: this.unary, infix: this.binary, bp: BP.Addition },
    Slash: { prefix: undefined, infix: this.binary, bp: BP.Multiplication },
    Star: { prefix: undefined, infix: this.binary, bp: BP.Multiplication },
    Bang: { prefix: this.unary, infix: undefined, bp: BP.Zero },

    Number: { prefix: this.literal, infix: undefined, bp: BP.Zero },
    String: { prefix: this.literal, infix: undefined, bp: BP.Zero },
    True: { prefix: this.literal, infix: undefined, bp: BP.Zero },
    False: { prefix: this.literal, infix: undefined, bp: BP.Zero },
    Null: { prefix: this.literal, infix: undefined, bp: BP.Zero },

    EqualEqual: { prefix: undefined, infix: this.binary, bp: BP.Equality },
    BangEqual: { prefix: undefined, infix: this.binary, bp: BP.Equality },

    Greater: { prefix: undefined, infix: this.binary, bp: BP.Comparison },
    GreaterEqual: { prefix: undefined, infix: this.binary, bp: BP.Comparison },
    Less: { prefix: undefined, infix: this.binary, bp: BP.Comparison },
    LessEqual: { prefix: undefined, infix: this.binary, bp: BP.Comparison },

    Identifier: { prefix: this.access, infix: undefined, bp: BP.Zero },

    And: { prefix: undefined, infix: this.binary, bp: BP.And },
    Or: { prefix: undefined, infix: this.binary, bp: BP.Or },
  };

  private getRule = (type: TokenType) => {
    const rule = this.rules[type];

    if (!rule) {
      return { prefix: undefined, infix: undefined, bp: BP.Zero };
    }

    return rule;
  };
}
