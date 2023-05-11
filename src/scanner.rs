use crate::token::{Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: vec!(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !(self.is_at_end()) {
            self.start = self.current;
            self.scan_token();
        }

        let end_token = Token::new(
            TokenType::Eof,
            String::new(),
            None,
            self.line
        );
        self.tokens.push(end_token);
        &self.tokens
    }

    fn scan_token(&self) {
        match self.advance() {
            "(" => add_token(TokenType::OpenParen),
            ")" => add_token(TokenType::CloseParen),
            "{" => add_token(TokenType::OpenBrace),
            "}" => add_token(TokenType::CloseBrace),
            "," => add_token(TokenType::Comma),
            "." => add_token(TokenType::Dot),
            "-" => add_token(TokenType::Minus),
            "+" => add_token(TokenType::Plus),
            ";" => add_token(TokenType::Semicolon),
            "*" => add_token(TokenType::Star),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() 
    }

    fn advance(&mut self) -> &str {
        let next = self.source.get(self.current..=self.current).unwrap();
        self.current += 1;
        next
    }

    fn add_token(&self, token_type: TokenType, literal: Option<String>) {
        let text = self.source[self.start..self.current].to_string();
        let token = Token::new(
            token_type,
            text,
            literal,
            self.line,
        );
        self.tokens.push(token);
    }
}












































