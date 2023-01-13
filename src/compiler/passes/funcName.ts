import { match } from "ts-pattern";
import { Expr, Program, Stmt } from "../../parser/AST";

// export const funcNamePass = (node: Expr | Stmt) => {
//   return match(node)
//     .with(['ResolvedFn', P._], ([, [expr]]) => this.compile(expr))
//     .otherwise(node)
// }