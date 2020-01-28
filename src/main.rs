#![allow(dead_code)]
#![allow(unused)]

mod token;
use token::*;

mod lexer;
use lexer::*;

mod ast;
use ast::*;
fn main() {
    
    let mut lexer = Lexer::new("1 ** (2+4) ** 3".to_string());
    let mut toks = Vec::<Token>::new();
    loop {
        let tok = lexer.lex_next();
        if tok.t == TokenType::EOF {break;};
        toks.push(tok);
    }
}