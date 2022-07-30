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
  ConditionalTernary,
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
type PostfixFn = (expr: Expr) => Expr;

type ParserRule = {
  prefix?: {
    fn: PrefixFn;
    bp: BP;
  };
  infix?: {
    fn: InfixFn;
    assoc: 'left' | 'right';
    bp: BP;
  };
  postfix?: {
    fn: PostfixFn;
    bp: BP;
  };
};

class Backtrack extends Error {}

const getBpForInfixRule = (
  infixRule:
    | {
        fn: InfixFn;
        assoc: 'left' | 'right';
        bp: BP;
      }
    | undefined,
) =>
  infixRule
    ? infixRule.assoc === 'left'
      ? [infixRule.bp, infixRule.bp + 0.1]
      : [infixRule.bp + 0.1, infixRule.bp]
    : [0, 0];

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
    if (this.panic || token.type === 'EOF') return;
    // console.log('error on token', token);
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

  private parseBp = (minBp: BP): Expr => {
    const cur = this.tokenStream.peek();
    const prefixRule = this.getRule(cur.type).prefix;

    if (prefixRule) {
      let expr = prefixRule.fn();

      while (true) {
        const cur = this.tokenStream.peek();
        const rule = this.getRule(cur.type);

        const postfixRule = rule.postfix;
        if (postfixRule) {
          const postfixBp = postfixRule.bp;
          if (postfixBp < minBp) break;

          expr = postfixRule.fn(expr);
        } else {
          const infixRule = rule.infix;
          const [lBp, rBp] = getBpForInfixRule(infixRule);
          if (lBp < minBp) break;

          expr = infixRule!.fn(expr);
        }
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
    const expr = this.parseBp(this.getRule(op.type).prefix!.bp);
    return createASTExpr('Unary', [op, expr]);
  };

  private unaryKeyword = (): Expr => {
    const op = this.tokenStream.next();
    const expr = this.parseBp(BP.KeywordExpr);
    return createASTExpr('Unary', [op, expr]);
  };

  private binary = (left: Expr): Expr => {
    const op = this.tokenStream.next();

    const infixRule = this.getRule(op.type).infix;
    const [lBp, rBp] = getBpForInfixRule(infixRule);
    const right = this.parseBp(rBp);

    return createASTExpr('Binary', [left, op, right]);
  };

  private conditionalTernary = (left: Expr): Expr => {
    // this is always of type Question
    const op = this.tokenStream.next();
    // console.log({ op });

    const thenExpr = this.expr();

    let elseExpr = createASTExpr('NullLiteral', [op]);
    if (this.tokenStream.nextIf('Colon')) {
      const infixRule = this.getRule(op.type).infix;
      const [lBp, rBp] = getBpForInfixRule(infixRule);
      elseExpr = this.parseBp(rBp);
    }

    // console.dir({ left, thenExpr, elseExpr }, { depth: null });

    return createASTExpr('IfElse', [left, thenExpr, elseExpr]);
  };

  private fnCall = (left: Expr): Expr => {
    // this is always of type LeftParen
    const op = this.tokenStream.next();

    let args: Expr[] = [];

    if (this.tokenStream.peek().type !== 'RightParen') {
      while (true) {
        args.push(this.expr());

        if (!this.tokenStream.nextIf('Comma')) {
          break;
        }
      }
    }

    this.consume('RightParen', 'Expected ) after argument list');

    return createASTExpr('FnCall', [left, args]);
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
      // console.log(err);
      backtrackPoint();

      // console.log(this.tokenStream.peek());

      const expr = this.expr();

      this.consume('RightParen', 'Unmatched (');

      return expr;
    }
  };

  private identifier = (): Expr => {
    const cur = this.tokenStream.next();

    if (this.tokenStream.peek().type === 'SlimArrow') {
      return this.fnFromIdentifier(cur);
    }

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

  private fnParamList = (endWith: 'RightParen' | 'SlimArrow'): Token[] => {
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

    // TODO: limit this reasonably to only several tokens
    while (true) {
      // look as far as a semi, eof, or leftparen
      // the LeftParen comes from the fact, that we
      // might have an expression like (a + (b) -> c)
      // where the existence of the nested lambda
      // confuses the parser, which now tries to parse the grouping as a lambda
      if (['Semi', 'EOF', 'LeftParen'].includes(this.tokenStream.peek().type)) {
        throw new Backtrack();
      }
      if (this.tokenStream.peek().type === 'RightParen') {
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

    const params = this.fnParamList('RightParen');

    this.tokenStream.next(); // RightParen
    this.tokenStream.next(); // SlimArrow

    return createASTExpr('Fn', [params, this.expr()]);
  };

  private fnNoParams = (): Expr => {
    this.tokenStream.next();
    return createASTExpr('Fn', [[], this.expr()]);
  };

  private fnFromIdentifier = (name: Token): Expr => {
    this.tokenStream.next();
    return createASTExpr('Fn', [[name], this.expr()]);
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
    LeftParen: {
      prefix: {
        fn: this.grouping,
        bp: BP.Unary,
      },
      postfix: {
        fn: this.fnCall,
        bp: BP.FnCall,
      },
    },
    Plus: {
      prefix: { fn: this.unary, bp: BP.Unary },
      infix: { fn: this.binary, bp: BP.Addition, assoc: 'left' },
    },
    Minus: {
      prefix: { fn: this.unary, bp: BP.Unary },
      infix: { fn: this.binary, bp: BP.Addition, assoc: 'left' },
    },
    Slash: {
      infix: { fn: this.binary, bp: BP.Multiplication, assoc: 'left' },
    },
    Star: {
      infix: { fn: this.binary, bp: BP.Multiplication, assoc: 'left' },
    },
    Bang: {
      prefix: { fn: this.unary, bp: BP.Unary },
    },

    Number: {
      prefix: { fn: this.literal, bp: BP.Atom },
    },
    String: {
      prefix: { fn: this.literal, bp: BP.Atom },
    },
    True: {
      prefix: { fn: this.literal, bp: BP.Atom },
    },
    False: {
      prefix: { fn: this.literal, bp: BP.Atom },
    },
    Null: {
      prefix: { fn: this.literal, bp: BP.Atom },
    },

    EqualEqual: {
      infix: { fn: this.binary, bp: BP.Equality, assoc: 'left' },
    },
    BangEqual: {
      infix: { fn: this.binary, bp: BP.Equality, assoc: 'left' },
    },

    Greater: {
      infix: { fn: this.binary, bp: BP.Comparison, assoc: 'left' },
    },
    GreaterEqual: {
      infix: { fn: this.binary, bp: BP.Comparison, assoc: 'left' },
    },
    Less: {
      infix: { fn: this.binary, bp: BP.Comparison, assoc: 'left' },
    },
    LessEqual: {
      infix: { fn: this.binary, bp: BP.Comparison, assoc: 'left' },
    },

    Identifier: {
      prefix: { fn: this.identifier, bp: BP.Atom },
    },

    And: {
      infix: { fn: this.binary, bp: BP.And, assoc: 'left' },
    },
    Or: {
      infix: { fn: this.binary, bp: BP.Or, assoc: 'left' },
    },

    SlimArrow: {
      prefix: { fn: this.fnNoParams, bp: BP.Unary },
    },

    Question: {
      infix: {
        fn: this.conditionalTernary,
        bp: BP.ConditionalTernary,
        assoc: 'right',
      },
    },
  };

  private getRule = (type: TokenType) => {
    const rule = this.rules[type];

    return rule ?? {};
  };
}
