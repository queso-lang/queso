use crate::*;

impl Token {
    pub fn to_value(&self) -> Value {
        if self.t == TokenType::True {
            return Value::Bool(true);
        }
        else if self.t == TokenType::False {
            return Value::Bool(false);
        }
        else if self.t == TokenType::Tilde {
            return Value::Null;
        }
        else if self.t == TokenType::Number {
            return Value::Number(
                self.val.parse::<f64>()
                    .expect("Error parsing float! This might be a problem with the interpreter itself.")
            );
        }
        unimplemented!()
    }
}

trait Compile {
    fn compile(&self, chk: &mut Chunk);
}

impl Compile for Expr {
    fn compile(&self, chk: &mut Chunk) {
        match self {
            Expr::Constant(tok) => {
                let const_id = chk.add_const(tok.to_value());
                chk.add_instr(Instruction::Constant(const_id), tok.pos.line);
            },
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

struct Compiler;
impl Compiler {
    pub fn compile<T: Compile>(&self, chk: &mut Chunk, ast: T) {
        ast.compile(chk);
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
}