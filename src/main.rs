#![allow(dead_code)]
#![allow(unused)]
use std::io::{self, Read};

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

mod chunk;
use chunk::*;

fn main() {
    repl();
}

fn repl() {
    loop {
        print!(">");
        io::Write::flush(&mut io::stdout()).expect("flush failed!");
        let mut buf = String::new();
        if let Ok(_) = io::stdin().read_line(&mut buf) {
            run(buf);
        }
        println!();
    }
}

fn run(src: String) -> bool {
    let mut lexer = Lexer::new(src);

    let mut toks = TokenStream::new(lexer);

    let mut parser = Parser::new(toks);
    parser.program().iter().for_each(|stmt| {
        println!("{}", stmt);
    });

    !parser.had_error
}