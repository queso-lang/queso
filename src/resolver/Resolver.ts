import { match, P } from 'ts-pattern';
import { ErrorReporter } from '../error/ErrorReporter';
import { Token } from '../lexer/Token';
import {
  createASTExpr,
  createASTStmt,
  Expr,
  Program,
  Stmt,
} from '../parser/AST';
import { Env } from './Env';
import { Resolution } from './Resolution';

type ResolverNode = {
  env: Env;
  pos: number;
};

export class Resolver {
  nodes: ResolverNode[] = [{ env: new Env(), pos: 0 }];
  cur = 0;

  constructor(public errorReporter: ErrorReporter) {}

  private parent = () => {
    if (this.cur === 0) return false;
    this.cur -= 1;
    return true;
  };

  private child = () => {
    this.cur += 1;
    return this.cur < this.nodes.length;
  };

  private newChild = () => {
    this.cur += 1;
    const pos = this.nodes.length - 1;
    this.nodes.push({ env: new Env(), pos });
  };

  private pop = () => {
    this.cur -= 1;
    return this.nodes.pop();
  };

  private frame = () => {
    return this.nodes[this.cur];
  };

  private local = (name: Token) => {
    let id = -1;

    for (const i of [...this.frame().env.locals.keys()].reverse()) {
      const local = this.frame().env.getLocalAt(i);

      if (local.name.val === name.val) {
        id = i;
        break;
      }
    }

    return id;
  };

  private upvalue = (name: Token): number => {
    if (this.parent()) {
      const id = this.local(name);

      if (id >= 0) {
        this.frame().env.capture(id);
        this.child();

        return this.frame().env.addUpvalue({
          id,
          isLocal: true,
        });
      } else {
        const id = this.upvalue(name);
        this.child();

        return this.frame().env.addUpvalue({
          id,
          isLocal: false,
        });
      }
    } else {
      this.errorReporter.report('Usage of an undefined variable', name.pos);
      return -1;
    }
  };

  private access = (name: Token): Resolution => {
    const localId = this.local(name);
    if (localId >= 0) {
      return { local: localId };
    }
    return { upvalue: this.upvalue(name) };
  };

  resolve = (program: Program) => {
    return this.resolveStmts(program);
  };

  private resolveStmts = (stmts: Stmt[]) => {
    const resolved: Stmt[] = [];
    for (const stmt of stmts) {
      resolved.push(this.resolveStmt(stmt));
    }

    return resolved;
  };

  private resolveStmt = (stmt: Stmt): Stmt => {
    return match(stmt)
      .with(['Expr', P._], ([, [expr]]) =>
        createASTStmt('Expr', [this.resolveExpr(expr)]),
      )
      .with(['MutDecl', P._], ([, [name, val]]) => {
        const resolvedVal = this.resolveExpr(val);

        if (this.frame().env.isRedefined(name)) {
          this.errorReporter.report(
            'Tried to redeclare a variable in the same scope',
            name.pos,
          );
        }

        const id = this.frame().env.addLocal(name);

        return createASTStmt('ResolvedMutDecl', {
          token: name,
          expr: resolvedVal,
          id,
        });
      })
      .run();
  };

  private resolveExpr = (expr: Expr): Expr => {
    return match(expr)
      .with(['Binary', P._], ([, [left, op, right]]) =>
        createASTExpr('Binary', [
          this.resolveExpr(left),
          op,
          this.resolveExpr(right),
        ]),
      )
      .with(['Access', P._], ([, [name]]) =>
        createASTExpr('ResolvedAccess', {
          token: name,
          resolution: this.access(name),
        }),
      )
      .with(['Block', P._], ([, [stmts]]) => {
        this.frame().env.openScope();
        const resolvedStmts = this.resolveStmts(stmts);
        this.frame().env.closeScope();
        return createASTExpr('Block', [resolvedStmts]);
      })
      .with(['Unary', P._], ([, [op, right]]) =>
        createASTExpr('Unary', [op, this.resolveExpr(right)]),
      )
      .with(['IfElse', P._], ([, [cond, thenExpr, elseExpr]]) =>
        createASTExpr('IfElse', [
          this.resolveExpr(cond),
          this.resolveExpr(thenExpr),
          this.resolveExpr(elseExpr),
        ]),
      )
      .with(['Fn', P._], ([, [params, body]]) => {
        this.newChild();

        for (const param of params) {
          this.frame().env.addLocal(param);
        }

        const resolvedBody = this.resolveExpr(body);
        const upvalues = this.frame().env.upvalues;
        const captured = this.frame().env.captured;

        this.pop();

        return createASTExpr('ResolvedFn', {
          upvalues,
          captured,
          params,
          body: resolvedBody,
        });
      })
      .otherwise((expr) => expr);
  };
}
