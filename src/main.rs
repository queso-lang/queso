#![allow(dead_code)]
#![allow(unused)]

mod token;
use token::*;

mod lexer;
use lexer::*;

mod parser;
use parser::*;

mod ast;
use ast::*;
fn main() {
    let mut lexer = Lexer::new("1 % (2+3) ** 4".to_string());
    let mut toks = Vec::<Token>::new();
    
    let mut toks = TokenStream::new(lexer);
    // let mut parser = Parser::new(toks);
    // println!("{}", parser.expr(BP::Zero as u8).expect(""));
}