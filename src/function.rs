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
    pub upvalues: Vec<UpValue>
}

impl Closure {
    pub fn from_function(func: Rc<Function>, upvalues: Vec<UpValue>) -> Closure {
        Closure {
            func,
            upvalues
        }
    }
}