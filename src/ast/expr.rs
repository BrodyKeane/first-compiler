use std::rc::Rc;

use crate::token::{Token, Value};

pub trait AcceptExprVisitor {
    fn accept<V: ExprVisitor>(&self, visitor: &mut V) -> V::Output;
}

pub trait ExprVisitor {
    type Output;

    fn visit_binary_expr(&mut self, expr: &Binary) -> Self::Output;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> Self::Output;
    fn visit_literal_expr(&mut self, expr: &Literal) -> Self::Output;
    fn visit_unary_expr(&mut self, expr: &Unary) -> Self::Output;
    fn visit_var_expr(&mut self, expr: &Var) -> Self::Output;
    fn visit_assign_expr(&mut self, expr: &Assign) -> Self::Output;
    fn visit_logical_expr(&mut self, expr: &Logical) -> Self::Output;
    fn visit_call_expr(&mut self, expr: &Call) -> Self::Output;
//    fn visit_get_expr(&mut self, expr: &Get) -> Self::Output;
//    fn visit_set_expr(&mut self, expr: &Set) -> Self::Output;
//    fn visit_super_expr(&mut self, expr: &Super) -> Self::Output;
//    fn visit_this_expr(&mut self, expr: &This) -> Self::Output;
}


#[derive(Clone)]
pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
    Var(Var),
    Assign(Assign),
    Logical(Logical),
    Call(Call),
}

impl AcceptExprVisitor for Expr {
    fn accept<V: ExprVisitor>(&self, visitor: &mut V) -> V::Output {
        match self {
            Expr::Binary(expr) => visitor.visit_binary_expr(expr),
            Expr::Grouping(expr) => visitor.visit_grouping_expr(expr),
            Expr::Literal(expr) => visitor.visit_literal_expr(expr),
            Expr::Unary(expr) => visitor.visit_unary_expr(expr),
            Expr::Var(expr) => visitor.visit_var_expr(expr),
            Expr::Assign(expr) => visitor.visit_assign_expr(expr),
            Expr::Logical(expr) => visitor.visit_logical_expr(expr),
            Expr::Call(expr) => visitor.visit_call_expr(expr),
        }
    }
}

impl Expr {
    pub fn new_binary(left: Expr, operator: Rc<Token>, right: Expr) -> Self {
        Expr::Binary(Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }
    
    pub fn new_grouping(expr: Expr) -> Self {
        Expr::Grouping(Grouping {
            expr: Box::new(expr),
        })
    }

    pub fn new_literal(value: Rc<Value>) -> Self {
        Expr::Literal(Literal { value })
    }

    pub fn new_unary(operator: Rc<Token>, right: Expr) -> Self {
        Expr::Unary(Unary {
            operator,
            right: Box::new(right),
        })
    }

    pub fn new_var(name: Rc<Token>) -> Self {
        Expr::Var(Var{
            name
        })
    }

    pub fn new_assign(name: Rc<Token>, value: Expr) -> Self {
        Expr::Assign(Assign {
            name,
            value: Box::new(value),
        })
    }

    pub fn new_logical(left: Expr, operator: Rc<Token>, right: Expr) -> Self {
        Expr::Logical(Logical{
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    pub fn new_call(callee: Expr, paren: Rc<Token>, args: Vec<Expr>) -> Self {
        Expr::Call(Call{
            callee: Box::new(callee),
            paren,
            args,
        })
    }
}

#[derive(Clone)]
pub struct Binary {
  pub left: Box<Expr>,
  pub operator: Rc<Token>,
  pub right: Box<Expr>,
}

#[derive(Clone)]
pub struct Grouping {
    pub expr: Box<Expr>, 
}

#[derive(Clone)]
pub struct Literal {
    pub value: Rc<Value>,
}

#[derive(Clone)]
pub struct Unary {
    pub operator: Rc<Token>,
    pub right: Box<Expr>,
}

#[derive(Clone)]
pub struct Var {
    pub name: Rc<Token>,
}

#[derive(Clone)]
pub struct Assign {
    pub name: Rc<Token>,
    pub value: Box<Expr>,
}

#[derive(Clone)]
pub struct Logical {
  pub left: Box<Expr>,
  pub operator: Rc<Token>,
  pub right: Box<Expr>,
}

#[derive(Clone)]
pub struct Call {
    pub callee: Box<Expr>,
    pub paren: Rc<Token>,
    pub args: Vec<Expr>
}
