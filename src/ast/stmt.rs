use crate::ast::expr::Expr;
#[derive(Debug)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
}
impl Stmt {
    pub fn accept<R, V: Visitor<R>>(&self, visitor: &mut V) -> R {
        match self {
            Stmt::Expression(a0) => visitor.visit_expression_stmt(a0),
            Stmt::Print(a0) => visitor.visit_print_stmt(a0),
        }
    }
}
pub trait Visitor<R> {
    fn visit_expression_stmt(&mut self, a0: &Expr) -> R;
    fn visit_print_stmt(&mut self, a0: &Expr) -> R;
}
