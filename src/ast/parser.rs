use crate::{
    token::{Token, TokenType},
    ast::expr::Expr,
};

#[derive(Debug)]
struct Parser {
    tokens: Vec<Token>,
    curr: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser{
            tokens,
            curr: 0
        }
    }

    fn expressoin(&self) -> Expr {
        self.equality()
    }

    fn equality(&self) -> Expr {
        let expr: Expr = self.comparison();
        
        while self.match_token(vec!(
            TokenType::BangEqual,
            TokenType::EqualEqual
        )) {
            let operator: Token = self.previous();
            let right: Expr = self.comparison();
            expr = Expr::new_binary(expr, operator, right);
        }

        expr
    }

    fn match_token(&self, types: Vec<TokenType>) -> bool {
        for token_type in types {
            if self.check(token_type) {
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

    fn peek(&self) -> Token {
        self.tokens[self.curr]
    }

    fn previous(&self) -> Token {
        self.tokens[self.curr - 1]
    }
}
