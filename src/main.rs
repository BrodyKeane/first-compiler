#![allow(unused_parens)]

use std::{
    io::{self, Write},
    env,
    process,
    fs,
};

use scanner::Scanner;
use token::{TokenType, Token};
use ast::{
    parser::Parser,
    ast_printer::AstPrinter,
};

mod scanner;
mod token;
mod ast;
mod interpreter;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut lax = Lax::new();
    match args.len() {
        0 => lax.run_prompt(),
        1 => lax.run_file(&args[0]),
        num_args => {
            eprintln!("Expected 1 argument but {} were given", num_args);
            process::exit(64);
        },
    };
    
}

pub struct Lax {
    pub had_error: bool,
}

impl Lax {
    pub fn new() -> Self {
        Lax { had_error: false }
    }

    pub fn run_file(&mut self, path: &String) {
        let source = fs::read_to_string(path).unwrap();
        self.run(source);
        if self.had_error {
            process::exit(65)
        };
    }

    pub fn run_prompt(&mut self) {
        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut line = String::new();
            if let Err(error) = io::stdin().read_line(&mut line) {
                eprintln!("Error reading line: {:?}", error);
            }
            line = line.trim().to_string();
            
            self.run(line);
            self.had_error = false;
        }
    }

    fn run(&mut self, source: String) {
        let mut scanner = Scanner::new(self, source);
        let tokens = scanner.scan_tokens().clone();
        let mut parser = Parser::new(self, tokens);
        let expr = parser.parse();
        if self.had_error {return};
        println!("{}", AstPrinter.print(expr.unwrap()));
    }

    pub fn error(&mut self, line_num: usize, message: &str) {
        self.report(line_num, "", message);
    }

    pub fn parse_error(&mut self, token: Token, message: &str) {
        match token.token_type == TokenType::Eof {
            true => self.report(token.line, " at end", message),
            false => { 
                let location = format!(" at '{}'", token.lexeme);
                self.report(token.line, &location, message);
            }
        }
    }

    fn report(&mut self, line_num: usize, location: &str, message: &str) {
        println!(
            "[line {}] Error{}: {}", line_num, location, message
        );
       self.had_error = true; 
    }

}
