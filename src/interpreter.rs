use crate::{
    ast::expr::{self, Expr, Data},
    token::{self, LitType, TokenType},
};

struct Interpreter;

impl Interpreter {
    fn evaluate(&mut self, expr: &Box<Expr>) -> token::LitType {
        expr.accept(self)
    }

    fn is_truthy(&self, value: LitType) -> bool {
        match value {
            LitType::Bool(val) => val,
            LitType::None => false,
            _ => true
        }
    }

}

impl expr::Visitor for Interpreter {
    type Output = token::LitType;

    fn visit_literal_expr(&mut self, expr: &expr::Literal) -> LitType {
        expr.value.clone()
    }

    fn visit_grouping_expr(&mut self, expr: &expr::Grouping) -> LitType {
        self.evaluate(&expr.expr)
    }

    fn visit_unary_expr(&mut self, expr: &expr::Unary) -> LitType {
        let output: LitType = self.evaluate(&expr.right);
        let token_type = &expr.operator.token_type;
        match (output, token_type) {
            (LitType::Num(val), TokenType::Minus) => LitType::Num(-val),
            (val, TokenType::Bang) => LitType::Bool(!self.is_truthy(val)),
            _ => LitType::None,
        }
    }

    fn visit_binary_expr(&mut self, expr: &expr::Binary) -> LitType {
        let left: LitType = self.evaluate(&expr.left);
        let right: LitType = self.evaluate(&expr.right);
        let token_type = &expr.operator.token_type;
        match (left, right) {
            (LitType::Num(left), LitType::Num(right)) => match token_type {
                TokenType::Plus => LitType::Num(left + right),
                TokenType::Minus => LitType::Num(left - right),
                TokenType::Star => LitType::Num(left * right),
                TokenType::Slash => LitType::Num(left / right),
                TokenType::Greater => LitType::Bool(left > right),
                TokenType::GreaterEqual => LitType::Bool(left >= right),
                TokenType::Less => LitType::Bool(left < right),
                TokenType::LessEqual => LitType::Bool(left <= right),
                _ => LitType::None,
            },
            (LitType::String(left), LitType::String(right)) => match token_type {
                TokenType::Plus => LitType::String(left + &right),
                _ => LitType::None
            },
            (left, right)=> match token_type {
                TokenType::EqualEqual => LitType::Bool(left == right),
                TokenType::BangEqual => LitType::Bool(left != right),
                _ => LitType::None,
            }
        }
    }
}


