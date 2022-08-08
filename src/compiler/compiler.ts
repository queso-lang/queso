import binaryen from 'binaryen';
import { match, P } from 'ts-pattern';
import { Expr, Stmt } from '../parser/AST.js';

const compileStmts = (m: binaryen.Module, stmts: Stmt[]) => {
  return m.block(
    null,
    stmts.map((stmt, i) =>
      i < stmts.length - 1 ? m.drop(compile(m, stmt)) : compile(m, stmt),
    ),
  );
};

export const compile = (m: binaryen.Module, node: Expr | Stmt): number => {
  return match(node)
    .with(['Expr', P._], ([, [expr]]) => compile(m, expr))
    .with(['ResolvedFn', P._], ([, { params, localCount, body }]) =>
      m.addFunction(
        (Math.random() + 1).toString(36).substring(7),
        binaryen.createType(Array(params.length).fill(binaryen.i32)),
        binaryen.i32,
        Array(localCount).fill(binaryen.i32),
        compile(m, body),
      ),
    )
    .with(['ResolvedAccess', P._], ([, { resolution }]) =>
      m.local.get((resolution as { local: number }).local, binaryen.i32),
    )
    .with(['ResolvedMutDecl', P._], ([, { expr, id, token }]) =>
      m.local.set(id, compile(m, expr)),
    )
    .with(['NullLiteral', P._], () =>
      m.call('~rt/createValue/null', [], binaryen.i32),
    )
    .with(['Constant', P._], ([, [token]]) => {
      if (token.type === 'Number') {
        return m.call(
          '~rt/createValue/number',
          [m.f64.const(+token.val)],
          binaryen.i32,
        );
      } else throw new Error('constant expr not implemented');
    })
    .with(['Block', P._], ([, [stmts]]) => compileStmts(m, stmts))
    .with(['Program', P._], ([, [stmts]]) => compileStmts(m, stmts))
    .otherwise((val) => {
      throw new Error('not implemented');
    });
};
