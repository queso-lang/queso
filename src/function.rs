use crate::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Function {
    pub chk: Chunk,
    pub name: String,
    pub captured: Vec<StackIdx>
}

#[derive(Clone, PartialEq, Debug)]
pub struct Closure {
    pub func: HeapIdx,
    pub upvalues: Vec<HeapIdx>
}

impl Closure {
    pub fn from_function(func: HeapIdx, upvalues: Vec<HeapIdx>) -> Closure {
        Closure {
            func,
            upvalues
        }
    }
}