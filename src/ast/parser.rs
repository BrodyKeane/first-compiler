use std::{
    sync::{Arc, Mutex},
    rc::Rc,
};

use crate::{
    token::{Token, TokenType, Value},
    error::{ErrorStatus, ParseError},
    callables::callable::FuncType,
    ast::{
        expr::Expr,
        stmt::Stmt,
    }, 
};

pub struct Parser<'a> {
    tokens: Vec<Rc<Token>>,
    curr: usize,
    status: &'a mut ErrorStatus,
}

impl<'a> Parser<'a> {
    pub fn new(status: &'a mut ErrorStatus, tokens: Vec<Rc<Token>>
        ) -> Self {
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
                    self.status.report_compile_error(error);
                    self.synchronize();
                }
            }
        }
        statements
    }

    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        match self.peek().token_type {
            TokenType::Let => {self.advance(); self.let_declaration()},
            TokenType::Fn => {self.advance(); self.func_declaration(FuncType::Function)},
            TokenType::Class => {self.advance(); self.class_declaration()},
            _ => self.stmt(),
        }
    }

    fn stmt(&mut self) -> Result<Stmt, ParseError> {
        match self.peek().token_type {
            TokenType::If => {self.advance(); self.if_stmt()},
            TokenType::Print => {self.advance(); self.print_stmt()},
            TokenType::While => {self.advance(); self.while_stmt()},
            TokenType::For => {self.advance(); self.for_stmt()},
            TokenType::Return => {self.advance(); self.return_stmt()},
            TokenType::OpenBrace => {
                self.advance(); Ok(Stmt::new_block(self.block()?))
            },
            _ => self.expr_stmt(),
        }
    }

    fn if_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::OpenParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::CloseParen, 
                     "Expect ')' after 'if' condition.")?;

        let body = self.stmt()?;
        let mut else_body = None;
        if self.match_token(TokenType::Else) {
            else_body = Some(self.stmt()?);
        }
        Ok(Stmt::new_if(condition, body, else_body))
    }


    fn print_stmt(&mut self) -> Result<Stmt, ParseError> {
        let value: Expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::new_print(value))
    }

    fn while_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::OpenParen, "Expect '(' after while.")?;
        let condition = self.expression()?;
        self.consume(TokenType::CloseParen, "Expect ')' after condition")?;
        let body = self.stmt()?;
        Ok(Stmt::new_while(condition, body))
    }

    fn for_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::OpenParen, "Expect '(' after 'for'.")?;
        let initializer = match self.peek().token_type {
            TokenType::Semicolon => {self.advance(); None},
            TokenType::Let => {self.advance(); Some(self.let_declaration()?)},
            _ => Some(self.expr_stmt()?),
        };

        let condition = match self.check(TokenType::Semicolon) {
            true => Expr::new_literal(Arc::new(Mutex::new(Value::Bool(true)))),
            false => self.expression()?,
        };
        self.consume(TokenType::Semicolon, 
                     "Expect ';' after loop condition.")?;

        let increment = match self.check(TokenType::CloseParen) {
            true => None,
            false => Some(self.expression()?),
        };
        self.consume(TokenType::CloseParen, 
                     "Expect ')' after for clauses.")?;

        let mut body = self.stmt()?;

        if let Some(increment) = increment {
            body = Stmt::new_block(vec!(
                body,
                Stmt::new_stmt_expr(increment),
            ));
        }
        
        body = Stmt::new_while(condition, body);

        if let Some(initializer) = initializer {
            body = Stmt::new_block(vec!(initializer, body));
        }
        Ok(body)
    }

    fn return_stmt(&mut self) -> Result<Stmt, ParseError> {
        let keyword = self.previous();
        let value = match self.match_token(TokenType::Semicolon) {
            true => None,
            false => Some(self.expression()?),
        };
        self.consume(TokenType::Semicolon, "Expect ';' after return value.")?;
        Ok(Stmt::new_return(keyword, value))
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts: Vec<Stmt> = vec!();
        while !self.check(TokenType::CloseBrace) && !self.is_at_end() {
            stmts.push(self.declaration()?);
        }
        self.consume(TokenType::CloseBrace, "Expect '}' after a block")?;
        Ok(stmts)
    }

    fn expr_stmt(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression")?;
        Ok(Stmt::new_stmt_expr(expr))
    }

    fn let_declaration(&mut self) -> Result<Stmt, ParseError> {
        let token = self
            .consume(TokenType::Identifier, "Expect variable name.")?;

        let mut initializer = None;
        if self.match_token(TokenType::Equal) {
            initializer = Some(self.expression()?);
        }

        self.consume(TokenType::Semicolon, 
                     "Expect ';' after variable declaration.")?;
        Ok(Stmt::new_let(token, initializer))
    }

    fn func_declaration(&mut self, func_type: FuncType) -> Result<Stmt, ParseError> {
        let message = format!("Expect {:?} name.", func_type);
        let token = self.consume(TokenType::Identifier, &message)?;
        self.consume(TokenType::OpenParen, "Expect '(' after function name.")?;

        let mut params = vec!();
        while !self.check(TokenType::CloseParen) {
            if params.len() >= 255 {
                return Err(ParseError::new(self.peek(), "Can't have more than 255 parameters."))
            }
            params.push(self.consume(TokenType::Identifier, "Expect parameter name.")?);
            if !self.match_token(TokenType::Comma) {
                break
            }
        }
        self.consume(TokenType::CloseParen, "Expect ')' after parameters.")?;
        let message = format!("Expect '{{' before {:?} body", func_type);
        self.consume(TokenType::OpenBrace, &message)?;
        let body = self.block()?;
        Ok(Stmt::new_func(token, params, body))
    }

    fn class_declaration(&mut self) -> Result<Stmt, ParseError> {
        let token = self.consume(TokenType::Identifier, "Expect class name.")?;
        self.consume(TokenType::OpenBrace, "Expect '{' before class body.")?;

        let mut methods = vec!();
        while !self.check(TokenType::CloseBrace) && !self.is_at_end() {
            methods.push(self.func_declaration(FuncType::Method)?);
        }
        self.consume(TokenType::CloseBrace, "Expect '}' after class body.")?;
        let class = Stmt::new_class(token, methods);
        Ok(class)
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.or()?;

        if !self.match_token(TokenType::Equal) {
            return Ok(expr)
        }

        let value: Expr = self.assignment()?;

        match expr {
            Expr::Var(var) => Ok(Expr::new_assign(var.token, value)),
            Expr::Get(var) => Ok(Expr::new_set(*var.object, var.token, value)),
            _ => {
                let equals = self.previous();
                Err(ParseError::new(equals, "Invalid assignment target."))
            }
        }
    }

    fn or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.and()?;

        while self.match_token(TokenType::Or) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Expr::new_logical(expr, operator, right);
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;

        while self.match_token(TokenType::And) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::new_logical(expr, operator, right);
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = self.comparison()?;
        
        while self.match_tokens(vec!(
            TokenType::BangEqual,
            TokenType::EqualEqual
        )) {
            let operator = self.previous();
            let right = self.comparison()?;
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
            let operator = self.previous();
            let right = self.term()?;
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
            let operator = self.previous();
            let right = self.factor()?;
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
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::new_binary(expr, operator, right);
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_tokens(vec!(
            TokenType::Bang,
            TokenType::Minus,
        )) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::new_unary(operator, right))
        }
        self.call()
    }

    fn call(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.primary()?;
        loop {
            if self.match_token(TokenType::OpenParen) {
                expr = self.finish_call(expr)?;
            } 
            else if self.match_token(TokenType::Dot) {
                let token = self.consume(TokenType::Identifier,
                    "Expect property name after '.'")?;
                expr = Expr::new_get(expr, token);
            } 
            else {break}
        }
        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        let expr = match self.advance().token_type {
            TokenType::False => 
                Expr::new_literal(Arc::new(Mutex::new(Value::Bool(false)))),
                
            TokenType::True => 
                Expr::new_literal(Arc::new(Mutex::new(Value::Bool(true)))),

            TokenType::Nil => 
                Expr::new_literal(Arc::new(Mutex::new(Value::None))),

            TokenType::Number | TokenType::String =>
                Expr::new_literal(self.previous().literal.clone()),

            TokenType::Identifier => 
                Expr::new_var(self.previous()),

            TokenType::OpenParen => 
                Expr::new_grouping(self.grouping()?),

            TokenType::This => 
                Expr::new_this(self.previous()),

            _ => return Err(
                ParseError::new(self.previous(), "Expected expression.")
            ),
        };
        Ok(expr) 
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, ParseError> {
        let mut args = vec!();
        while !self.check(TokenType::CloseParen) {
            args.push(self.expression()?);
            if args.len() > 255 {
                return Err(ParseError::new(self.peek(), 
                               "Can't have more than 255 arguments.")
                )
            }
            if !self.match_token(TokenType::Comma) {
                break
            }
        }
        let paren = self.consume(TokenType::CloseParen,
                                 "Expected ')' after arguments.")?;

        Ok(Expr::new_call(callee, paren, args))
    }

    fn grouping(&mut self) -> Result<Expr, ParseError> {
        let expr = self.expression()?;
        self.consume(TokenType::CloseParen, 
                     "Expect ')' after expression.")?;
        Ok(expr)
    }

    fn consume(&mut self, token_type: TokenType, message: &str
        ) -> Result<Rc<Token>, ParseError> {
        match self.check(token_type) {
            true =>  Ok(self.advance()),
            false => Err(
                ParseError::new(self.peek(), message)
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

    fn advance(&mut self) -> Rc<Token> {
        if !(self.is_at_end()) { self.curr += 1 }
        self.previous()
    }
    
    fn peek(&self) -> Rc<Token> {
        self.tokens[self.curr].clone()
    }

    fn previous(&self) -> Rc<Token> {
        self.tokens[self.curr - 1].clone()
    }

    //jumps to start of next statement
    fn synchronize(&mut self) {
        while !(self.is_at_end()) {
            self.advance();
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
    }
}


