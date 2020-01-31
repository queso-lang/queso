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
    pub fn to_number(&self) -> Result<f64, ()> {
        match self {
            Value::Number(num) => return Ok(num.clone()),
            Value::Bool(b) => return Err(()),
            Value::Null => return Err(())
        }
    }
    pub fn is_greater_than(&self, than: &Value) -> bool {
        return match (self, than) {
            (Value::Number(n1), Value::Number(n2)) => n1 > n2,
            _ => false
        }
    }
    pub fn is_equal_to(&self, to: &Value) -> bool {
        return match (self, to) {
            (Value::Bool(b), val) | (val, Value::Bool(b)) => *b == val.is_truthy(),
            (Value::Number(n), val) | (val, Value::Number(n)) => {
                if let Ok(num) = val.to_number() {
                    return *n == num
                }
                return false
            },
            (Value::Null, Value::Null) => true,
            _ => false
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