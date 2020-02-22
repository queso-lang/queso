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
    pub fn compile(&mut self, program: Program) {
        for stmt in program {
            self.compile_stmt(stmt);
        };
        self.chk.add_instr(Instruction::Return, 0);
    }
    fn compile_expr(&mut self, expr: Expr) {
        match expr {
            Expr::Constant(tok) => {
                let const_id = self.chk.add_const(Value::from(&tok));
                self.chk.add_instr(Instruction::PushConstant(const_id), tok.pos.line);
            },
            Expr::Binary(left, op, right) => {
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
            Expr::ResolvedBlock(stmts, pop_count) => {
                for stmt in stmts {
                    self.compile_stmt(stmt);
                }
                for _ in 0..pop_count {
                    self.chk.add_instr(Instruction::Pop, 0); //fix this
                }
                self.chk.add_instr(Instruction::PushNull, 0); //fix this
            },
            Expr::ResolvedAccess(name, id) => self.chk.add_instr(Instruction::PushVariable(id as u16), name.pos.line),
            Expr::ResolvedAssign(name, id, val) => {
                self.compile_expr(*val);
                self.chk.add_instr(Instruction::Assign(id as u16), name.pos.line)
            },
            _ => panic!("This is a problem with the compiler itself")
        }

    }
    fn compile_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Expr(expr) => {
                self.compile_expr(*expr);
                self.chk.add_instr(Instruction::Pop, 0);
            },
            Stmt::MutDecl(name, val) => {
                self.compile_expr(*val);
            }
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