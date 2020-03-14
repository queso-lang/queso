use crate::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Function {
    pub chk: Chunk,
    pub name: String,
    pub captured: Vec<u16>
}

#[derive(Clone, PartialEq, Debug)]
pub struct Closure {
    pub func: *mut Function,
    pub upvalues: Vec<MutRc<UpValue>>
}

impl Closure {
    pub fn from_function(func: *mut Function, upvalues: Vec<MutRc<UpValue>>) -> Closure {
        Closure {
            func,
            upvalues
        }
    }

    pub fn get_function(&self) -> &Function {
        unsafe {&*self.func}
    }
}