use crate::*;

pub fn error(at: &Token, msg: &'static str) {
    println!("[{}:{}-{}] {}", at.pos.line, at.pos.from_col, at.pos.to_col, msg);
}