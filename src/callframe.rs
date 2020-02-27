use crate::*;

pub enum FunctionType {
    Program,
    Function
}

pub struct CallFrame {
    pub func: Function,
    pub funct: FunctionType,
    pub cur_instr: usize,
    pub stack_base: usize
}

impl CallFrame {
    pub fn new(chk: Chunk, stack_base: usize) -> CallFrame {
        CallFrame {
            func: Function {
                chk,
                name: "".to_string()
            },
            funct: FunctionType::Program,
            cur_instr: 0,
            stack_base
        }
    }
}

impl From<Function> for CallFrame {
    fn from(func: Function) -> CallFrame {
        CallFrame {
            func: func,
            funct: FunctionType::Function,
            cur_instr: 0,
            stack_base: 0
        }
    }
}