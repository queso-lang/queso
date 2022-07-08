import { Token } from 'src/lexer/Token';

type ADT<name extends string, value extends Record<string, any> | any[] | null = null> = value extends null ? [name] : [
  name,
  value,
];

export type Expr =
  | ADT<'Constant', [Token]>
  | ADT<'Binary', [Expr, Expr]>
  | ADT<'Unary', [Expr]>
  | ADT<'TrueLiteral', [Token]>
  | ADT<'FalseLiteral', [Token]>
  | ADT<'NullLiteral', [Token]>
  | ADT<'Block', [Stmt[]]>
  | ADT<'FnCall', [Expr, Expr[], number]>
  | ADT<'IfElse', [Expr, Expr, Expr | null]>
  | ADT<'Access', [Token]>
  | ADT<'Error'>;


export type Stmt =
  | ADT<'Expr', [Expr]>
  | ADT<'Error'>;