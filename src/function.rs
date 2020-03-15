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
    pub upvalues: Vec<MutRc<ObjUpValue>>
}

impl Closure {
    pub fn from_function(func: u16, upvalues: Vec<MutRc<ObjUpValue>>) -> Closure {
        Closure {
            func,
            upvalues
        }
    }
}