use crate::{
    token::TokenType,
    scanner::ScanError,
    ast::parser::ParseError,
    interpreter::RuntimeError,
};

pub struct ErrorStatus {
    pub had_error: bool,
}

impl ErrorStatus {
    pub fn new() -> Self {
        ErrorStatus { had_error: false }
    }

    pub fn scan_error(&mut self, error: ScanError) {
        self.print(error.line, String::new(), error.message);
    }

    pub fn parse_error(&mut self, error: ParseError) {
        let token = error.token;
        match token.token_type == TokenType::Eof {
            true => {
                self.print(token.line, " at end".to_string(), error.message);
            },
            false => { 
                let location = format!(" at '{}'", token.lexeme);
                self.print(token.line, location, error.message);
            }
        }
    }

    pub fn runtime_error(&mut self, error: RuntimeError) {

    }

    fn print(&mut self, line_num: usize, location: String, message: String) {
        println!(
            "[line {}] Error{}: {}", line_num, location, message
        );
       self.had_error = true; 
    }
}
