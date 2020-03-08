use crate::*;

#[derive(Debug, Clone)]
pub enum ResolveType {
    Local {id: u16}, //id = slot in frame
    UpValue {id: u16} //id = slot in closure's upvalue array
}
impl std::fmt::Display for ResolveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResolveType::Local {id} => write!(f, "#{}", id),
            ResolveType::UpValue {id} => write!(f, "^{}", id)
        }
    }
}

pub struct Resolver {
    pub list: Vec<ResolverNode>,
    cur: usize
}

impl Resolver {
    pub fn new() -> Resolver {
        let mut res = Resolver {
            list: vec![],
            cur: 0
        };
        let pos = 0;
        res.list.push(ResolverNode {
            env: Env::new(),
            pos
        });
        res
    }

    fn parent(&mut self) -> bool {
        if self.cur == 0 {return false}
        self.cur -= 1;
        true
    }

    fn child(&mut self) -> bool {
        self.cur += 1;
        self.cur < self.list.len()
    }

    fn new_child(&mut self) {
        self.cur += 1;
        let pos = self.list.len() - 1;
        self.list.push(ResolverNode {
            env: Env::new(),
            pos
        });
    }

    fn pop(&mut self) {
        self.cur -= 1;
        self.list.pop();
    }

    fn frame(&mut self) -> &mut ResolverNode {
        self.list.get_mut(self.cur).expect("This is a problem with the resolver itself")
    }
} 

pub struct ResolverNode {
    env: Env,
    pos: usize
}

impl Resolver {
    fn local(&mut self, name: &Token) -> Result<u16, &'static str> {
        let mut id = -1;
        for i in (0..self.frame().env.locals.len()).rev() {
            let local = self.frame().env.get(i);
            if local.name.val == name.val{
                id = i as i32;
                break;
            }
        }
        if id < 0 {
            return Err("Usage of an undefined variable");
        }
        else {
            let id = id as u16;
            return Ok(id)
        }
    }

    fn upvalue(&mut self, name: &Token) -> Result<u16, &'static str> {
        if self.parent() {
            if let Ok(id) = self.local(name) {
                self.frame().env.capture(id);
                self.child();
                let upv_id = self.frame().env.add_upvalue(UpValueIndex {
                    is_local: true,
                    id
                });
                return Ok(upv_id)
            }
            else {
                let id = self.upvalue(name)?;
                self.child();
                let upv_id = self.frame().env.add_upvalue(UpValueIndex {
                    is_local: false,
                    id
                });
                return Ok(upv_id)
            }
        }
        Err("Usage of an undefined variable")
    }

    fn access(&mut self, name: &Token) -> Result<ResolveType, &'static str> {
        if let Ok(id) = self.local(name) {
            return Ok(ResolveType::Local {id});
        }
        let id = self.upvalue(name)?;
        return Ok(ResolveType::UpValue {id});
    }
}

impl Resolver {
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
                if self.frame().env.is_redefined(&name) {
                    return Err("Tried to redeclare a variable in the same scope");
                }
                let id = self.frame().env.add_local(name.clone());

                Ok(Stmt::ResolvedMutDecl(id, Box::new(val)))
            },
            Stmt::Expr(expr) => {
                Ok(Stmt::Expr(Box::new(self.resolve_expr(*expr)?)))
            },
            Stmt::FnDecl(name, params, body) => {
                if self.frame().env.is_redefined(&name) {
                    return Err("Tried to redeclare a variable in the same scope");
                }
                let id = self.frame().env.add_local(name.clone());

                self.new_child();

                self.frame().env.add_local(name.clone());

                for param in params.clone() {
                    self.frame().env.add_local(param);
                }
                
                let body = self.resolve_expr(*body)?;
                let upvalues = self.frame().env.upvalues.clone();
                let captured = self.frame().env.captured.clone();

                self.pop();
                
                Ok(Stmt::ResolvedFnDecl {
                    name,
                    id,
                    params,
                    upvalues,
                    captured,
                    body: Box::new(body)
                })
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
                self.frame().env.open();
                let mut stmts = self.resolve_stmts(stmts)?;
                let pop_count = self.frame().env.close();
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
