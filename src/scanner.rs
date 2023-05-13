use crate::{
    token::{Token, TokenType, Literal},
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
            Literal::NotLit,
            self.line
        );

        self.tokens.push(end_token);
        &self.tokens
    }

    fn scan_token(&mut self) {
        self.advance();
        match self.curr() {
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
            "!" => self.add_token(match self.peek() {
                "=" => TokenType::BangEqual,
                _ => TokenType::Bang,
            }),
            "=" => self.add_token(match self.peek() {
                "=" => TokenType::EqualEqual,
                _ => TokenType::Equal,
            }),           
            ">" => self.add_token(match self.peek() {
                "=" => TokenType::GreaterEqual,
                _ => TokenType::Greater,
            }),
            "<" => self.add_token(match self.peek() {
                "=" => TokenType::LessEqual,
                _ => TokenType::Less,
            }),
            "/" => match self.peek() {
                "/" => self.skip_comment(),
                _ => self.add_token(TokenType::Slash),
            },

            (" " | "\r" | "\t") => {} //Whitespace chars are skipped
            "\n" => self.line += 1,
            "\"" => self.tokenize_string(),

            c if self.is_digit(c) => self.tokenize_number(),
            c if self.is_alpha(c) => self.tokenize_identifier(),
            
            _ => self.lax.error(
                    self.line,
                    "Unexpected character.".to_string()
            ),
        }
    }
    
    fn tokenize_string(&mut self) {
        while (self.peek() != "\"") && (!self.is_at_end()) {
            if self.peek() == "\n" {self.line += 1};
            self.advance();
        }
        if self.is_at_end() {
            self.lax.error(
                self.line,
                "Unterminated string.".to_string()
            );
            return
        }
        self.advance();

        let value = 
            self.source[self.start+1..self.current-1].to_string();

        self.add_literal_token(
            TokenType::String,
            Literal::StringLit(value)
        );
    }

    fn tokenize_number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance()
        };
        if (self.peek() == ".") && (self.is_digit(self.peek_next())) {
            self.advance()
        };
        while self.is_digit(self.peek()) {
            self.advance();
        };
        let num = 
            self.source[self.start..self.current]
            .to_string()
            .parse::<f64>();
        match num {
            Ok(n) => self.add_literal_token(
                TokenType::Number,
                Literal::NumLit(n)
            ),
            Err(_) => self.lax.error(
                self.line,
                "Failed to parse number.".to_string()
            )
        }
    }

    fn tokenize_identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        };
        let text = &self.source[self.start..self.current];
        let token_type = match self.match_keyword(text) {
            Some(t) => t.clone(),
            None => TokenType::Identifier,
        };
        self.add_token(token_type);
    }

    fn match_keyword(&self, keyword: &str) -> Option<TokenType> {
        Some(match keyword {
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "and" => TokenType::And,
            "or" => TokenType::Or,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "while" => TokenType::While,
            "let" => TokenType::Let,
            "fn" => TokenType::Fn,
            "class" => TokenType::Class,
            "return" => TokenType::Return,
            "Nil" =>TokenType::Nil,
            "print" => TokenType::Print,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            _ => return None,   
        })
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.push_token(token_type, Literal::NotLit)
    }

    fn add_literal_token(
        &mut self, token_type: TokenType, literal: Literal) {
       self.push_token(token_type, literal) 
    }

    fn push_token(
        &mut self, token_type: TokenType, literal: Literal) {
        let text = self.source[self.start..self.current].to_string();
        let token = Token::new(
            token_type,
            text,
            literal,
            self.line,
        );
        self.tokens.push(token);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() 
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn curr(&self) -> &str {
        &self.source[self.current-1..=self.current-1]
    }

    fn peek(&self) -> &str {
        if self.is_at_end() {return "\0"}
        &self.source[self.current..=self.current]
    }

    fn peek_next(&self) -> &str {
        if self.current+1 >= self.source.len() {return "\0"};
        &self.source[self.current+1..=self.current+1]
    }
    
    fn skip_comment(&mut self) {
        while (self.peek() != "\n") && (!self.is_at_end()) {
            self.advance();
        }
    }

    fn is_digit(&self, c: &str) -> bool {
        (c >= "0") && (c <= "9")
    }

    fn is_alpha(&self, c: &str) -> bool {
        return  (c >= "a" && c <= "z") ||
                (c >= "A" && c <= "Z") ||
                c == "_"
    }

    fn is_alpha_numeric(&self, c: &str) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }
}
