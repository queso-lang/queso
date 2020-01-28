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

mod error_reporter;
use error_reporter::*;

fn main() {
    let mut lexer = Lexer::new("1 + 2 * 3".to_string());
    let mut toks = Vec::<Token>::new();
    
    let mut toks = TokenStream::new(lexer);

    let mut parser = Parser::new(toks);
    println!("{}", parser.expr());
}