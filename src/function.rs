use crate::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Function {
    pub chk: Chunk,
    pub name: String,
    pub captured: Vec<u16>
}

#[derive(Clone, PartialEq, Debug)]
pub struct Closure {
    pub func: u32,
    pub upvalues: Vec<u32>
}

impl Closure {
    pub fn from_function(func: u32, upvalues: Vec<u32>) -> Closure {
        Closure {
            func,
            upvalues
        }
    }
}