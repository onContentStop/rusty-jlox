use crate::ast::expr::Expr;
use crate::token::Token;
#[derive(Debug)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var(Token, Option<Expr>),
}
impl Stmt {
    pub fn accept<R, V: Visitor<R>>(&self, visitor: &mut V) -> R {
        match self {
            Stmt::Expression(a0) => visitor.visit_expression_stmt(a0),
            Stmt::Print(a0) => visitor.visit_print_stmt(a0),
            Stmt::Var(a0, a1) => visitor.visit_var_stmt(a0, a1),
        }
    }
}
pub trait Visitor<R> {
    fn visit_expression_stmt(&mut self, a0: &Expr) -> R;
    fn visit_print_stmt(&mut self, a0: &Expr) -> R;
    fn visit_var_stmt(&mut self, a0: &Token, a1: &Option<Expr>) -> R;
}
