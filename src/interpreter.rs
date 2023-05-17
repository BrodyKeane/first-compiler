use crate::{
    ast::expr::{self, Expr, Data},
    token::{Token, LitType, TokenType},
};

struct Interpreter;

impl Interpreter {
    fn evaluate(&mut self, expr: &Box<Expr>) -> Result<LitType, RuntimeError> {
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
    type Output = Result<LitType, RuntimeError>;

    fn visit_literal_expr(&mut self, expr: &expr::Literal
        ) -> Result<LitType, RuntimeError> {
        Ok(expr.value.clone())
    }

    fn visit_grouping_expr(&mut self, expr: &expr::Grouping
        ) -> Result<LitType, RuntimeError> {
        self.evaluate(&expr.expr)
    }

    fn visit_unary_expr(&mut self, expr: &expr::Unary
        ) -> Result<LitType, RuntimeError> {
        let output: LitType = self.evaluate(&expr.right)?;
        let token_type = &expr.operator.token_type;
        Ok(match (output, token_type) {
            (LitType::Num(val), TokenType::Minus) => LitType::Num(-val),
            (_, TokenType::Minus) => return Err(
                RuntimeError::new(expr.operator.clone(), "Operand must be a number.")
            ),
            (val, TokenType::Bang) => LitType::Bool(!self.is_truthy(val)),
            _ => LitType::None,
        })
    }

    fn visit_binary_expr(&mut self, expr: &expr::Binary
        ) -> Result<LitType, RuntimeError> {
        let left: LitType = self.evaluate(&expr.left)?;
        let right: LitType = self.evaluate(&expr.right)?;
        let token_type = &expr.operator.token_type;
        Ok(match (left, right) {
            (LitType::Num(left), LitType::Num(right)) => match token_type {
                TokenType::Plus => LitType::Num(left + right),
                TokenType::Minus => LitType::Num(left - right),
                TokenType::Star => LitType::Num(left * right),
                TokenType::Slash => LitType::Num(left / right),
                TokenType::Greater => LitType::Bool(left > right),
                TokenType::GreaterEqual => LitType::Bool(left >= right),
                TokenType::Less => LitType::Bool(left < right),
                TokenType::LessEqual => LitType::Bool(left <= right),
                _ => return Err(RuntimeError::new(
                    expr.operator.clone(),
                    "Operator cannot be used on numbers"
                )),
            },
            (LitType::String(left), LitType::String(right)) => match token_type {
                TokenType::Plus => LitType::String(left + &right),
                _ => return Err(RuntimeError::new(
                    expr.operator.clone(),
                    "Operator cannot be used on strings"
                ))
            },
            (left, right)=> match token_type {
                TokenType::EqualEqual => LitType::Bool(left == right),
                TokenType::BangEqual => LitType::Bool(left != right),
                _ => return Err(RuntimeError::new(
                    expr.operator.clone(),
                    "Operator cannot be used on values of this type"
                ))
            }
        })
    }
}

pub struct RuntimeError {
    token: Token,
    message: String,
}

impl RuntimeError {
    pub fn new(token: Token, message: &str) -> Self {
        RuntimeError { token, message: message.to_string() }
    }
}

