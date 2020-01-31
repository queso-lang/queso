use crate::*;

pub trait Compile {
    fn compile(&self, chk: &mut Chunk);
}

impl Compile for Expr {
    fn compile(&self, chk: &mut Chunk) {
        match self {
            Expr::Constant(tok) => {
                let const_id = chk.add_const(Value::from(tok));
                chk.add_instr(Instruction::PushConstant(const_id), tok.pos.line);
            },
            Expr::Binary(left, op, right) => {
                left.compile(chk);
                right.compile(chk);
                match op.t {
                    TokenType::Plus  => chk.add_instr(Instruction::Add, op.pos.line),
                    TokenType::Minus => chk.add_instr(Instruction::Subtract, op.pos.line),
                    TokenType::Star  => chk.add_instr(Instruction::Multiply, op.pos.line),
                    TokenType::Slash => chk.add_instr(Instruction::Divide, op.pos.line),
                    TokenType::Slash => chk.add_instr(Instruction::Divide, op.pos.line),

                    TokenType::EqualEqual   => chk.add_instr(Instruction::Equal, op.pos.line),
                    TokenType::BangEqual    => chk.add_instr(Instruction::NotEqual, op.pos.line),
                    TokenType::GreaterEqual => chk.add_instr(Instruction::GreaterEqual, op.pos.line),
                    TokenType::LessEqual    => chk.add_instr(Instruction::LessEqual, op.pos.line),
                    TokenType::Greater      => chk.add_instr(Instruction::Greater, op.pos.line),
                    TokenType::Less         => chk.add_instr(Instruction::Less, op.pos.line),

                    _ => unimplemented!()
                }
            },
            Expr::Unary(op, right) => {
                right.compile(chk);
                match op.t {
                    TokenType::Minus  => chk.add_instr(Instruction::Negate, op.pos.line),
                    TokenType::Bang => chk.add_instr(Instruction::Not, op.pos.line),
                    _ => unimplemented!()
                }
            }
            Expr::TrueLiteral(tok) => chk.add_instr(Instruction::PushTrue, tok.pos.line),
            Expr::FalseLiteral(tok) => chk.add_instr(Instruction::PushFalse, tok.pos.line),
            Expr::NullLiteral(tok) => chk.add_instr(Instruction::PushNull, tok.pos.line),
            _ => unimplemented!()
        }
    }
}

impl Compile for Stmt {
    fn compile(&self, chk: &mut Chunk) {
        match self {
            Stmt::Expr(expr) => expr.compile(chk),
            _ => unimplemented!()
        }
    }
}

pub struct Compiler;
impl Compiler {
    pub fn compile<T: Compile>(&self, chk: &mut Chunk, ast: T) {
        ast.compile(chk);
        chk.add_instr(Instruction::Return, 0);
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
        let expr_stmt = Stmt::Expr(expr);

        let compiler = Compiler {};
        compiler.compile(&mut chk, expr_stmt);

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
        let expr_stmt = Stmt::Expr(expr);

        let compiler = Compiler {};
        compiler.compile(&mut chk, expr_stmt);

        assert_eq!(chk.get_instr(0).clone(), Instruction::PushConstant(0));
        assert_eq!(chk.get_instr(1).clone(), Instruction::PushConstant(1));
        assert_eq!(chk.get_instr(2).clone(), Instruction::Multiply);
        assert_eq!(chk.get_instr(3).clone(), Instruction::Return);
    }
}