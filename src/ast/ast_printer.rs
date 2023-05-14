use crate::ast::expr::{self, Expr, Data};

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&mut self, expr: Expr) -> String{
        expr.accept(self)
    }

    pub fn parenthesize(&mut self, name: &str,
        exprs: Option<Vec<&Box<Expr>>>) -> String {
        let mut builder = String::new();
        builder += "(";
        builder += name;
        if let Some(exprs) = exprs {
            for expr in exprs {
                builder += " ";
                builder += &expr.accept(self);
            }
        }
        builder += ")";
        builder
    }
}

impl expr::Visitor for AstPrinter {
    type Output = String;

    fn visit_binary_expr(&mut self, expr: &expr::Binary) -> String {
        self.parenthesize(
            &expr.operator.lexeme,
            Some(vec![&expr.left, &expr.right]),
        )
    }

    fn visit_grouping_expr(&mut self, expr: &expr::Grouping) -> String {
        self.parenthesize("group", Some(vec![&expr.expr]))
    }

    fn visit_literal_expr(&mut self, expr: &expr::Literal) -> String {
        match &expr.value {
            Some(val) => val.to_string(),
            None => "nil".to_string(),
        }
    }

    fn visit_unary_expr(&mut self, expr: &expr::Unary) -> String {
        self.parenthesize(&expr.operator.lexeme, Some(vec![&expr.right]))
    }
}
