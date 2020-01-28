use crate::*;

#[derive(Debug)]
pub enum Expr {
    Constant(Token),
    Grouping(Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),

    Error
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Constant(tok) => write!(f, "`{}`", tok.val),
            Expr::Grouping(expr) => write!(f, "{}", **expr),
            Expr::Binary(left, op, right) => 
                write!(f, "({} {} {})", op.val, **left, **right),
            Expr::Unary(tok, right) => write!(f, "{}{}", tok.val, **right),
            _ => panic!("display trait not defined")
        }
    }
}

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    // FnDecl(Token, )
}

impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Expr(expr) => write!(f, "{};", expr),
            _ => panic!("display trait not defined")
        }
    }
}

