use std::error::Error;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use crate::{
    ast::{
        expr::{self, Expr, AcceptExprVisitor, ExprVisitor},
        stmt::{self, Stmt, AcceptStmtVisitor, StmtVisitor},
    },
    token::{Token, Value, TokenType},
    environment::Environment,
    native_functions::NativeDeclarations,
};

pub struct Interpreter {
    globals: Arc<Mutex<Environment>>,
    environment: Arc<Mutex<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut globals = Environment::new_wrapped(None);
        let mut native = NativeDeclarations::new(Arc::clone(&globals));
        globals = native.declare_natives();
        let environment = Environment::new_wrapped(Some(Arc::clone(&globals)));
        Interpreter { globals, environment }
    }

    pub fn interpret(&mut self, stmts: &Vec<Stmt>) -> Result<(), RuntimeError>{
        for stmt in stmts {
            self.execute(stmt)?;
        };
        Ok(())
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Rc<Value>, RuntimeError> {
        expr.accept(self)
    }
 
    fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        stmt.accept(self)
    }

    fn execute_block(&mut self, stmts: &Vec<Stmt>) -> Result<(), RuntimeError> {
        let prev = Arc::clone(&self.environment);
        self.environment = Environment::new_wrapped(Some(Arc::clone(&prev)));
        for stmt in stmts {
            self.execute(stmt)?;
        }
        self.environment = prev;
        Ok(())
    }

    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Bool(val) => val.to_owned(),
            Value::None => false,
            _ => true
        }
    }
}

impl ExprVisitor for Interpreter {
    type Output = Result<Rc<Value>, RuntimeError>;

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
        let binding = self.evaluate(&expr.right)?;
        let output = binding.deref();
        let token_type = &expr.operator.token_type;
        Ok(Rc::new( match (output, token_type) {
            (Value::Num(val), TokenType::Minus) => Value::Num(-val),
            (_, TokenType::Minus) => return Err(
                RuntimeError::new(expr.operator.clone(), "Operand must be a number.")
            ),
            (val, TokenType::Bang) => Value::Bool(!self.is_truthy(&val)),
            _ => Value::None,
        }))
    }

    fn visit_binary_expr(&mut self, expr: &expr::Binary
        ) -> Self::Output {
        let binding = self.evaluate(&expr.left)?;
        let left = binding.deref();
        let binding = self.evaluate(&expr.right)?;
        let right = binding.deref();
        let token_type = &expr.operator.token_type;
        let value = match (left, right) {
            (Value::Num(left), Value::Num(right)) => match token_type {
                TokenType::Plus => Value::Num(left + right),
                TokenType::Minus => Value::Num(left - right),
                TokenType::Star => Value::Num(left * right),
                TokenType::Slash => Value::Num(left / right),
                TokenType::Greater => Value::Bool(left > right),
                TokenType::GreaterEqual => Value::Bool(left >= right),
                TokenType::Less => Value::Bool(left < right),
                TokenType::LessEqual => Value::Bool(left <= right),
                _ => return Err(RuntimeError::new(
                    expr.operator.clone(),
                    "Operator cannot be used on numbers"
                )),
            },

            (Value::String(left), Value::String(right)) => match token_type {
                TokenType::Plus => Value::String(left.to_string() + right),
                _ => return Err(RuntimeError::new(
                    expr.operator.clone(),
                    "Operator cannot be used on strings"
                ))
            },

            (left, right) => match token_type {
                TokenType::EqualEqual => Value::Bool(left == right),
                TokenType::BangEqual => Value::Bool(left != right),
                _ => return Err(RuntimeError::new(
                    expr.operator.clone(),
                    "Operator cannot be used on values of this type"
                ))
            }
        };
        Ok(Rc::new(value))
    }

    fn visit_var_expr(&mut self, expr: &expr::Var) -> Self::Output {
        Ok(self.environment.lock().unwrap().get(expr.name.clone())?)
    }

    fn visit_assign_expr(&mut self, expr: &expr::Assign) -> Self::Output {
        let value = self.evaluate(&expr.value)?;
        self.environment
            .lock()
            .unwrap()
            .assign(expr.name.clone(), value.clone())?;
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

    fn visit_call_expr(&mut self, expr: &expr::Call) -> Self::Output {
        let callee = self.evaluate(&expr.callee)?;
        let function = match *callee {
            Value::Callable(callee) => callee,
            _ => {
                let message = "Can only call functions and classes.";
                return Err(RuntimeError::new(expr.paren.clone(), message))
            }
        };

        let mut args = vec!();
        for arg in &expr.args {
            args.push(self.evaluate(&arg)?);
        }
        
        if args.len() != function.arity() {
            let message = format!("Expected {} arguments but got {}.",
                                  function.arity(), args.len());
            return Err(RuntimeError::new(expr.paren.clone(), &message))
        }
        Ok(Rc::new(function.call(&self, args)))
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
            None => Rc::new(Value::None),
        };
        self.environment
            .lock()
            .unwrap()
            .define(stmt.name.lexeme.clone(), value);
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
    token: Rc<Token>,
    message: String,
}

impl RuntimeError {
    pub fn new(token: Rc<Token>, message: &str) -> Self {
        RuntimeError { token, message: message.to_string() }
    }
}

impl Error for RuntimeError {}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[line {}] {}", self.token.line, self.message)
    }
}
