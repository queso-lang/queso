use crate::*;

pub struct CallFrame {
    pub chk: Chunk,
    pub cur_instr: usize,
    pub stack_base: usize
}

impl CallFrame {
    pub fn new(chk: Chunk, stack_base: usize) -> CallFrame {
        CallFrame {
            chk,
            cur_instr: 0,
            stack_base
        }
    }
}