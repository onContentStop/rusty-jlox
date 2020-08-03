use jlox::{ast::expr, token::Token};
use expr::Expr;
use std::{fmt::Display, sync::Arc};

pub struct AstPrinter {}
impl expr::Visitor<String> for AstPrinter {
    fn visit_binary_expr(&mut self, a0: &Expr, a1: &Token, a2: &Expr) -> String {
        self.parenthesize(&a1.lexeme, vec![a0, a2])
    }
    fn visit_grouping_expr(&mut self, a0: &Expr) -> String {
        self.parenthesize("group", vec![a0])
    }
    fn visit_literal_expr(&mut self, a0: &Option<Arc<dyn Display>>) -> String {
        match a0 {
            Some(value) => format!("{}", value),
            None => String::from("nil"),
        }
    }
    fn visit_unary_expr(&mut self, a0: &Token, a1: &Expr) -> String {
        self.parenthesize(&a0.lexeme, vec![a1])
    }
}

impl AstPrinter {
    fn parenthesize(&mut self, name: &str, exprs: Vec<&Expr>) -> String {
        let mut s = String::new();
        s.push('(');
        s.push_str(name);
        for expr in exprs {
            s.push(' ');
            s.push_str(&expr.accept(self));
        }
        s
    }
}

fn main() {
}
