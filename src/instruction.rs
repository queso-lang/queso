use crate::*;

pub type ConstIdx = u16;

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

    GetLocal(StackIdx),
    GetLocalField {
        id: StackIdx,
        list: Vec<Box<str>>
    },
    GetUpValue(u16),
    GetUpValueField {
        id: u16,
        list: Vec<Box<str>>
    },
    SetLocal(u16),
    SetLocalField {
        id: StackIdx,
        list: Vec<Box<str>>
    },
    SetUpValue(u16),
    SetUpValueField {
        id: u16,
        list: Vec<Box<str>>
    },
    Declare(u16),

    FnCall(u16),
    DeclareClosure(u16, u16, Vec<UpValueIndex>), //assignid, constid
    DeclareClass {
        assign_id: StackIdx,
        const_id: ConstIdx
    },

    JumpIfFalsy(u16),
    PopAndJumpIfFalsy(u16), //always pop, that is
    JumpIfTruthy(u16),
    Jump(u16),
    JumpPlaceholder,
    Pop, Return,

    ReservePlaceholder,
    Reserve(u16)
}