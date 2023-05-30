use std::collections::hash_map::HashMap;
use std::rc::Rc;

use crate::{
    interpreter::Interpreter,
    ast::{
        expr::{self, ExprVisitor, Expr, AcceptExprVisitor},
        stmt::{self, StmtVisitor, Stmt, AcceptStmtVisitor, Func},
    },
    token::Token,
    error::{ErrorStatus, ParseError},
};

struct Resolver<'a> {
    interpreter: Interpreter,
    scopes: Vec<HashMap<String, bool>>, //bool represent weather the entry has been resolved yet.
    status: &'a mut ErrorStatus,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: Interpreter, status: &'a mut ErrorStatus) -> Self {
        Resolver {
            interpreter,
            scopes: Vec::new(),
            status
        } 
    }

    fn resolve_stmts(&mut self, stmts: Vec<Stmt>) -> Result<(), ParseError> {
        for stmt in stmts {
            self.resolve_stmt(stmt);
        }
        Ok(())
    }

    fn resolve_stmt(&mut self, stmt: Stmt) {
        stmt.accept(self)
    }

    fn resolve_expr(&mut self, expr: Expr) {
        expr.accept(self)
    }
    
    fn resolve_local(&mut self, expr: Expr, token: Rc<Token>) {
        for (i, scope) in self.scopes.iter().enumerate().rev() {
            if scope.contains_key(&token.lexeme) {
                self.interpreter.resolve(expr, i);
            }
        }
    }

    fn resolve_func(&mut self, func: Func) {
        self.begin_scope();
        for param in func.params {
            self.declare(param);
            self.define(param);
        }
        self.resolve_stmts(func.body);
        self.end_scope();
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new())
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, token: Rc<Token>) {
        match self.scopes.last() {
            Some(scope) => scope.insert(token.lexeme, false),
            None => return
        };
    }

    fn define(&mut self, token: Rc<Token>) {
        match self.scopes.last() {
            Some(scope) => scope.insert(token.lexeme, true),
            None => return
        };
    }

    

    fn is_resolved(&self, name: String) -> bool {
        let scope = match self.scopes.last() {
            Some(scope) => scope,
            None => return false,
        };
        match scope.get(&name) {
            Some(name) => *name,
            None => false,
        }
    }
}

impl StmtVisitor for Resolver<'_> {
    type Output = ();

    fn visit_block_stmt(&mut self, stmt: &stmt::Block) -> Self::Output {
        self.begin_scope();
        self.resolve_stmts(stmt.stmts);
        self.end_scope();
    }

    fn visit_let_stmt(&mut self, stmt: &stmt::Let) -> Self::Output {
        self.declare(stmt.token);
        if let Some(init) = stmt.initializer {
            self.resolve_expr(init);
        }
        self.define(stmt.token);
    }

    fn visit_func_stmt(&mut self, stmt: &stmt::Func) -> Self::Output {
        self.declare(stmt.token);
        self.define(stmt.token);
        self.resolve_func(*stmt);
    }

    fn visit_expr_stmt(&mut self, stmt: &stmt::StmtExpr) -> Self::Output {
       self.resolve_expr(stmt.expr);
    }

    fn visit_if_stmt(&mut self, stmt: &stmt::If) -> Self::Output {
        self.resolve_expr(stmt.condition);
        self.resolve_stmt(*stmt.body);
        if let Some(else_body) = stmt.else_body {
            self.resolve_stmt(*else_body);
        }
    }

    fn visit_print_stmt(&mut self, stmt: &stmt::Print) -> Self::Output {
        self.resolve_expr(stmt.expr);
    }

    fn visit_return_stmt(&mut self, stmt: &stmt::Return) -> Self::Output {
        if let Some(value) = stmt.value {
            self.resolve_expr(value);
        }
    }

    fn visit_while_stmt(&mut self, stmt: &stmt::While) -> Self::Output {
        self.resolve_expr(stmt.condition);
        self.resolve_stmt(*stmt.body);
    }
}

impl ExprVisitor for Resolver<'_> {
    type Output = ();

    fn visit_var_expr(&mut self, expr: &expr::Var) -> Self::Output {
        if !self.scopes.is_empty() && !self.is_resolved(expr.token.lexeme) {
            let error = ParseError::new(expr.token,
                "Can't read local variable in its own initializer."
            );
            self.status.report_compile_error(error);
        }
        self.resolve_local(Expr::Var(*expr), expr.token);
    }

    fn visit_assign_expr(&mut self, expr: &expr::Assign) -> Self::Output {
        self.resolve_expr(*expr.value);
        self.resolve_local(Expr::Assign(*expr), expr.token);
    }

    fn visit_binary_expr(&mut self, expr: &expr::Binary) -> Self::Output {
        self.resolve_expr(*expr.left);
        self.resolve_expr(*expr.right);
    }
    
    fn visit_call_expr(&mut self, expr: &expr::Call) -> Self::Output {
        self.resolve_expr(*expr.callee);
        for arg in expr.args {
            self.resolve_expr(arg);
        }
    }

    fn visit_grouping_expr(&mut self, expr: &expr::Grouping) -> Self::Output {
        self.resolve_expr(*expr.expr);
    }

    fn visit_literal_expr(&mut self, expr: &expr::Literal) -> Self::Output {}

    fn visit_logical_expr(&mut self, expr: &expr::Logical) -> Self::Output {
        self.resolve_expr(*expr.left);
        self.resolve_expr(*expr.right);
    }

    fn visit_unary_expr(&mut self, expr: &expr::Unary) -> Self::Output {
        self.resolve_expr(*expr.right);
    }
}
