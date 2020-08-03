use crate::literal::Value;
use crate::token::Token;
#[derive(Debug)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Value),
    Unary(Token, Box<Expr>),
}
impl Expr {
    pub fn accept<R, V: Visitor<R>>(&self, visitor: &mut V) -> R {
        match self {
            Expr::Binary(a0, a1, a2) => visitor.visit_binary_expr(a0, a1, a2),
            Expr::Grouping(a0) => visitor.visit_grouping_expr(a0),
            Expr::Literal(a0) => visitor.visit_literal_expr(a0),
            Expr::Unary(a0, a1) => visitor.visit_unary_expr(a0, a1),
        }
    }
}
pub trait Visitor<R> {
    fn visit_binary_expr(&mut self, a0: &Expr, a1: &Token, a2: &Expr) -> R;
    fn visit_grouping_expr(&mut self, a0: &Expr) -> R;
    fn visit_literal_expr(&mut self, a0: &Value) -> R;
    fn visit_unary_expr(&mut self, a0: &Token, a1: &Expr) -> R;
}
