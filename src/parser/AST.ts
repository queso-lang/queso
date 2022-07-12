import { Token } from "../lexer/Token";

type ADT<
  name extends string,
  value extends Record<string, any> | any[] | null = null,
> = value extends null ? [name] : [name, value];

export type Expr =
  | ADT<'Constant', [Token]>
  | ADT<'Binary', [Expr, Token, Expr]>
  | ADT<'Unary', [Token, Expr]>
  | ADT<'TrueLiteral', [Token]>
  | ADT<'FalseLiteral', [Token]>
  | ADT<'NullLiteral', [Token]>
  | ADT<'Block', [Stmt[]]>
  | ADT<'FnCall', [Expr, Expr[], number]>
  | ADT<'IfElse', [Expr, Expr, Expr | null]>
  | ADT<'Access', [Token]>
  | ADT<'Fn', [Token[], Expr]>
  | ADT<'Error'>;

export type Stmt = ADT<'Expr', [Expr]> | ADT<'Error'> | ADT<'MutDecl', [Token, Expr]>;

export type Program = Stmt[];

type ASTNode = Expr | Stmt;

type InferADTValueFromName<T> = ASTNode extends infer R ? (R extends [T, infer S] ? S : never) : never;
type Rest<T> = InferADTValueFromName<T> extends never ? [] : [
  value: InferADTValueFromName<T>,
];

export const createASTExpr = <T extends Expr[0]>(
  type: T,
  ...value: Rest<T>
): Expr => [type, value[0]] as any;

export const createASTStmt = <T extends Stmt[0]>(
  type: T,
  ...value: Rest<T>
): Stmt => [type, value[0]] as any;
