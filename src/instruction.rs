use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    PushConstant (u16),
    PushTrue,
    PushFalse,
    PushNull,

    Negate,
    ToNumber,
    Not,
    Add,
    Subtract,
    Multiply,
    Divide,

    Equal,
    NotEqual,
    GreaterEqual,
    LessEqual,
    Greater,
    Less,

    Trace,

    PushVariable(u16),
    Assign(u16),

    JumpIfFalse(u16),
    PopAndJumpIfFalse(u16),
    Jump(u16),
    JumpPlaceholder,
    Pop, Return
}