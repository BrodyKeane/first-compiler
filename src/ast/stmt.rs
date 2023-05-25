use std::rc::Rc;

use crate::{
    ast::expr::Expr,
    token::Token,
};


pub trait AcceptStmtVisitor {
    fn accept<V: StmtVisitor>(&self, visitor: &mut V) -> V::Output;
}

pub trait StmtVisitor {
    type Output;

    fn visit_expression_stmt(&mut self, stmt: &StmtExpr) -> Self::Output;
    fn visit_print_stmt(&mut self, stmt: &Print) -> Self::Output;
    fn visit_let_stmt(&mut self, stmt: &Let) -> Self::Output;
    fn visit_block_stmt(&mut self, stmt: &Block) -> Self::Output;
    fn visit_if_stmt(&mut self, stmt: &If) -> Self::Output;
    fn visit_while_stmt(&mut self, stmt: &While) -> Self::Output;
    fn visit_func_stmt(&mut self, stmt: &Func) -> Self::Output;
    fn visit_return_stmt(&mut self, stmt: &Return) -> Self::Output;
//    fn visit_class_stmt(&mut self, stmt: &Class) -> Self::Output;
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
}

impl AcceptStmtVisitor for Stmt {
    fn accept<V: StmtVisitor>(&self, visitor: &mut V) -> V::Output {
        match self {
            Stmt::StmtExpr(stmt) => visitor.visit_expression_stmt(stmt),
            Stmt::Print(stmt) => visitor.visit_print_stmt(stmt),
            Stmt::Let(stmt) => visitor.visit_let_stmt(stmt),
            Stmt::Block(stmt) => visitor.visit_block_stmt(stmt),
            Stmt::If(stmt) => visitor.visit_if_stmt(stmt),
            Stmt::While(stmt) => visitor.visit_while_stmt(stmt),
            Stmt::Func(stmt) => visitor.visit_func_stmt(stmt),
            Stmt::Return(stmt) => visitor.visit_return_stmt(stmt),
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

    pub fn new_let(name: Rc<Token>, initializer: Option<Expr>) -> Self {
        Self::Let(Let{ name, initializer })
    }

    pub fn new_block(stmts: Vec<Stmt>) -> Self {
        Self::Block(Block{ stmts })
    }

    pub fn new_if(condition: Expr, body: Stmt, else_body: Option<Stmt>) -> Self {
        Self::If(If{
            condition,
            body: Box::new(body),
            else_body: Box::new(else_body)
        })
    }

    pub fn new_while(condition: Expr, body: Stmt) -> Self {
        Self::While(While{
            condition,
            body: Box::new(body),
        })
    }

    pub fn new_func(name: Rc<Token>, params: Vec<Rc<Token>>, body: Vec<Stmt>) -> Self {
        Self::Func(Func { name, params, body })
    }

    pub fn new_return(keyword: Rc<Token>, value: Expr) -> Self {
        Self::Return(Return { keyword, value })
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
    pub name: Rc<Token>,
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
    pub else_body: Box<Option<Stmt>>,
}

#[derive(Clone)]
pub struct While {
    pub condition: Expr,
    pub body: Box<Stmt>,
}

#[derive(Clone)]
pub struct Func {
    pub name: Rc<Token>,
    pub params: Vec<Rc<Token>>,
    pub body: Vec<Stmt>,
}

#[derive(Clone)]
pub struct Return {
    pub keyword: Rc<Token>,
    pub value: Expr,
}
