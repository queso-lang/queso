use crate::*;

#[derive(Debug, Clone)]
pub enum FunctionType {
    Program,
    Function
}

#[derive(Debug, Clone)]
pub struct CallFrame {
    pub clsr: Closure,
    pub funct: FunctionType,
    pub cur_instr: usize,
    pub stack_base: usize
}

impl CallFrame {
    pub fn new(chk: Chunk, stack_base: usize) -> CallFrame {
        let func = Box::new(Function {
            chk,
            name: "".to_string(),
            captured: vec![]
        });
        let func_ptr = Box::into_raw(func);
        CallFrame {
            clsr: Closure::from_function(func_ptr, vec![]),
            funct: FunctionType::Program,
            cur_instr: 0,
            stack_base
        }
    }
    pub fn from_closure(clsr: Closure, stack_base: usize) -> CallFrame {
        CallFrame {
            clsr,
            funct: FunctionType::Function,
            cur_instr: 0,
            stack_base
        }
    }
}
