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

    GetLocal(u16),
    GetCaptured(u16),
    SetLocal(u16),
    SetCaptured(u16),
    Declare(u16),
    
    FnCall(u16),
    Closure(u16, u16, Vec<UpValue>), //assignid, constid

    JumpIfFalsy(u16),
    PopAndJumpIfFalsy(u16), //always pop, that is
    JumpIfTruthy(u16),
    Jump(u16),
    JumpPlaceholder,
    Pop, Return,

    ReservePlaceholder,
    Reserve(u16)
}