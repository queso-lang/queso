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

mod compiler;
use compiler::*;

mod vm;
use vm::*;

mod instruction;
use instruction::*;

mod value;
use value::*;

extern crate clap; 
use clap::{App, Arg, crate_version};

#[derive(Clone)]
struct DebugOpts {
    pub tokens: bool, pub ast: bool, pub instrs: bool
}

#[derive(Clone)]
struct QuesoOpts {
    pub debug: DebugOpts
}


fn main() {
    let matches = App::new("queso")
       .version(crate_version!())
       .about("The interpreter for the queso language")
       .arg(
           Arg::with_name("file")
           .help("The file to be run")
           .index(1)
        )
        .arg(
            Arg::with_name("debug tokens")
            .long("#tokens")
            .help("turns on debug token logging")
            .hidden(true)
        )
        .arg(
            Arg::with_name("debug ast")
            .long("#ast")
            .help("turns on debug AST visualisation")
            .hidden(true)
        )
        .arg(
            Arg::with_name("debug instrs")
            .long("#instrs")
            .help("turns on bytecode instructions logging")
            .hidden(true)
        )
       .get_matches();


    let debug_opts = DebugOpts {
        tokens: matches.occurrences_of("debug tokens") > 0,
        ast: matches.occurrences_of("debug ast") > 0,
        instrs: matches.occurrences_of("debug instrs") > 0
    };

    let opts = QuesoOpts {
        debug: debug_opts
    };

    if let Some(file) = matches.value_of("file") {
        unimplemented!()
    }
    else {
        repl(opts)
    }
}

fn repl(opts: QuesoOpts) {
    loop {
        print!(">");
        io::Write::flush(&mut io::stdout()).expect("flush failed!");
        let mut buf = String::new();
        if let Ok(_) = io::stdin().read_line(&mut buf) {
            run(opts.clone(), buf);
        }
        println!();
    }
}

fn run(opts: QuesoOpts, src: String) -> bool {
    let mut lexer = Lexer::new(src);

    let mut toks = TokenStream::new(lexer);
    if opts.debug.tokens {
        let mut toks = toks.clone();
        println!("\nTOKENS:");
        while toks.peek().t != TokenType::EOF {
            println!("{}", toks.next());
        }
    }

    let mut parser = Parser::new(toks);
    let stmt = parser.program();

    if !parser.had_error {

        if opts.debug.ast {
            let mut stmt = stmt.clone();
            println!("\nAST:");
            if let Stmt::Program(stmts) = stmt {
                stmts.iter().for_each(|stmt| {
                    println!("{}", stmt);
                });
            }
        }

        let mut chk = Chunk::new();
        let compiler = Compiler {};
        compiler.compile(&mut chk, stmt);

        let mut vm = VM::new(opts.debug.instrs);
        let res = vm.execute(chk);
        match res {
            Err(err) => println!("{}", err),
            _ => {}
        }

        return true;
    }

    false
}