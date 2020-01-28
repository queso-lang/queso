use crate::*;

#[derive(Debug)]
pub enum Expr {
    Number(Token),
    Grouping(Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>)
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Number(tok) => write!(f, "{}", tok.val),
            Expr::Grouping(expr) => write!(f, "{}", **expr),
            Expr::Binary(left, op, right) => 
                write!(f, "({} {} {})", op.val, **left, **right),
            Expr::Unary(tok, right) => write!(f, "{}{}", tok.val, **right)
        }
    }
}

