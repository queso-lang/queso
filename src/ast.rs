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

    ResolvedAccess(Token, ResolveType),
    ResolvedFieldAccess {
        list: Vec<Token>,
        id: ResolveType
    },
    ResolvedAssign(Token, ResolveType, Box<Expr>),
    ResolvedFieldAssign {
        list: Vec<Token>,
        id: ResolveType,
        set_to: Box<Expr>
    },
    ResolvedBlock(Vec<Stmt>),

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
            Expr::ResolvedBlock(stmts) => {
                writeln!(f, "{{");
                for stmt in stmts {
                    std::fmt::Display::fmt(&stmt, f);
                    writeln!(f, ";");
                }
                writeln!(f, "}}")
            },
            Expr::ResolvedAccess(tok, id) => write!(f, "{}", id),
            Expr::ResolvedAssign(tok, id, val) => write!(f, "{} = {}", id, val),
            Expr::ResolvedFieldAssign {
                list,
                id,
                set_to
            } => {
                let list_str = list.iter().for_each(|el| {std::fmt::Display::fmt(el, f);});
                write!(f, "#{} = {}", id, set_to)
            },
            Expr::FnCall(_, _, _) => write!(f, "call"),
            _ => panic!("display trait not defined")
        }
    }
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Box<Expr>),
    MutDecl(Token, Box<Expr>),
    ResolvedMutDecl(StackIdx, Box<Expr>),
    FnDecl(Token, Vec<Token>, Box<Expr>),
    ResolvedFnDecl {
        name: Token,
        id: StackIdx,
        upvalues: Vec<UpValueIndex>,
        captured: Vec<StackIdx>,
        params: Vec<Token>,
        body: Box<Expr>
    },
    ClassDecl {
        name: Token,
        decls: Vec<Stmt>
    },
    ResolvedClassDecl {
        id: StackIdx,
        name: Token,
        decls: Vec<Stmt>
    },

    Error
}

impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Expr(expr) => writeln!(f, "{}", expr),
            Stmt::ResolvedMutDecl(id, val) => {
                write!(f, "#{} = {}", id, val)
            },
            Stmt::ResolvedFnDecl {
                name,
                id,
                upvalues,
                captured,
                params,
                body
            } => {
                write!(f, "fn #{}", id);
                for param in params {
                    write!(f, "({})", param.val);
                }
                write!(f, ": ");
                std::fmt::Display::fmt(&**body, f);
                Ok(())
            },
            Stmt::ResolvedClassDecl {
                name,
                id,
                decls
            } => {
                write!(f, "class #{}", id);
                Ok(())
            },
            _ => panic!("display trait not defined")
        }
    }
}
