use crate::*;

#[derive(Debug, Clone)]
pub enum FunctionType {
    Program,
    Function
}

#[derive(Debug, Clone)]
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
    pub fn from_function(func: Function, stack_base: usize) -> CallFrame {
        CallFrame {
            func,
            funct: FunctionType::Function,
            cur_instr: 0,
            stack_base
        }
    }
}