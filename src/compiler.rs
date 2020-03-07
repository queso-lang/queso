use crate::*;

pub struct Compiler<'a> {
    chk: &'a mut Chunk
}

impl<'a> Compiler<'a> {
    pub fn new(chk: &'a mut Chunk) -> Compiler<'a> {
        Compiler {
            chk
        }
    }
    fn make_jump(&mut self) -> usize {
        self.chk.add_instr(Instruction::JumpPlaceholder, 1);
        self.chk.instrs.len() - 1
    }
    fn label_jump_if_truthy(&mut self, jump_id: usize) {
        self.chk.set_instr(jump_id, Instruction::JumpIfTruthy((self.chk.instrs.len() - 1 - jump_id) as u16));
    }
    fn label_pop_and_jump_if_falsy(&mut self, jump_id: usize) {
        self.chk.set_instr(jump_id, Instruction::PopAndJumpIfFalsy((self.chk.instrs.len() - 1 - jump_id) as u16));
    }
    fn label_jump_if_falsy(&mut self, jump_id: usize) {
        self.chk.set_instr(jump_id, Instruction::JumpIfFalsy((self.chk.instrs.len() - 1 - jump_id) as u16));
    }
    fn label_jump(&mut self, jump_id: usize) {
        self.chk.set_instr(jump_id, Instruction::Jump((self.chk.instrs.len() - 1 - jump_id) as u16));
    }
    fn make_reserve(&mut self) -> usize {
        self.chk.add_instr(Instruction::ReservePlaceholder, 1);
        self.chk.instrs.len() - 1
    }
    fn patch_reserve(&mut self, instr_id: usize) {
        if self.chk.var_count > 0 {
            self.chk.set_instr(instr_id, Instruction::Reserve(self.chk.var_count));
        }
        else {
            self.chk.instrs.remove(instr_id);
        }
    } 
    pub fn compile(&mut self, program: Program) {
        let reserve_id = self.make_reserve();
        for stmt in program {
            self.compile_stmt(stmt, false);
        };
        self.patch_reserve(reserve_id);
        self.chk.add_instr(Instruction::Return, 0);
    }
    pub fn compile_func(&mut self, expr: Expr) {
        let reserve_id = self.make_reserve();
        self.compile_expr(expr);
        self.patch_reserve(reserve_id);
        self.chk.add_instr(Instruction::Return, 0);
    }
    fn compile_expr(&mut self, expr: Expr) {
        match expr {
            Expr::Constant(tok) => {
                let const_id = self.chk.add_const(Value::from(&tok));
                self.chk.add_instr(Instruction::PushConstant(const_id), tok.pos.line);
            },
            Expr::Binary(left, op, right) => {
                if op.t == TokenType::And {
                    self.compile_expr(*left);
                    let jump = self.make_jump();
                    self.chk.add_instr(Instruction::Pop, 0);
                    self.compile_expr(*right);
                    self.label_jump_if_falsy(jump);
                    return;
                }
                if op.t == TokenType::Or {
                    self.compile_expr(*left);
                    let jump = self.make_jump();
                    self.chk.add_instr(Instruction::Pop, 0);
                    self.compile_expr(*right);
                    self.label_jump_if_truthy(jump);
                    return;
                }

                self.compile_expr(*left);
                self.compile_expr(*right);
                match op.t {
                    TokenType::Plus  => self.chk.add_instr(Instruction::Add, op.pos.line),
                    TokenType::Minus => self.chk.add_instr(Instruction::Subtract, op.pos.line),
                    TokenType::Star  => self.chk.add_instr(Instruction::Multiply, op.pos.line),
                    TokenType::Slash => self.chk.add_instr(Instruction::Divide, op.pos.line),
                    TokenType::Slash => self.chk.add_instr(Instruction::Divide, op.pos.line),

                    TokenType::EqualEqual   => self.chk.add_instr(Instruction::Equal, op.pos.line),
                    TokenType::BangEqual    => self.chk.add_instr(Instruction::NotEqual, op.pos.line),
                    TokenType::GreaterEqual => self.chk.add_instr(Instruction::GreaterEqual, op.pos.line),
                    TokenType::LessEqual    => self.chk.add_instr(Instruction::LessEqual, op.pos.line),
                    TokenType::Greater      => self.chk.add_instr(Instruction::Greater, op.pos.line),
                    TokenType::Less         => self.chk.add_instr(Instruction::Less, op.pos.line),

                    _ => unimplemented!()
                }
            },
            Expr::Unary(op, right) => {
                self.compile_expr(*right);
                match op.t {
                    TokenType::Minus => self.chk.add_instr(Instruction::Negate, op.pos.line),
                    TokenType::Plus  => self.chk.add_instr(Instruction::ToNumber, op.pos.line),
                    TokenType::Bang  => self.chk.add_instr(Instruction::Not, op.pos.line),
                    TokenType::Trace => self.chk.add_instr(Instruction::Trace, op.pos.line),
                    _ => unimplemented!()
                }
            }
            Expr::TrueLiteral(tok) => self.chk.add_instr(Instruction::PushTrue, tok.pos.line),
            Expr::FalseLiteral(tok) => self.chk.add_instr(Instruction::PushFalse, tok.pos.line),
            Expr::NullLiteral(tok) => self.chk.add_instr(Instruction::PushNull, tok.pos.line),
            Expr::IfElse(cond, ib, eb) => {
                //jump_a -> omit if branch - jump to else or outside
                //jump_b -> omit else branch - jump from if branch to outside

                self.compile_expr(*cond);
                //emit jump
                let jump_a = self.make_jump();

                self.compile_expr(*ib);

                let jump_b = self.make_jump();
                
                self.label_pop_and_jump_if_falsy(jump_a);

                if let Some(eb) = eb {
                    self.compile_expr(*eb);
                }
                else {
                    self.chk.add_instr(Instruction::PushNull, 1);
                }

                self.label_jump(jump_b);

            },
            Expr::FnCall(left, args, pop_count) => {
                self.compile_expr(*left);
                for arg in args {
                    self.compile_expr(arg);
                }
                self.chk.add_instr(Instruction::FnCall(pop_count), 0);
            },

            Expr::ResolvedBlock(stmts, pop_count) => {
                self.compile_stmts_with_return(stmts);
            },
            Expr::ResolvedAccess(name, id) => {
                match id {
                    ResolveType::Local {id} => {
                        self.chk.add_instr(Instruction::GetLocal(id as u16), name.pos.line)
                    },
                    ResolveType::UpValue {id} => {
                        self.chk.add_instr(Instruction::GetCaptured(id as u16), name.pos.line)
                    },
                }
            },
            Expr::ResolvedAssign(name, id, val) => {
                self.compile_expr(*val);

                match id {
                    ResolveType::Local {id} => {
                        self.chk.add_instr(Instruction::SetLocal(id as u16), name.pos.line)
                    },
                    ResolveType::UpValue {id} => {
                        self.chk.add_instr(Instruction::SetCaptured(id as u16), name.pos.line)
                    },
                }
            },
            _ => panic!("This is a problem with the compiler itself")
        }

    }
    fn compile_stmts_with_return(&mut self, stmts: Vec<Stmt>) {
        if let Some((last, rest)) = stmts.split_last() {
            for stmt in rest {
                self.compile_stmt(stmt.to_owned(), false);
            };
            self.compile_stmt(last.to_owned(), true);
        }
        else {
            self.chk.add_instr(Instruction::PushNull, 0);
        }
    }
    fn compile_stmt(&mut self, stmt: Stmt, is_last: bool) {
        match stmt {
            Stmt::Expr(expr) => {
                self.compile_expr(*expr);
                if !is_last {
                    self.chk.add_instr(Instruction::Pop, 0);
                }
            },
            Stmt::ResolvedMutDecl(id, val) => {
                self.chk.var_count += 1;
                self.compile_expr(*val);
                self.chk.add_instr(Instruction::Declare(id), 0)
            },
            Stmt::ResolvedFnDecl {
                name,
                id,
                upvalues,
                captured,
                params,
                body
            } => {
                self.chk.var_count += 1;
                let mut chk = Chunk::new();
                let mut compiler = Compiler::new(&mut chk);

                compiler.compile_func(*body);
                // chk.print_debug(&name.val);

                let func = Function {
                    chk,
                    name: name.val,
                    captured
                };
                let const_id = self.chk.add_const(Value::Function(Rc::new(func)));

                self.chk.add_instr(Instruction::Closure(id, const_id, upvalues), 0)
            }
            _ => panic!("This is a problem with the compiler itself")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant() {
        let mut chk = Chunk::new();
        let token = Token {
            pos: TokenPos {from_col: 0, to_col: 1, line: 1},
            t: TokenType::Number,
            val: "1.23".to_string()
        };
        let expr = Expr::Constant(token);
        let expr_stmt = Stmt::Expr(Box::new(expr));
        let program = vec![expr_stmt];

        let mut compiler = Compiler::new(&mut chk);
        compiler.compile(program);

        assert_eq!(chk.get_const(0).clone(), Value::Number(1.23));
        assert_eq!(chk.get_line_no(0), 1);
    }

    #[test]
    fn test_arithmetic() {
        let mut chk = Chunk::new();
        let left = Token {
            pos: TokenPos {from_col: 0, to_col: 1, line: 1},
            t: TokenType::Number,
            val: "5".to_string()
        };
        let op = Token {
            pos: TokenPos {from_col: 1, to_col: 2, line: 1},
            t: TokenType::Star,
            val: "*".to_string()
        };
        let right = Token {
            pos: TokenPos {from_col: 2, to_col: 3, line: 1},
            t: TokenType::Number,
            val: "2".to_string()
        };
        let left = Expr::Constant(left);
        let right = Expr::Constant(right);
        let expr = Expr::Binary(Box::new(left), op, Box::new(right));
        let expr_stmt = Stmt::Expr(Box::new(expr));
        let program = vec![expr_stmt];

        let mut compiler = Compiler::new(&mut chk);
        compiler.compile(program);

        assert_eq!(chk.get_instr(0).clone(), Instruction::PushConstant(0));
        assert_eq!(chk.get_instr(1).clone(), Instruction::PushConstant(1));
        assert_eq!(chk.get_instr(2).clone(), Instruction::Multiply);
        assert_eq!(chk.get_instr(3).clone(), Instruction::Pop);
        assert_eq!(chk.get_instr(4).clone(), Instruction::Return);
    }
}