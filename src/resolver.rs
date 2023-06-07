use std::collections::hash_map::HashMap;
use std::rc::Rc;
use std::mem;

use crate::{
    interpreter::Interpreter,
    ast::{
        expr::{self, ExprVisitor, Expr, AcceptExprVisitor},
        stmt::{self, StmtVisitor, Stmt, AcceptStmtVisitor, Func},
    },
    token::Token,
    error::{ErrorStatus, ParseError, RuntimeError},
    callables::callable::FuncType,
};

enum ClassType {
    Class,
    SubClass,
    None,
}

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>, //bool represent weather the entry has been resolved yet.
    func_type: FuncType,
    class_type: ClassType,
    status: &'a mut ErrorStatus,
}

impl<'a> Resolver<'a>  {
    pub fn new(status: &'a mut ErrorStatus,
        interpreter: &'a mut Interpreter) -> Self {
        Resolver {
            interpreter,
            scopes: vec!(),
            func_type: FuncType::None,
            class_type: ClassType::None,
            status
        } 
    }

    pub fn resolve(&mut self, stmts: &Vec<Stmt>) {
        self.begin_scope();
        self.resolve_stmts(stmts);
        self.end_scope();
    }

    fn resolve_stmts(&mut self, stmts: &Vec<Stmt>) {
        for stmt in stmts {
            self.resolve_stmt(stmt);
        }
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) {
        stmt.accept(self)
    }

    fn resolve_expr(&mut self, expr: &Expr) {
        expr.accept(self)
    }
    
    fn resolve_local(&mut self, id: u64, token: Rc<Token>) {
        let scopes = self.scopes.iter().rev().enumerate();
        for (depth, scope) in scopes {
            if scope.contains_key(&token.lexeme.to_string()) {
                self.interpreter.resolve(id, depth);
                return
            }
        }
    }

    fn resolve_func(&mut self, func: &Func, func_type: FuncType) {
        let enclosing_func = mem::replace(&mut self.func_type, func_type);

        self.begin_scope();
        for param in &func.params {
            self.declare(Rc::clone(param));
            self.define(Rc::clone(param));
        }
        self.resolve_stmts(&func.body);
        self.end_scope();

        self.func_type = enclosing_func;
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new())
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, token: Rc<Token>) {
        let scope = match self.scopes.last_mut() {
            Some(scope) => scope,
            None => return
        };
        if scope.contains_key(&token.lexeme.to_string()) {
            let error = RuntimeError::new(Rc::clone(&token),
                "Already variable with this name declared in this scope."
            );
            self.status.report_runtime_error(error)
        }
        scope.insert(token.lexeme.to_string(), false);
    }

    fn define(&mut self, token: Rc<Token>) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(token.lexeme.to_string(), true);
        }
    }

    fn is_accessed_in_initializer(&self, var_name: Rc<String>) -> bool {
        let scope = match self.scopes.last() {
            Some(scope) => scope,
            None => return false,
        };
        match scope.get(&var_name.to_string()) {
            Some(resolved) => !*resolved,
            None => false,
        }
    }
}

impl StmtVisitor for Resolver<'_> {
    type Output = ();

    fn visit_block_stmt(&mut self, stmt: &stmt::Block) -> Self::Output {
        self.begin_scope();
        self.resolve_stmts(&stmt.stmts);
        self.end_scope();
    }

    fn visit_let_stmt(&mut self, stmt: &stmt::Let) -> Self::Output {
        self.declare(Rc::clone(&stmt.token));
        if let Some(init) = &stmt.initializer {
            self.resolve_expr(init);
        }
        self.define(Rc::clone(&stmt.token));
    }

    fn visit_func_stmt(&mut self, stmt: &stmt::Func) -> Self::Output {
        self.declare(Rc::clone(&stmt.token));
        self.define(Rc::clone(&stmt.token));
        self.resolve_func(stmt, FuncType::Function);
    }

    fn visit_expr_stmt(&mut self, stmt: &stmt::StmtExpr) -> Self::Output {
       self.resolve_expr(&stmt.expr);
    }

    fn visit_if_stmt(&mut self, stmt: &stmt::If) -> Self::Output {
        self.resolve_expr(&stmt.condition);
        self.resolve_stmt(&stmt.body);
        if let Some(else_body) = &stmt.else_body {
            self.resolve_stmt(else_body);
        }
    }

    fn visit_print_stmt(&mut self, stmt: &stmt::Print) -> Self::Output {
        self.resolve_expr(&stmt.expr);
    }

    fn visit_return_stmt(&mut self, stmt: &stmt::Return) -> Self::Output {
        if let FuncType::None = self.func_type {
            let error = RuntimeError::new(Rc::clone(&stmt.keyword),
                "Can't return from top-level code.");
            self.status.report_runtime_error(error);
        }

        if let Some(value) = &stmt.value {
            if let FuncType::Initializer = self.func_type {
                let error = RuntimeError::new(Rc::clone(&stmt.keyword),
                    "Can't return a value from an initializer.");
                self.status.report_runtime_error(error);
            }
            self.resolve_expr(value);
        }
    }

    fn visit_while_stmt(&mut self, stmt: &stmt::While) -> Self::Output {
        self.resolve_expr(&stmt.condition);
        self.resolve_stmt(&stmt.body);
    }

    fn visit_class_stmt(&mut self, stmt: &stmt::Class) -> Self::Output {
        let enclosing_class = mem::replace(&mut self.class_type, ClassType::Class);
        self.declare(Rc::clone(&stmt.token));
        self.define(Rc::clone(&stmt.token));

        if let Some(expr) = &stmt.superclass {
            self.class_type = ClassType::SubClass;
            if let Expr::Var(superclass) = expr {
                if stmt.token.lexeme == superclass.token.lexeme {
                    let error = RuntimeError::new(Rc::clone(&superclass.token), 
                        "A class can't inherit from itself.");
                    self.status.report_runtime_error(error);
                }
            self.resolve_expr(expr);
            }
        }

        if stmt.superclass.is_some() {
            self.begin_scope();
            self.scopes.last_mut().unwrap().insert("super".to_string(), true);
        }
        
        self.begin_scope();
        self.scopes.last_mut().unwrap().insert("this".to_string(), true);

        for wrapped_method in &stmt.methods {
            let method = match wrapped_method {
                Stmt::Func(func) => func,
                _ => {
                    let error = RuntimeError::new(Rc::clone(&stmt.token), "Undefined method");
                    self.status.report_runtime_error(error);
                    continue
                }
            };
            let lexeme = method.token.lexeme.as_str();
            let func_type = match lexeme == "init" {
                true => FuncType::Initializer,
                false => FuncType::Method,
            };
            self.resolve_func(method, func_type);
        }

        self.end_scope();
        if stmt.superclass.is_some() {self.end_scope()}
        self.class_type = enclosing_class;
    }
}

impl ExprVisitor for Resolver<'_> {
    type Output = ();

    fn visit_var_expr(&mut self, expr: &expr::Var) -> Self::Output {
        if self.is_accessed_in_initializer(Rc::clone(&expr.token.lexeme)) {
            let error = ParseError::new(Rc::clone(&expr.token),
                "Can't read local variable in its own initializer."
            );
            self.status.report_compile_error(error);
        }
        self.resolve_local(expr.id, Rc::clone(&expr.token));
    }

    fn visit_assign_expr(&mut self, expr: &expr::Assign) -> Self::Output {
        self.resolve_expr(&expr.value);
        self.resolve_local(expr.id, Rc::clone(&expr.token));
    }

    fn visit_binary_expr(&mut self, expr: &expr::Binary) -> Self::Output {
        self.resolve_expr(&expr.left);
        self.resolve_expr(&expr.right);
    }

    fn visit_call_expr(&mut self, expr: &expr::Call) -> Self::Output {
        self.resolve_expr(&expr.callee);
        for arg in &expr.args {
            self.resolve_expr(arg);
        }
    }

    fn visit_grouping_expr(&mut self, expr: &expr::Grouping) -> Self::Output {
        self.resolve_expr(&expr.expr);
    }

    fn visit_literal_expr(&mut self, _expr: &expr::Literal) -> Self::Output {}

    fn visit_logical_expr(&mut self, expr: &expr::Logical) -> Self::Output {
        self.resolve_expr(&expr.left);
        self.resolve_expr(&expr.right);
    }

    fn visit_unary_expr(&mut self, expr: &expr::Unary) -> Self::Output {
        self.resolve_expr(&expr.right);
    }

    fn visit_get_expr(&mut self, expr: &expr::Get) -> Self::Output {
        self.resolve_expr(&expr.object);
    }
    
    fn visit_set_expr(&mut self, expr: &expr::Set) -> Self::Output {
        self.resolve_expr(&expr.value);
        self.resolve_expr(&expr.object);
    }

    fn visit_this_expr(&mut self, expr: &expr::This) -> Self::Output {
        match self.class_type {
            ClassType::None => {
                let error = RuntimeError::new(Rc::clone(&expr.keyword), 
                        "Can't use 'this' outside of a class.");
                self.status.report_runtime_error(error);
            },
            _ => self.resolve_local(expr.id, Rc::clone(&expr.keyword)),
        }
    }

    fn visit_super_expr(&mut self, expr: &expr::Super) -> Self::Output {
        match self.class_type {
            ClassType::None => {
                let error = RuntimeError::new(Rc::clone(&expr.keyword), 
                    "Can't use 'super' outside of a class.");
                self.status.report_runtime_error(error);
            }

            ClassType::Class => {
                let error = RuntimeError::new(Rc::clone(&expr.keyword), 
                    "Can't use 'super' in a class with no superclass.");
                self.status.report_runtime_error(error);
            }

            ClassType::SubClass => 
                self.resolve_local(expr.id, Rc::clone(&expr.keyword)),
        }
    }
}

