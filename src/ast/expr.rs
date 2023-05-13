use crate::token::Token;

pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

pub trait Visitor<R> {
    fn visit_binary_expr(&mut self, expr: &Binary) -> R;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> R;
    fn visit_literal_expr(&mut self, expr: &Literal) -> R;
    fn visit_unary_expr(&mut self, expr: &Unary) -> R;
//    fn visit_assign_expr(&mut self, expr: &Assign) -> R;
//    fn visit_call_expr(&mut self, expr: &Call) -> R;
//    fn visit_get_expr(&mut self, expr: &Get) -> R;
//    fn visit_logical_expr(&mut self, expr: &Logical) -> R;
//    fn visit_set_expr(&mut self, expr: &Set) -> R;
//    fn visit_super_expr(&mut self, expr: &Super) -> R;
//    fn visit_this_expr(&mut self, expr: &This) -> R;
//    fn visit_variable_expr(&mut self, expr: &Variable) -> R;
}

impl Expr {
    pub fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        match self {
            Expr::Binary(expr) => visitor.visit_binary_expr(expr),
            Expr::Grouping(expr) => visitor.visit_grouping_expr(expr),
            Expr::Literal(expr) => visitor.visit_literal_expr(expr),
            Expr::Unary(expr) => visitor.visit_unary_expr(expr),
        }
    }
}

pub struct Binary {
  pub left: Box<Expr>,
  pub operator: Token,
  pub right: Box<Expr>,
}

impl Binary {
    fn new(left: Expr, operator: Token, right: Expr) -> Binary {
        Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

pub struct Grouping {
    pub expr: Box<Expr>, 
}

impl Grouping {
    fn new(expr: Expr) -> Grouping {
        Grouping {
            expr: Box::new(expr),
        }
    }
}

pub struct Literal {
    pub value: Option<String>,
}

impl Literal {
    fn new(value: Option<String>) -> Literal {
        Literal { value }
    }
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Unary {
    fn new(operator: Token, right: Expr) -> Unary {
        Unary {
            operator,
            right: Box::new(right),
        }
    }
}
