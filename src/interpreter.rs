use std::error::Error;
use std::fmt;

use crate::{
    ast::{
        expr::{self, Expr, AcceptExprVisitor, ExprVisitor},
        stmt::{self, Stmt, AcceptStmtVisitor, StmtVisitor},
    },
    token::{Token, LitType, TokenType},
    enviroment::Enviroment,
};

pub struct Interpreter {
    enviroment: Enviroment,
}

impl Interpreter {
    pub fn new() -> Self {
       Interpreter { enviroment: Enviroment::new() }
    }

    pub fn interpret(&mut self, stmts: &Vec<Stmt>) -> Result<(), RuntimeError>{
        for stmt in stmts {
            self.execute(stmt)?;
        };
        Ok(())
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<LitType, RuntimeError> {
        expr.accept(self)
    }
 
    fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        stmt.accept(self)
    }

    fn is_truthy(&self, value: LitType) -> bool {
        match value {
            LitType::Bool(val) => val,
            LitType::None => false,
            _ => true
        }
    }
}

impl ExprVisitor for Interpreter {
    type Output = Result<LitType, RuntimeError>;

    fn visit_literal_expr(&mut self, expr: &expr::Literal
        ) -> Self::Output {
        Ok(expr.value.clone())
    }

    fn visit_grouping_expr(&mut self, expr: &expr::Grouping
        ) -> Self::Output {
        self.evaluate(&expr.expr)
    }

    fn visit_unary_expr(&mut self, expr: &expr::Unary
        ) -> Self::Output {
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
        ) -> Self::Output {
        let left: LitType = self.evaluate(&expr.left)?;
        let right: LitType = self.evaluate(&expr.right)?;
        let token_type = &expr.operator.token_type;

        let value = match (left, right) {
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

            (left, right) => match token_type {
                TokenType::EqualEqual => LitType::Bool(left == right),
                TokenType::BangEqual => LitType::Bool(left != right),
                _ => return Err(RuntimeError::new(
                    expr.operator.clone(),
                    "Operator cannot be used on values of this type"
                ))
            }
        };
        Ok(value)
    }

    fn visit_var_expr(&mut self, expr: &expr::Var) -> Self::Output {
        self.enviroment.get(&expr.name).cloned()
    }

    fn visit_assign_expr(&mut self, expr: &expr::Assign) -> Self::Output {
        let value: LitType = self.evaluate(&expr.value)?;
        self.enviroment.assign(expr.name.clone(), value.clone())?;
        Ok(value)
    }
}

impl StmtVisitor for Interpreter {
    type Output = Result<(), RuntimeError>;

    fn visit_expression_stmt(&mut self, stmt: &stmt::StmtExpr
        ) -> Self::Output {
        self.evaluate(&stmt.expr)?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &stmt::Print
        ) -> Self::Output {
        let value = self.evaluate(&stmt.expr)?;
        println!("{}", value);
        Ok(())
    }

    fn visit_let_stmt(&mut self, stmt: &stmt::Let) -> Self::Output {
        let value = match &stmt.initializer {
            Some(expr) => self.evaluate(&expr)?,
            None => LitType::None,
        };
        self.enviroment.define(&stmt.name.lexeme, value);
        Ok(())
    }
}

#[derive(Debug)]
pub struct RuntimeError {
    token: Token,
    message: String,
}

impl RuntimeError {
    pub fn new(token: Token, message: &str) -> Self {
        RuntimeError { token, message: message.to_string() }
    }
}

impl Error for RuntimeError {}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n[line {}]", self.message, self.token.line, )
    }
}
