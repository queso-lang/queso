import { Token } from '../lexer/Token.js';
import { EnvUpvalue } from '../resolver/Env.js';
import { Resolution } from '../resolver/Resolution.js';

export type ADT<
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
  | ADT<'FnCall', [Expr, Expr[]]>
  | ADT<'IfElse', [Expr, Expr, Expr]>
  | ADT<'Access', [Token]>
  | ADT<'Fn', [Token[], Expr]>
  | ADT<'ResolvedAccess', { token: Token; resolution: Resolution }>
  | ADT<
      'ResolvedFn',
      {
        upvalues: EnvUpvalue[];
        captured: number[];
        params: Token[];
        body: Expr;
        localCount: number;
      }
    >
  | ADT<'Error'>;

export type Stmt =
  | Program
  | ADT<'ResolvedProgram', {body: Stmt[], localCount: number;}>
  | ADT<'Expr', [Expr]>
  | ADT<'Error'>
  | ADT<'MutDecl', [Token, Expr]>
  | ADT<'ResolvedMutDecl', { token: Token; id: number; expr: Expr }>;

export type Program = ADT<'Program', {body: Stmt[]}>;

type ASTNode = Expr | Stmt;

type InferADTValueFromName<T> = ASTNode extends infer R
  ? R extends [T, infer S]
    ? S
    : never
  : never;
type Rest<T> = InferADTValueFromName<T> extends never
  ? []
  : [value: InferADTValueFromName<T>];

export const createASTExpr = <T extends Expr[0]>(
  type: T,
  ...value: Rest<T>
): Expr => [type, value[0]] as any;

export const createASTStmt = <T extends Stmt[0]>(
  type: T,
  ...value: Rest<T>
): Stmt => [type, value[0]] as any;