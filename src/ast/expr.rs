use lazy_static::lazy_static;

use std::{
    rc::Rc,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
        RwLock,
    },
};

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
    fn visit_get_expr(&mut self, expr: &Get) -> Self::Output;
    fn visit_set_expr(&mut self, expr: &Set) -> Self::Output;
    fn visit_this_expr(&mut self, expr: &This) -> Self::Output;
//    fn visit_super_expr(&mut self, expr: &Super) -> Self::Output;
}

#[derive(Clone, Debug)]
pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
    Var(Var),
    Assign(Assign),
    Logical(Logical),
    Call(Call),
    Get(Get),
    Set(Set),
    This(This),
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
            Expr::Get(expr) => visitor.visit_get_expr(expr),
            Expr::Set(expr) => visitor.visit_set_expr(expr),
            Expr::This(expr) => visitor.visit_this_expr(expr),
        }
    }
}

impl Expr {
    pub fn new_binary(left: Expr, operator: Rc<Token>, right: Expr) -> Self {
        let id = ID_GENERATOR.generate_id();
        Expr::Binary(Binary {
            id,
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }
    
    pub fn new_grouping(expr: Expr) -> Self {
        let id = ID_GENERATOR.generate_id();
        Expr::Grouping(Grouping {
            id,
            expr: Box::new(expr),
        })
    }

    pub fn new_literal(value: Arc<RwLock<Value>>) -> Self {
        let id = ID_GENERATOR.generate_id();
        Expr::Literal(Literal { id, value })
    }

    pub fn new_unary(operator: Rc<Token>, right: Expr) -> Self {
        let id = ID_GENERATOR.generate_id();
        Expr::Unary(Unary {
            id,
            operator,
            right: Box::new(right),
        })
    }

    pub fn new_var(token: Rc<Token>) -> Self {
        let id = ID_GENERATOR.generate_id();
        Expr::Var(Var{
            id,
            token 
        })
    }

    pub fn new_assign(token: Rc<Token>, value: Expr) -> Self {
        let id = ID_GENERATOR.generate_id();
        Expr::Assign(Assign {
            id,
            token,
            value: Box::new(value),
        })
    }

    pub fn new_logical(left: Expr, operator: Rc<Token>, right: Expr) -> Self {
        let id = ID_GENERATOR.generate_id();
        Expr::Logical(Logical{
            id,
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    pub fn new_call(callee: Expr, paren: Rc<Token>, args: Vec<Expr>) -> Self {
        let id = ID_GENERATOR.generate_id();
        Expr::Call(Call{
            id,
            callee: Box::new(callee),
            paren,
            args,
        })
    }

    pub fn new_get(object: Expr, token: Rc<Token>) -> Self {
        let id = ID_GENERATOR.generate_id();
        Expr::Get(Get{
            id,
            object: Box::new(object),
            token,
        })
    }

    pub fn new_set(object: Expr, token: Rc<Token>, value: Expr) -> Self {
        let id = ID_GENERATOR.generate_id();
        Expr::Set(Set{
            id,
            object: Box::new(object),
            token,
            value: Box::new(value),
        })
    }

    pub fn new_this(keyword: Rc<Token>) -> Self {
        let id = ID_GENERATOR.generate_id();
        Expr::This(This { id, keyword } )
    }
}

lazy_static! {
    static ref ID_GENERATOR: IdGenerator = IdGenerator::new();
}

struct IdGenerator {
    next_id: AtomicU64,
}

impl IdGenerator {
    fn new() -> Self {
        IdGenerator {
            next_id: AtomicU64::new(0),
        }
    }

    fn generate_id(&self) -> u64 {
        self.next_id.fetch_add(1, Ordering::Relaxed)
    }
}


#[derive(Clone, Debug)]
pub struct Binary {
    pub id: u64,
    pub left: Box<Expr>,
    pub operator: Rc<Token>,
    pub right: Box<Expr>,
}

#[derive(Clone, Debug)]
pub struct Grouping {
    pub id: u64,
    pub expr: Box<Expr>, 
}

#[derive(Clone, Debug)]
pub struct Literal {
    pub id: u64,
    pub value: Arc<RwLock<Value>>,
}

#[derive(Clone, Debug)]
pub struct Unary {
    pub id: u64,
    pub operator: Rc<Token>,
    pub right: Box<Expr>,
}

#[derive(Clone, Debug)]
pub struct Var {
    pub id: u64,
    pub token: Rc<Token>,
}

#[derive(Clone, Debug)]
pub struct Assign {
    pub id: u64,
    pub token: Rc<Token>,
    pub value: Box<Expr>,
}

#[derive(Clone, Debug)]
pub struct Logical {
    pub id: u64,
    pub left: Box<Expr>,
    pub operator: Rc<Token>,
    pub right: Box<Expr>,
}

#[derive(Clone, Debug)]
pub struct Call {
    pub id: u64,
    pub callee: Box<Expr>,
    pub paren: Rc<Token>,
    pub args: Vec<Expr>
}

#[derive(Clone, Debug)]
pub struct Get {
    pub id: u64,
    pub object: Box<Expr>,
    pub token: Rc<Token>,
}

#[derive(Clone, Debug)]
pub struct Set {
    pub id: u64, 
    pub object:  Box<Expr>,
    pub token: Rc<Token>,
    pub value: Box<Expr>,
}

#[derive(Clone, Debug)]
pub struct This {
    pub id: u64,
    pub keyword: Rc<Token>,
}
