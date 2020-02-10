use crate::*;

pub fn error(at: Token, msg: &'static str) {
    if at.pos.from_col == at.pos.to_col {
        println!("[{}:{}] {}", at.pos.line, at.pos.from_col, msg);
    }
    else {
        println!("[{}:{}-{}] {}", at.pos.line, at.pos.from_col, at.pos.to_col, msg);
    }
}

pub fn runtime_error() {}