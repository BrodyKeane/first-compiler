use crate::{
    token::{Token, TokenType, LitType},
    ast::expr::Expr,
};

pub struct Parser {
    tokens: Vec<Token>,
    curr: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser{
            tokens,
            curr: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = self.comparison()?;
        
        while self.match_token(vec!(
            TokenType::BangEqual,
            TokenType::EqualEqual
        )) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.comparison()?;
            expr = Expr::new_binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;
        while self.match_token(vec!(
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        )) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.term()?;
            expr = Expr::new_binary(expr, operator, right);
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;
        while self.match_token(vec!(
            TokenType::Minus,
            TokenType::Plus,
        )) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.factor()?;
            expr = Expr::new_binary(expr, operator, right)
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;
        while self.match_token(vec!(
            TokenType::Star,
            TokenType::Slash,
        )) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.unary()?;
            expr = Expr::new_binary(expr, operator, right);
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(vec!(
            TokenType::Bang,
            TokenType::Minus,
        )) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::new_unary(operator, right))
        }
        return self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        let value = match self.advance().token_type {
            TokenType::False => LitType::Bool(false),
            TokenType::True => LitType::Bool(true),
            TokenType::Nil => LitType::None,
            TokenType::Number 
            | TokenType::String => self.previous().literal.clone(),
            TokenType::OpenParen => {
                let expr = self.expression()?;
                self.consume(TokenType::CloseParen, "Expect ')' after expression.")?;
                return Ok(Expr::new_grouping(expr))
            }
            _ => return Err(
                ParseError::new(self.peek().clone(), "Expected expression.")
            ),
        };
        Ok(Expr::new_literal(value))
    }

    fn consume(&mut self, token_type: TokenType, message: &str
        ) -> Result<&Token, ParseError> {
        match self.check(token_type) {
            true => Ok(self.advance()),
            false => Err(
                ParseError::new(self.peek().clone(), message)
            ),
        }
    }

    fn match_token(&mut self, types: Vec<TokenType>) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {return false};
        self.peek().token_type == token_type
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn advance(&mut self) -> &Token{
        if !(self.is_at_end()) { self.curr += 1 }
        self.previous()
    }
    
    fn peek(&self) -> &Token {
        &self.tokens[self.curr]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.curr - 1]
    }

    //jumps to start of next statement
    fn synchronize(&mut self) {
        self.advance();

        while !(self.is_at_end()) {
            if self.previous().token_type == TokenType::Semicolon {
                return
            }
            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fn
                | TokenType::Let
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => continue,
            }
        };
        self.advance();
    }
}

pub struct ParseError {
    pub token: Token,
    pub message: String,
}

impl ParseError {
    fn new(token: Token, message: &str) -> Self {
       ParseError { token, message: message.to_string() }
    }
}
