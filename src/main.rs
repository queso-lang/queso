#![allow(dead_code)]
#![allow(unused)]

use std::cell::RefCell;
pub type MutRc<T> = Rc<RefCell<T>>;

use std::io::{self, Read};
use std::rc::Rc;

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

mod env;
use env::*;

mod resolver;
use resolver::*;

mod function;
use function::*;

mod callframe;
use callframe::*;

mod upvalue;
use upvalue::*;

mod gc;
use gc::*;

mod heap;
use heap::*;

extern crate clap; 
use clap::{App, Arg, crate_version};

#[derive(Clone)]
struct DebugOpts {
    pub tokens: bool, pub ast: bool, pub instrs: bool, pub gc: bool
}

#[derive(Clone)]
struct QuesoOpts {
    pub debug: DebugOpts
}


fn main() {
    {std::io::stdout();};
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
        .arg(
            Arg::with_name("debug gc")
            .long("#gc")
            .help("turns on garbager collection logging")
            .hidden(true)
        )
       .get_matches();


    let debug_opts = DebugOpts {
        tokens: matches.occurrences_of("debug tokens") > 0,
        ast: matches.occurrences_of("debug ast") > 0,
        instrs: matches.occurrences_of("debug instrs") > 0,
        gc: matches.occurrences_of("debug gc") > 0
    };

    let opts = QuesoOpts {
        debug: debug_opts
    };

    if let Some(file) = matches.value_of("file") {
        use std::fs;

        let contents = fs::read_to_string(file).unwrap_or_else(|_| {
            fs::read_to_string(file.to_string() + &".queso".to_string()).expect("Could not read the file")
        });

        run(opts, contents);
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
    let program = parser.program();

    if !parser.had_error {
        
        let mut resolver = Resolver::new();
        let program = resolver.resolve(program).expect("");
        
        if opts.debug.ast {
            let mut program = program.clone();
            println!("\nAST:");
            program.iter().for_each(|stmt| {
                println!("{}", stmt);
            })
        }

        let mut chk = Chunk::new();
        let mut compiler = Compiler::new(&mut chk);
        compiler.compile(program);

        let mut vm = VM::new(chk, opts.debug.instrs);


        use std::time::Instant;
        let now = Instant::now();
        let res = vm.execute();
        let new_now = Instant::now();
        println!("{:?}", new_now.duration_since(now));
        
        
        match res {
            Err(err) => println!("{}", err),
            _ => {}
        }

        return true;
    }

    false
}