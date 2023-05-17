#![allow(unused_parens)]

use std::{
    io::{self, Write},
    env,
    process,
    fs, 
};

use scanner::Scanner;
use error::ErrorStatus;
use ast::{
    parser::Parser,
    ast_printer::AstPrinter,
};

mod scanner;
mod token;
mod ast;
mod interpreter;
mod error;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.len() {
        0 => run_prompt(),
        1 => run_file(&args[0]),
        num_args => {
            eprintln!("Expected 1 argument but {} were given", num_args);
            process::exit(64);
        },
    };
    
}

pub fn run_file(path: &String) {
    let source = fs::read_to_string(path).unwrap();
    let mut status = ErrorStatus::new();
    run(&mut status, source);
    if status.had_compile_error {process::exit(65)}
    if status.had_runtime_error {process::exit(70)}
}

pub fn run_prompt() {
    let mut status = ErrorStatus::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if let Err(error) = io::stdin().read_line(&mut line) {
            eprintln!("Error reading line: {:?}", error);
        }
        line = line.trim().to_string();
        
        run(&mut status, line);
        status.had_compile_error = false;
    }
}

fn run(status: &mut ErrorStatus, source: String) {
    let mut parser = Parser::new(
        Scanner::new(status, source)
        .scan_tokens()
    );
    if status.had_compile_error{return};
    match parser.parse() {
        Ok(expr) => println!("{}", AstPrinter.print(expr)),
        Err(error) => status.report_compile_error(error),
    }
}


