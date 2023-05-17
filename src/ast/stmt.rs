use crate::ast::expr;

pub trait AcceptStmtVisitor {
    fn accept<V: StmtVisitor>(&self, visitor: &mut V) -> V::Output;
}

pub trait StmtVisitor {
    type Output;

    fn visit_expression_stmt(&mut self, stmt: &StmtExpr) -> Self::Output;
    fn visit_print_stmt(&mut self, stmt: &Print) -> Self::Output;
//    fn visit_block_stmt(&mut self, stmt: &Block) -> Self::Output;
//    fn visit_function_stmt(&mut self, stmt: &Function) -> Self::Output;
//    fn visit_return_stmt(&mut self, stmt: &Return) -> Self::Output;
//    fn visit_class_stmt(&mut self, stmt: &Class) -> Self::Output;
//    fn visit_while_stmt(&mut self, stmt: &While) -> Self::Output;
//    fn visit_var_stmt(&mut self, stmt: &Var) -> Self::Output;
//    fn visit_if_stmt(&mut self, stmt: &If) -> Self::Output;
}

pub enum Stmt{
    StmtExpr(StmtExpr),
    Print(Print),
}

impl AcceptStmtVisitor for Stmt {
    fn accept<V: StmtVisitor>(&self, visitor: &mut V) -> V::Output {
        match self {
            Stmt::StmtExpr(stmt) => visitor.visit_expression_stmt(stmt),
            Stmt::Print(stmt) => visitor.visit_print_stmt(stmt),
        }
    }
}

impl Stmt {
    pub fn new_stmt_expr(expr: expr::Expr) -> Self {
        Self::StmtExpr(StmtExpr{expr})
    }

    pub fn new_print(expr: expr::Expr) -> Self {
        Self::Print(Print{expr})
    }
}

pub struct StmtExpr {
    pub expr: expr::Expr
}

pub struct Print {
    pub expr: expr::Expr
}
