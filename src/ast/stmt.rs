use std::rc::Rc;
use std::fmt;

use crate::{
    ast::expr::Expr,
    token::Token,
};

pub trait AcceptStmtVisitor {
    fn accept<V: StmtVisitor>(&self, visitor: &mut V) -> V::Output;
}

pub trait StmtVisitor {
    type Output;

    fn visit_expr_stmt(&mut self, stmt: &StmtExpr) -> Self::Output;
    fn visit_print_stmt(&mut self, stmt: &Print) -> Self::Output;
    fn visit_let_stmt(&mut self, stmt: &Let) -> Self::Output;
    fn visit_block_stmt(&mut self, stmt: &Block) -> Self::Output;
    fn visit_if_stmt(&mut self, stmt: &If) -> Self::Output;
    fn visit_while_stmt(&mut self, stmt: &While) -> Self::Output;
    fn visit_func_stmt(&mut self, stmt: &Func) -> Self::Output;
    fn visit_return_stmt(&mut self, stmt: &Return) -> Self::Output;
    fn visit_class_stmt(&mut self, stmt: &Class) -> Self::Output;
}

#[derive(Clone)]
pub enum Stmt{
    StmtExpr(StmtExpr),
    Print(Print),
    Let(Let),
    Block(Block),
    If(If),
    While(While),
    Func(Func),
    Return(Return),
    Class(Class),
}

impl AcceptStmtVisitor for Stmt {
    fn accept<V: StmtVisitor>(&self, visitor: &mut V) -> V::Output {
        match self {
            Stmt::StmtExpr(stmt) => visitor.visit_expr_stmt(stmt),
            Stmt::Print(stmt) => visitor.visit_print_stmt(stmt),
            Stmt::Let(stmt) => visitor.visit_let_stmt(stmt),
            Stmt::Block(stmt) => visitor.visit_block_stmt(stmt),
            Stmt::If(stmt) => visitor.visit_if_stmt(stmt),
            Stmt::While(stmt) => visitor.visit_while_stmt(stmt),
            Stmt::Func(stmt) => visitor.visit_func_stmt(stmt),
            Stmt::Return(stmt) => visitor.visit_return_stmt(stmt),
            Stmt::Class(stmt) => visitor.visit_class_stmt(stmt),
        }
    }
}

impl Stmt {
    pub fn new_stmt_expr(expr: Expr) -> Self {
        Self::StmtExpr(StmtExpr{ expr })
    }

    pub fn new_print(expr: Expr) -> Self {
        Self::Print(Print{ expr })
    }

    pub fn new_let(token: Rc<Token>, initializer: Option<Expr>) -> Self {
        Self::Let(Let{ token, initializer })
    }

    pub fn new_block(stmts: Vec<Stmt>) -> Self {
        Self::Block(Block{ stmts })
    }

    pub fn new_if(condition: Expr, body: Stmt, else_body: Option<Stmt>) -> Self {
        let else_body = else_body.map(|s| Box::new(s));
        Self::If(If{
            condition,
            body: Box::new(body),
            else_body
        })
    }

    pub fn new_while(condition: Expr, body: Stmt) -> Self {
        Self::While(While{
            condition,
            body: Box::new(body),
        })
    }

    pub fn new_func(token: Rc<Token>, params: Vec<Rc<Token>>, body: Vec<Stmt>) -> Self {
        Self::Func(Func { token, params, body })
    }

    pub fn new_return(keyword: Rc<Token>, value: Option<Expr>) -> Self {
        Self::Return(Return { keyword, value })
    }

    pub fn new_class(token: Rc<Token>, methods: Vec<Stmt>) -> Self {
        Self::Class(Class { token , methods })
    }
}

#[derive(Clone)]
pub struct StmtExpr {
    pub expr: Expr
}

#[derive(Clone)]
pub struct Print {
    pub expr: Expr
}

#[derive(Clone)]
pub struct Let {
    pub token: Rc<Token>,
    pub initializer: Option<Expr>,
}

#[derive(Clone)]
pub struct Block {
    pub stmts: Vec<Stmt>
}

#[derive(Clone)]
pub struct If {
    pub condition: Expr,
    pub body: Box<Stmt>,
    pub else_body: Option<Box<Stmt>>,
}

#[derive(Clone)]
pub struct While {
    pub condition: Expr,
    pub body: Box<Stmt>,
}

#[derive(Clone)]
pub struct Func {
    pub token: Rc<Token>,
    pub params: Vec<Rc<Token>>,
    pub body: Vec<Stmt>,
}

#[derive(Clone)]
pub struct Return {
    pub keyword: Rc<Token>,
    pub value: Option<Expr>,
}

#[derive(Clone)]
pub struct Class {
    pub token: Rc<Token>,
    pub methods: Vec<Stmt>,
}

impl fmt::Display for Func {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}

