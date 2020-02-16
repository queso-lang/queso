use crate::*;

pub struct Env {
    locals: Vec<Local>,
    locals_count: u32,
    scope_depth: u32
}

impl Env {
    pub fn new() -> Env {
        Env {
            locals: Vec::<Local>::new(),
            locals_count: 0,
            scope_depth: 0
        }
    }
}

pub struct Local {
    name: Token,
    depth: u8
}