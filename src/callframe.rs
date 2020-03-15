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
    pub fn new(chk: Chunk, heap: &mut Heap, stack_base: usize) -> CallFrame {
        let func = heap.alloc(ObjType::Function(Function {
            chk,
            name: "".to_string(),
            captured: vec![]
        }));

        CallFrame {
            clsr: Closure::from_function(func, vec![]),
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
