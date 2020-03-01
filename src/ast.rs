use crate::*;

pub type Program = Vec<Stmt>;

#[derive(Debug, Clone)]
pub enum Expr {
    Constant(Token),
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),

    TrueLiteral(Token), FalseLiteral(Token), NullLiteral(Token),

    Block(Vec<Stmt>),

    FnCall(Box<Expr>, Vec<Expr>, u16),

    IfElse(Box<Expr>, Box<Expr>, Option<Box<Expr>>),

    Access(Token),

    ResolvedAccess(Token, u32),
    ResolvedAssign(Token, u32, Box<Expr>),
    ResolvedBlock(Vec<Stmt>, u16),

    Error
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Constant(tok) => write!(f, "{}", tok.val),
            Expr::Binary(left, op, right) => 
                write!(f, "({} {} {})", op.val, **left, **right),
            Expr::Unary(tok, right) => {
                write!(f, "{} ", tok.val);
                std::fmt::Display::fmt(&**right, f)
            },
            Expr::NullLiteral(tok) => write!(f, "null"),
            Expr::TrueLiteral(tok) => write!(f, "true"),
            Expr::FalseLiteral(tok) => write!(f, "false"),
            Expr::IfElse(cond, if_branch, else_branch) => {
                write!(f, "if {} -> {}", **cond, **if_branch);
                if let Some(else_branch) = else_branch {
                    write!(f, " else {}", **else_branch);
                }
                Ok(())
            },
            Expr::ResolvedBlock(stmts, pop_count) => {
                writeln!(f, "{{");
                for stmt in stmts {
                    std::fmt::Display::fmt(&stmt, f);
                    writeln!(f, ";");
                }
                writeln!(f, "}}")
                // writeln!(f, "variables popped: {}", pop_count)
            },
            Expr::ResolvedAccess(tok, id) => write!(f, "#{} ({})", id, tok),
            Expr::ResolvedAssign(tok, id, val) => write!(f, "#{} ({}) = {}", id, tok, val),
            _ => panic!("display trait not defined")
        }
    }
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Box<Expr>),
    MutDecl(Token, Box<Expr>),
    ResolvedMutDecl(u16, Box<Expr>),
    FnDecl(Token, Vec<Token>, Box<Expr>),
    ResolvedFnDecl(Token, u16, Vec<Token>, Box<Expr>),

    Error
}

impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Expr(expr) => write!(f, "{}", expr),
            Stmt::MutDecl(name, val) => {
                write!(f, "mut {} = {}", name.val, val)
            },
            _ => panic!("display trait not defined")
        }
    }
}
