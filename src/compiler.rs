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

impl Expr {
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
