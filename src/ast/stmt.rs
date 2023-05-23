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
//    fn visit_function_stmt(&mut self, stmt: &Function) -> Self::Output;
//    fn visit_return_stmt(&mut self, stmt: &Return) -> Self::Output;
//    fn visit_class_stmt(&mut self, stmt: &Class) -> Self::Output;
//    fn visit_while_stmt(&mut self, stmt: &While) -> Self::Output;
}

pub enum Stmt{
    StmtExpr(StmtExpr),
    Print(Print),
    Let(Let),
    Block(Block),
    If(If),
}

impl AcceptStmtVisitor for Stmt {
    fn accept<V: StmtVisitor>(&self, visitor: &mut V) -> V::Output {
        match self {
            Stmt::StmtExpr(stmt) => visitor.visit_expression_stmt(stmt),
            Stmt::Print(stmt) => visitor.visit_print_stmt(stmt),
            Stmt::Let(stmt) => visitor.visit_let_stmt(stmt),
            Stmt::Block(stmt) => visitor.visit_block_stmt(stmt),
            Stmt::If(stmt) => visitor.visit_if_stmt(stmt),
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

    pub fn new_let(name: Token, initializer: Option<Expr>) -> Self {
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
}

pub struct StmtExpr {
    pub expr: Expr
}

pub struct Print {
    pub expr: Expr
}

pub struct Let {
    pub name: Token,
    pub initializer: Option<Expr>,
}

pub struct Block {
    pub stmts: Vec<Stmt>
}

pub struct If {
    pub condition: Expr,
    pub body: Box<Stmt>,
    pub else_body: Box<Option<Stmt>>,
}
