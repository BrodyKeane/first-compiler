enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

trait Visitor<R> {
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

struct Binary {
  left: Box<Expr>,
  operator: Token,
  right: Box<Expr>,
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

struct Grouping {
    expr: Box<Expr>, 
}

impl Grouping {
    fn new(expr: Expr) -> Grouping {
        Grouping {
            expr: Box::new(expr),
        }
    }
}

struct Literal {
    value: String,
}

impl Literal {
    fn new(value: String) -> Literal {
        Literal { value }
    }
}

struct Unary {
    operator: Token,
    right: Box<Expr>,
}

impl Unary {
    fn new(operator: Token, right: Expr) -> Unary {
        Unary {
            operator,
            right: Box::new(right),
        }
    }
}
