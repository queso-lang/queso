use crate::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Function {
    pub chk: Chunk,
    pub name: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Closure {
    pub func: Rc<Function>
}

impl Closure {
    pub fn from_function(func: Rc<Function>) -> Closure {
        Closure {
            func
        }
    }
}