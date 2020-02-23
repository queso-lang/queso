use crate::*;

pub struct Resolver {
    env: Env
}

impl Resolver {
    pub fn new() -> Resolver {
        Resolver {
            env: Env::new()
        }
    }

    fn access(&mut self, name: &Token) -> Result<u32, &'static str> {
        let mut id = -1;
        for i in (0..self.env.locals.len()) {
            let local = self.env.get(i);
            if local.name.val == name.val{
                id = i as i32;
            }
        }
        if id < 0 {
            return Err("Usage of an undefined variable");
        }
        else {
            let mut id: u32 = id as u32;
            return Ok(id)
        }
    }

    pub fn resolve(&mut self, program: Program) -> Result<Program, &'static str> {
        self.resolve_stmts(program)
    }

    fn resolve_stmts(&mut self, stmts: Vec<Stmt>) -> Result<Vec<Stmt>, &'static str> {
        let mut resolved = Vec::<Stmt>::new();
        for stmt in stmts {
            resolved.push(self.resolve_stmt(stmt)?)
        }
        Ok(resolved)
    }

    fn resolve_stmt(&mut self, stmt: Stmt) -> Result<Stmt, &'static str> {
        match stmt {
            Stmt::MutDecl(name, val) => {
                let val = self.resolve_expr(*val)?;
                if self.env.is_redefined(&name) {
                    return Err("Tried to redeclare a variable in the same scope");
                }
                self.env.add(name.clone());
                Ok(Stmt::MutDecl(name, Box::new(val)))
            },
            Stmt::Expr(expr) => {
                Ok(Stmt::Expr(Box::new(self.resolve_expr(*expr)?)))
            },
            _ => panic!()
        }
    }

    fn resolve_expr(&mut self, expr: Expr) -> Result<Expr, &'static str> {
        match expr {
            Expr::Binary(left, op, right) => {
                if op.t == TokenType::Equal {
                    let right = self.resolve_expr(*right)?;
                    if let Expr::Access(l) = *(left.clone()) {
                        let id = self.access(&l)?;
                        return Ok(Expr::ResolvedAssign(l, id, Box::new(right)));
                    }
                    else {
                        let err = "Invalid assignment target. Expected an identifier";
                        error(op.clone(), err);
                        return Err(err);
                    }
                }
                let left = self.resolve_expr(*left)?;
                let right = self.resolve_expr(*right)?;
                Ok(Expr::Binary(Box::new(left), op, Box::new(right)))
            },
            Expr::Access(name) => {
                let id = self.access(&name)?;
                Ok(Expr::ResolvedAccess(name, id))
            },
            Expr::Block(stmts) => {
                self.env.open();
                let mut stmts = self.resolve_stmts(stmts)?;
                let pop_count = self.env.close();
                Ok(Expr::ResolvedBlock(stmts, pop_count))
            },
            Expr::Unary(op, right) => {
                let right = self.resolve_expr(*right)?;
                Ok(Expr::Unary(op, Box::new(right)))
            },
            Expr::IfElse(cond, if_branch, else_branch) => {
                let cond = self.resolve_expr(*cond)?;
                let ifb = self.resolve_expr(*if_branch)?;
                let mut eb = else_branch;
                if let Some(else_branch_expr) = eb {
                    eb = Some(Box::new(self.resolve_expr(*else_branch_expr)?));
                }
                Ok(Expr::IfElse(Box::new(cond), Box::new(ifb), eb))
            },
            _ => Ok(expr)
        }
    }
}