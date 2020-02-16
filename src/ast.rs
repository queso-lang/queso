use crate::*;

#[derive(Debug, Clone)]
pub enum Expr {
    Constant(Token),
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),

    TrueLiteral(Token), FalseLiteral(Token), NullLiteral(Token),

    Block(Vec<Stmt>),

    Access(Token),
    Assign(Token, Box<Expr>),

    Error
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Constant(tok) => write!(f, "{}", tok.val),
            Expr::Binary(left, op, right) => 
                write!(f, "({} {} {})", op.val, **left, **right),
            Expr::Unary(tok, right) => write!(f, "{}{}", tok.val, **right),
            Expr::NullLiteral(tok) => write!(f, "null"),
            Expr::Block(stmts) => write!(f, "{{ {:#?} }}", stmts),
            _ => panic!("display trait not defined")
        }
    }
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Program(Vec<Stmt>),
    Expr(Expr),
    MutDecl(Token, Expr)
}

impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Expr(expr) => write!(f, "{};", expr),
            Stmt::MutDecl(name, val) => {
                write!(f, "mut {} = {};", name.val, val)
            },
            _ => panic!("display trait not defined")
        }
    }
}
