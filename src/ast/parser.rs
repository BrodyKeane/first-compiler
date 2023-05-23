use std::error::Error;
use std::fmt;

use crate::{
    token::{Token, TokenType, LitType},
    error::ErrorStatus,
    ast::{
        expr::Expr,
        stmt::Stmt,
    }, 
};

pub struct Parser<'a> {
    tokens: Vec<Token>,
    curr: usize,
    status: &'a mut ErrorStatus,
}

impl<'a> Parser<'a> {
    pub fn new(status: &'a mut ErrorStatus, tokens: Vec<Token>) -> Self {
        Parser{
            tokens,
            curr: 0,
            status,
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = vec![];
        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(error) => {
                    self.synchronize();
                    self.status.report_compile_error(error);
                }
            }
        }
        statements
    }

    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(TokenType::Let) {
            return self.let_declaration();
        }
        self.statement()
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(TokenType::Print) {
            return self.print_statement()
        }
        if self.match_token(TokenType::OpenBrace) {
            return Ok(Stmt::new_block(self.block()?))
        }
        self.expr_stmt()
    }

    fn expr_stmt(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression")?;
        Ok(Stmt::new_stmt_expr(expr))
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts: Vec<Stmt> = vec!();
        while !self.match_token(TokenType::CloseBrace) && !self.is_at_end() {
            stmts.push(self.declaration()?);
        }
        self.consume(TokenType::CloseBrace, "Expect '}' after a block");
        Ok(stmts)
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let value: Expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::new_print(value))
    }


    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.equality()?;

        if !self.match_token(TokenType::Equal) {
            return Ok(expr)
        } 

        let value: Expr = self.assignment()?;

        match expr {
            Expr::Var(var) => Ok(Expr::new_assign(var.name, value)),
            _ => {
                let equals = self.previous().clone();
                Err(ParseError::new(equals, "Invalid assignment target."))
            }
        }
    }

    

    fn let_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name = self
            .consume(TokenType::Identifier, "Expect variable name.")?
            .clone();

        let mut initializer = None;
        if self.match_token(TokenType::Equal) {
            initializer = Some(self.expression()?);
        }

        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.")?;
        Ok(Stmt::new_let(name, initializer))
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = self.comparison()?;
        
        while self.match_tokens(vec!(
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
        while self.match_tokens(vec!(
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
        while self.match_tokens(vec!(
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
        while self.match_tokens(vec!(
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
        if self.match_tokens(vec!(
            TokenType::Bang,
            TokenType::Minus,
        )) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::new_unary(operator, right))
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        let expr = match self.advance().token_type {
            TokenType::False => 
                Expr::new_literal(LitType::Bool(false)),
                
            TokenType::True => 
                Expr::new_literal(LitType::Bool(true)),

            TokenType::Nil => 
                Expr::new_literal(LitType::None),

            TokenType::Number | TokenType::String =>
                Expr::new_literal(self.previous().literal.clone()),

            TokenType::Identifier => 
                Expr::new_var(self.previous().clone()),

            TokenType::OpenParen => 
                Expr::new_grouping(self.grouping()?),

            _ => return Err(
                ParseError::new(self.peek().clone(), "Expected expression.")
            ),
        };
        Ok(expr) 
    }

    fn grouping(&mut self) -> Result<Expr, ParseError> {
        let expr = self.expression()?;
        self.consume(TokenType::CloseParen, "Expect ')' after expression.")?;
        Ok(expr)
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

    fn match_tokens(&mut self, types: Vec<TokenType>) -> bool {
        for token_type in types {
            if self.match_token(token_type) {
                return true
            }
        }
        false
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            return true
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

#[derive(Debug)]
pub struct ParseError {
    pub token: Token,
    pub message: String,
}

impl ParseError {
    fn new(token: Token, message: &str) -> Self {
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
