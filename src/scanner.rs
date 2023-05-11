use crate::{
    token::{Token, TokenType},
    Lax,
};

pub struct Scanner<'a> {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    lax: &'a mut Lax,
}

impl<'a> Scanner<'a> {
    pub fn new(lax: &'a mut Lax, source: String,) -> Scanner {
        Scanner {
            source,
            tokens: vec!(),
            start: 0,
            current: 0,
            line: 1,
            lax,
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

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() 
    }

    fn scan_token(&mut self) {
        match self.advance() {
            "(" => self.add_token(TokenType::OpenParen),
            ")" => self.add_token(TokenType::CloseParen),
            "{" => self.add_token(TokenType::OpenBrace),
            "}" => self.add_token(TokenType::CloseBrace),
            "," => self.add_token(TokenType::Comma),
            "." => self.add_token(TokenType::Dot),
            "-" => self.add_token(TokenType::Minus),
            "+" => self.add_token(TokenType::Plus),
            ";" => self.add_token(TokenType::Semicolon),
            "*" => self.add_token(TokenType::Star),
            
            "!" => self.add_token(match self.next_char() {
                "=" => TokenType::BangEqual,
                _ => TokenType::Bang,
            }),
                
            "=" => self.add_token(match self.next_char() {
                "=" => TokenType::EqualEqual,
                _ => TokenType::Equal,
            }),           
            
            ">" => self.add_token(match self.next_char() {
                "=" => TokenType::GreaterEqual,
                _ => TokenType::Greater,
            }),

            "<" => self.add_token(match self.next_char() {
                "=" => TokenType::LessEqual,
                _ => TokenType::Less,
            }),

            _ => self.lax.error(
                    self.line,
                    "Unexpected character.".to_string(),
                )
        }
    }

    fn advance(&mut self) -> &str {
        let next = self.source.get(self.current..=self.current).unwrap();
        self.current += 1;
        next
    }

    fn next_char(&self) -> &str {
        if self.is_at_end() {return ""}
        &self.source[self.current..=self.current]
    }


    fn add_token(&mut self, token_type: TokenType) {
        self.push_token(token_type, None)
    }

    fn add_literal_token(
        &mut self, token_type: TokenType, literal: String) {
       self.push_token(token_type, Some(literal)) 
    }

    fn push_token(
        &mut self, token_type: TokenType, literal: Option<String>) {
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












































