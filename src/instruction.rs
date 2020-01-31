use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    PushConstant (u16),
    PushTrue,
    PushFalse,
    PushNull,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    Not,

    Return
}