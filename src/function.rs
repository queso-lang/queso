use crate::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Function {
    pub chk: Chunk,
    pub name: String,
    pub captured: Vec<u16>
}

#[derive(Clone, PartialEq, Debug)]
pub struct Closure {
    pub func: u16,
    pub upvalues: Vec<u16>
}

impl Closure {
    pub fn from_function(func: u16, upvalues: Vec<u16>) -> Closure {
        Closure {
            func,
            upvalues
        }
    }
}