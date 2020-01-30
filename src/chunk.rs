#[derive(Debug)]
pub enum Op {
    Constant {id: u16},
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub enum Value {
    Bool(bool),
    Number(f64),
    Null
}

struct LineRL {pub line: u32, pub repeat: u16}
type LineVec = Vec<LineRL>;

pub struct Chunk {
    ops: Vec<Op>,
    consts: Vec<Value>,
    lines: LineVec
}