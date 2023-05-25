use std::{
    error::Error,
    fmt,
    rc::Rc,
};

use crate::token::{Token, TokenType};

pub struct ErrorStatus {
    pub had_compile_error: bool,
    pub had_runtime_error: bool,
}

impl ErrorStatus {
    pub fn new() -> Self {
        ErrorStatus { 
            had_compile_error: false,
            had_runtime_error: false,
        }
    }

    pub fn report_compile_error<E: Error>(&mut self, error: E) {
        eprintln!("{}", error);
        self.had_compile_error = true;
    }

    pub fn report_runtime_error<E: Error>(&mut self, error: E) {
        eprintln!("{}", error);
        self.had_runtime_error = true;
    }
}

#[derive(Debug)]
pub struct RuntimeError {
    token: Rc<Token>,
    message: String,
}

impl RuntimeError {
    pub fn new(token: Rc<Token>, message: &str) -> Self {
        RuntimeError { token, message: message.to_string() }
    }
}

impl Error for RuntimeError {}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[line {}] {}", self.token.line, self.message)
    }
}

#[derive(Debug)]
pub struct ScanError {
    pub line: usize,
    pub message: String,
}

impl ScanError {
    pub fn new(line: usize, message: &str) -> Self {
        ScanError { line, message: message.to_string() }
    }
}

impl Error for ScanError {}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[line {}] Error: {}", self.line, self.message)
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub token: Rc<Token>,
    pub message: String,
}

impl ParseError {
    pub fn new(token: Rc<Token>, message: &str) -> Self {
       ParseError { token, message: message.to_string() }
    }
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.token.token_type == TokenType::Eof {
            true => {
                write!(f, "[line {}] Error at end: {}",
                    self.token.line, self.message)
            },
            false => { 
                write!(f, "[line {}] Error at '{}': {}",
                    self.token.line, self.token.lexeme, self.message)
            }
        }
    }
}
