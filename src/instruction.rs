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
    FnCall(u16),

    EndBlock(u16),
    JumpIfFalsy(u16),
    PopAndJumpIfFalsy(u16), //always pop, that is
    JumpIfTruthy(u16),
    Jump(u16),
    JumpPlaceholder,
    Pop, Return
}