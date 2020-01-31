use crate::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Bool(bool),
    Number(f64),
    Null
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Number(n) => *n!=0.,
            Value::Null => false
        }
    }
}

impl From<&Token> for Value {
    fn from(tok: &Token) -> Value {
        if tok.t == TokenType::True {
            return Value::Bool(true);
        }
        else if tok.t == TokenType::False {
            return Value::Bool(false);
        }
        else if tok.t == TokenType::Null {
            return Value::Null;
        }
        else if tok.t == TokenType::Number {
            return Value::Number(
                tok.val.parse::<f64>()
                    .expect("Error parsing float! This might be a problem with the interpreter itself.")
            );
        }
        unimplemented!()
    }
}