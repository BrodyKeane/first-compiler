use std::{
    collections::HashMap, 
    rc::Rc,
    sync::{Arc, Mutex, RwLock},
};

use crate::{
    callables::{
        native_functions::NativeDeclarations,
        lax_functions::LaxFn,
        Callable,
    },
    ast::{
        expr::{self, Expr, AcceptExprVisitor, ExprVisitor},
        stmt::{self, Stmt, AcceptStmtVisitor, StmtVisitor},
    },
    token::{Value, TokenType},
    environment::Environment,
    error::RuntimeError,
    token::Token,
};

pub struct Interpreter {
    pub globals: Arc<Mutex<Environment>>,
    environment: Arc<Mutex<Environment>>,
    locals: HashMap<u64, usize>
}

impl Interpreter {
    pub fn new() -> Self {
        let mut globals = Environment::new_wrapped(None);
        let mut native = NativeDeclarations::new(Arc::clone(&globals));
        globals = native.declare_natives();
        let environment = Environment::new_wrapped(Some(Arc::clone(&globals)));
        
        Interpreter {
            globals,
            environment,
            locals: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, stmts: &Vec<Stmt>) -> Result<(), RuntimeError>{
        for stmt in stmts {
            self.execute(stmt)?;
        };
        Ok(())
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<Option<Arc<RwLock<Value>>>, RuntimeError> {
        stmt.accept(self)
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Arc<RwLock<Value>>, RuntimeError> {
        expr.accept(self)
    }

    pub fn resolve(&mut self, id: u64, depth: usize) {
        self.locals.insert(id, depth);
    }

    pub fn execute_block(&mut self, stmts: &Vec<Stmt>, env: Arc<Mutex<Environment>>
        ) -> Result<Option<Arc<RwLock<Value>>>, RuntimeError> {
        let prev = std::mem::replace(&mut self.environment, env);
        for stmt in stmts {
            if let Some(val) = self.execute(stmt)? {
                self.environment = prev;
                return Ok(Some(val))
            }
        }
        self.environment = prev;
        Ok(None)
    }

    fn lookup_variable(&self, token: Rc<Token>, id: u64
        ) -> Result<Arc<RwLock<Value>>, RuntimeError> {
        match self.locals.get(&id) {
            Some(depth) => {
                self.environment
                    .lock()
                    .unwrap()
                    .get_at(depth.to_owned(), token)
            },
            None => self.globals.lock().unwrap().get(token),
        }
    }

    fn is_truthy(&self, value: &Arc<RwLock<Value>>) -> bool {
        match *value.read().unwrap() {
            Value::Bool(val) => val.to_owned(),
            Value::None => false,
            _ => true,
        }
    }
}

impl ExprVisitor for Interpreter {
    type Output = Result<Arc<RwLock<Value>>, RuntimeError>;

    fn visit_literal_expr(&mut self, expr: &expr::Literal
        ) -> Self::Output {
        Ok(Arc::clone(&expr.value))
    }

    fn visit_grouping_expr(&mut self, expr: &expr::Grouping
        ) -> Self::Output {
        self.evaluate(&expr.expr)
    }

    fn visit_unary_expr(&mut self, expr: &expr::Unary
        ) -> Self::Output {
        let binding = self.evaluate(&expr.right)?;
        let output = binding.read().unwrap();
        let token_type = &expr.operator.token_type;

        let value = match (&*output, token_type) {
            (Value::Num(val), TokenType::Minus) => Value::Num(-val),
            (_, TokenType::Minus) => return Err(
                RuntimeError::new(Rc::clone(&expr.operator), "Operand must be a number.")
            ),
            (_, TokenType::Bang) => Value::Bool(!self.is_truthy(&binding)),
            _ => Value::None,
        };
        Ok(Arc::new(RwLock::new(value)))
    }

    fn visit_binary_expr(&mut self, expr: &expr::Binary
        ) -> Self::Output {
        let binding = self.evaluate(&expr.left)?;
        let left = binding.read().unwrap();
        let binding = self.evaluate(&expr.right)?;
        let right = binding.read().unwrap();
        let token_type = &expr.operator.token_type;
        let value = match (&*left, &*right) {
            (Value::Num(left), Value::Num(right)) => match token_type {
                TokenType::Plus => Value::Num(left + right),
                TokenType::Minus => Value::Num(left - right),
                TokenType::Star => Value::Num(left * right),
                TokenType::Slash => Value::Num(left / right),
                TokenType::Greater => Value::Bool(left > right),
                TokenType::GreaterEqual => Value::Bool(left >= right),
                TokenType::Less => Value::Bool(left < right),
                TokenType::LessEqual => Value::Bool(left <= right),
                TokenType::EqualEqual => Value::Bool(left == right),
                _ => return Err(RuntimeError::new(
                    Rc::clone(&expr.operator),
                    "Operator cannot be used on numbers"
                )),
            },

            (Value::String(left), Value::String(right)) => match token_type {
                TokenType::Plus => Value::String(left.to_string() + right),
                TokenType::EqualEqual => Value::Bool(left == right),
                _ => return Err(RuntimeError::new(
                    Rc::clone(&expr.operator),
                    "Operator cannot be used on strings"
                ))
            },

            (left, right) => match token_type {
                TokenType::EqualEqual => Value::Bool(left == right),
                TokenType::BangEqual => Value::Bool(left != right),
                _ => return Err(RuntimeError::new(
                    Rc::clone(&expr.operator),
                    "Operator cannot be used on values of this type"
                ))
            }
        };
        Ok(Arc::new(RwLock::new(value)))
    }

    fn visit_var_expr(&mut self, expr: &expr::Var) -> Self::Output {
        self.lookup_variable(Rc::clone(&expr.token), expr.id)
    }

    fn visit_assign_expr(&mut self, expr: &expr::Assign) -> Self::Output {
        let value = self.evaluate(&expr.value)?;
        match self.locals.get(&expr.id) {
            Some(distance) => {
                self.environment
                    .lock()
                    .unwrap()
                    .assign_at(distance.to_owned(), Rc::clone(&expr.token), Arc::clone(&value))?;
            },
            None => {
                self.globals
                    .lock()
                    .unwrap()
                    .assign(Rc::clone(&expr.token), Arc::clone(&value))?;
            },
        }
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
        let binding = self.evaluate(&expr.callee)?;
        let callee = binding.read().unwrap();
        let function = match &*callee {
            Value::Callable(callee) => callee,
            _ => {
                let message = "Can only call functions and classes.";
                return Err(RuntimeError::new(Rc::clone(&expr.paren), message))
            }
        };

        let mut args = vec!();
        for arg in &expr.args {
            args.push(self.evaluate(arg)?);
        }
        
        if args.len() != function.arity() {
            let message = format!("Expected {} arguments but got {}.",
                                  function.arity(), args.len());
            return Err(RuntimeError::new(Rc::clone(&expr.paren), &message))
        }
        function.call(self, args)
    }

    fn visit_get_expr(&mut self, expr: &expr::Get) -> Self::Output {
        let binding = self.evaluate(&expr.object)?;
        let value = binding.read().unwrap();
        match &*value { 
            Value::LaxObject(object) => {
                Ok(object.lock().unwrap().get(Rc::clone(&expr.token), Arc::clone(&binding))?)
            },
            _ => Err(RuntimeError::new(Rc::clone(&expr.token), 
                    "Only instances have properties."))
        }
    }

    fn visit_set_expr(&mut self, expr: &expr::Set) -> Self::Output {
        let binding = self.evaluate(&expr.object)?;
        let value = binding.write().unwrap();

        let object = match &*value {
            Value::LaxObject(object) => Arc::clone(object),
            _ => return Err(RuntimeError::new(Rc::clone(&expr.token), 
                            "Only instances have fields."))
        };
        
        let value = self.evaluate(&expr.value)?;
        object.lock()
            .unwrap()
            .set(Rc::clone(&expr.token), Arc::clone(&value));
        Ok(value)
    }

    fn visit_this_expr(&mut self, expr: &expr::This) -> Self::Output {
        self.lookup_variable(Rc::clone(&expr.keyword), expr.id)
    }
}

impl StmtVisitor for Interpreter {
    type Output = Result<Option<Arc<RwLock<Value>>>, RuntimeError>;

    fn visit_expr_stmt(&mut self, stmt: &stmt::StmtExpr
        ) -> Self::Output {
        self.evaluate(&stmt.expr)?;
        Ok(None)
    }

    fn visit_print_stmt(&mut self, stmt: &stmt::Print
        ) -> Self::Output {
        let binding = self.evaluate(&stmt.expr)?;
        let value = binding.read().unwrap();
        println!("{}", value);
        Ok(None)
    }

    fn visit_let_stmt(&mut self, stmt: &stmt::Let) -> Self::Output {
        let value = match &stmt.initializer {
            Some(expr) => self.evaluate(expr)?,
            None => Arc::new(RwLock::new(Value::None)),
        };
        self.environment
            .lock()
            .unwrap()
            .define(stmt.token.lexeme.to_string(), value);
        Ok(None)
    }

    fn visit_block_stmt(&mut self, stmt: &stmt::Block) -> Self::Output {
        let env = Environment::new_wrapped(Some(Arc::clone(&self.environment)));
        let value = self.execute_block(&stmt.stmts, env)?;
        Ok(value)
    }

    fn visit_if_stmt(&mut self, stmt: &stmt::If) -> Self::Output {
        let literal = self.evaluate(&stmt.condition)?;
        if self.is_truthy(&literal) {
            self.execute(&stmt.body)
        } else if let Some(else_body) = &stmt.else_body {
            self.execute(else_body)
        } else {
            Ok(None)
        }
    }

    fn visit_while_stmt(&mut self, stmt: &stmt::While) -> Self::Output {
        let mut condition_result;
        while {
            condition_result = self.evaluate(&stmt.condition)?; 
            self.is_truthy(&condition_result)
        } { 
            if let Some(val) = self.execute(&stmt.body)? {
                return Ok(Some(val))
            }
        }
        Ok(None)
    }

    fn visit_func_stmt(&mut self, stmt: &stmt::Func) -> Self::Output {
        let env = Arc::clone(&self.environment);
        let name = stmt.token.lexeme.to_string();
        let func = Callable::new_lax_fn(stmt.clone(), env, false);
        let value = Arc::new(RwLock::new(Value::Callable(func)));
        self.environment.lock().unwrap().define(name, value);
        Ok(None)
    }
    
    fn visit_return_stmt(&mut self, stmt: &stmt::Return) -> Self::Output {
        let value = match &stmt.value {
            Some(value) => Some(self.evaluate(value)?),
            None => None
        };
        Ok(value)
    }

    fn visit_class_stmt(&mut self, stmt: &stmt::Class) -> Self::Output {
        self.environment
            .lock()
            .unwrap()
            .define(stmt.token.lexeme.to_string(), Arc::new(RwLock::new(Value::None)));
        let name = Rc::clone(&stmt.token.lexeme);

        let mut methods: HashMap<String, LaxFn> = HashMap::new();
        for wrapped_method in &stmt.methods {
            let method = match wrapped_method {
                Stmt::Func(func) => func.clone(),
                _ => return Err(
                    RuntimeError::new(Rc::clone(&stmt.token), "Undefined method")
                )
            };
            let name = Rc::clone(&method.token.lexeme);

            let is_init = name.as_str() == "init";
            let function = LaxFn::new(method, Arc::clone(&self.environment), is_init);
            methods.insert(name.to_string(), function);
        }

        let class = Callable::new_lax_class(name, methods);
        let value = Arc::new(RwLock::new(Value::Callable(class)));
        self.environment.lock().unwrap().assign(stmt.token.clone(), value)?;
        Ok(None)
    }
}
