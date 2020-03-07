use crate::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Function {
    pub chk: Chunk,
    pub name: String,
    pub captured: Vec<u16>
}

#[derive(Clone, PartialEq, Debug)]
pub struct Closure {
    pub func: Rc<Function>,
    pub captured: Vec<*mut Value>
}

impl Closure {
    pub fn from_function(func: Rc<Function>, captured: Vec<*mut Value>) -> Closure {
        Closure {
            func,
            captured
        }
    }
}