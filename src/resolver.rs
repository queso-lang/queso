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
                let id = self.env.locals.len() as u16 - 1;
                Ok(Stmt::ResolvedMutDecl(id, Box::new(val)))
            },
            Stmt::Expr(expr) => {
                Ok(Stmt::Expr(Box::new(self.resolve_expr(*expr)?)))
            },
            Stmt::FnDecl(name, params, body) => {
                if self.env.is_redefined(&name) {
                    return Err("Tried to redeclare a variable in the same scope");
                }
                self.env.add(name.clone());

                self.env.open();

                for param in params.clone() {
                    self.env.add(param);
                }
                
                let body = self.resolve_expr(*body)?;

                self.env.close();
                let id = self.env.locals.len() as u16 - 1;
                Ok(Stmt::ResolvedFnDecl(name, id, params, Box::new(body)))
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
            Expr::IfElse(cond, ib, eb) => {
                let cond = self.resolve_expr(*cond)?;
                let ib = self.resolve_expr(*ib)?;
                let mut eb = eb;
                if let Some(eb_expr) = eb {
                    eb = Some(Box::new(self.resolve_expr(*eb_expr)?));
                }
                Ok(Expr::IfElse(Box::new(cond), Box::new(ib), eb))
            },
            Expr::FnCall(name, args, pop_count) => {
                let name = self.resolve_expr(*name)?;
                let mut rargs = Vec::<Expr>::new();
                for arg in args {
                    rargs.push(self.resolve_expr(arg)?)
                }

                Ok(Expr::FnCall(Box::new(name), rargs, pop_count))
            }
            _ => Ok(expr)
        }
    }
}