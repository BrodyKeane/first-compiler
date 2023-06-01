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
};

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>, //bool represent weather the entry has been resolved yet.
    func_type: FuncType,
    status: &'a mut ErrorStatus,
}

impl<'a> Resolver<'a> {
    pub fn new(status: &'a mut ErrorStatus, interpreter: &'a mut Interpreter) -> Self {
        Resolver {
            interpreter,
            scopes: vec!(),
            func_type: FuncType::None,
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
            if scope.contains_key(&token.lexeme) {
                self.interpreter.resolve(id, depth);
                return
            }
        }
    }

    fn resolve_func(&mut self, func: &Func, func_type: FuncType) {
        let enclosing_func = mem::replace(&mut self.func_type, func_type);

        self.begin_scope();
        for param in &func.params {
            self.declare(param.clone());
            self.define(param.clone());
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
        if scope.contains_key(&token.lexeme) {
            let error = RuntimeError::new(token.clone(),
                "Already variable with this name declared in this scope."
            );
            self.status.report_runtime_error(error)
        }
        scope.insert(token.lexeme.clone(), false);
    }

    fn define(&mut self, token: Rc<Token>) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(token.lexeme.clone(), true);
        }
    }

    fn is_accessed_in_initializer(&self, var_name: String) -> bool {
        let scope = match self.scopes.last() {
            Some(scope) => scope,
            None => return false,
        };
        match scope.get(&var_name) {
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
        self.declare(stmt.token.clone());
        if let Some(init) = &stmt.initializer {
            self.resolve_expr(init);
        }
        self.define(stmt.token.clone());
    }

    fn visit_func_stmt(&mut self, stmt: &stmt::Func) -> Self::Output {
        self.declare(stmt.token.clone());
        self.define(stmt.token.clone());
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
            let error = RuntimeError::new(stmt.keyword.clone(),
                "Can't return from top-level code.");
            self.status.report_runtime_error(error);
        }
        if let Some(value) = &stmt.value {
            self.resolve_expr(value);
        }
    }

    fn visit_while_stmt(&mut self, stmt: &stmt::While) -> Self::Output {
        self.resolve_expr(&stmt.condition);
        self.resolve_stmt(&stmt.body);
    }

    fn visit_class_stmt(&mut self, stmt: &stmt::Class) -> Self::Output {
        self.declare(stmt.token.clone());
        self.define(stmt.token.clone());
    }
}

impl ExprVisitor for Resolver<'_> {
    type Output = ();

    fn visit_var_expr(&mut self, expr: &expr::Var) -> Self::Output {
        if self.is_accessed_in_initializer(expr.token.lexeme.clone()) {
            let error = ParseError::new(expr.token.clone(),
                "Can't read local variable in its own initializer."
            );
            self.status.report_compile_error(error);
        }
        self.resolve_local(expr.id, expr.token.clone());
    }

    fn visit_assign_expr(&mut self, expr: &expr::Assign) -> Self::Output {
        self.resolve_expr(&expr.value);
        self.resolve_local(expr.id, expr.token.clone());
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
}

enum FuncType {
    Function, 
    None,
}
