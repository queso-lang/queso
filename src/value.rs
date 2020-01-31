use crate::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Bool(bool),
    Number(f64),
    String(String),
    Null
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Number(n) => *n!=0.,
            Value::String(s) => s.len() > 0,
            Value::Null => false
        }
    }
    pub fn to_number(&self) -> Result<f64, ()> {
        match self {
            Value::Number(num) => return Ok(num.clone()),
            Value::Bool(b) => return Err(()),
            Value::String(s) => return match s.parse::<f64>() {
                Ok(num) => return Ok(num),
                _ => return Err(())
            },
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
        return self == to;
    }
}

impl From<&Token> for Value {
    fn from(tok: &Token) -> Value {
        return match tok.t {
            TokenType::Number => Value::Number(
                tok.val.parse::<f64>()
                    .expect("Error parsing float! This might be a problem with the interpreter itself.")
            ),
            TokenType::String => {
                let s = tok.val.clone();
                Value::String(s[1..s.len()-1].to_string())
            },
            TokenType::True   => Value::Bool(true),
            TokenType::False  => Value::Bool(false),
            TokenType::Null   => Value::Null,
            _ => unimplemented!()
        }
    }
}