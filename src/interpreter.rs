use std::error::Error;
use std::fmt;
use std::mem;

use crate::{
    ast::{
        expr::{self, Expr, AcceptExprVisitor, ExprVisitor},
        stmt::{self, Stmt, AcceptStmtVisitor, StmtVisitor},
    },
    token::{Token, LitType, TokenType},
    environment::Environment,
};

pub struct Interpreter {
    enviroment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
       Interpreter { enviroment: Environment::new(None) }
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

    fn execute_block(&mut self, stmts: &Vec<Stmt>) -> Result<(), RuntimeError> {
        self.enviroment = Environment::new(Some(self.enviroment.clone()));
        for stmt in stmts {
            self.execute(stmt)?;
        }
        self.enviroment = Environment::new(*self.enviroment.enclosing.clone());
        Ok(())
    }

    fn is_truthy(&self, value: &LitType) -> bool {
        match value {
            LitType::Bool(val) => val.to_owned(),
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
            (val, TokenType::Bang) => LitType::Bool(!self.is_truthy(&val)),
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

    fn visit_logical_expr(&mut self, expr: &expr::Logical) -> Self::Output {
        let left = self.evaluate(&expr.left)?;
        //tries to short circuit logical if result can be determined ealry
        match expr.operator.token_type {
            TokenType::Or => if self.is_truthy(&left) {
                return Ok(left)
            },
            _ => if !self.is_truthy(&left) {
                return Ok(left)
            },
        }
        self.evaluate(&expr.right)
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
            Some(expr) => self.evaluate(expr)?,
            None => LitType::None,
        };
        self.enviroment.define(&stmt.name.lexeme, value);
        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: &stmt::Block) -> Self::Output {
        self.execute_block(&stmt.stmts)?;
        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: &stmt::If) -> Self::Output {
        let literal = self.evaluate(&stmt.condition)?;
        if self.is_truthy(&literal) {
            self.execute(&stmt.body)?;
        } else if let Some(else_body) = &*stmt.else_body {
            self.execute(else_body)?;
        }
        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &stmt::While) -> Self::Output {
        let mut condition_result;
        while {
            condition_result = self.evaluate(&stmt.condition)?; 
            self.is_truthy(&condition_result)
        } { 
            self.execute(&stmt.body)?;
        }
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
        write!(f, "[line {}] {}", self.token.line, self.message)
    }
}
