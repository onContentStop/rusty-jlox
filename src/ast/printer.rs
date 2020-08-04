use super::expr;
use crate::{literal::Value, token::Token};
use expr::Expr;

pub struct AstPrinter {}
impl expr::Visitor<String> for AstPrinter {
    fn visit_binary_expr(&mut self, a0: &Expr, a1: &Token, a2: &Expr) -> String {
        self.parenthesize(&a1.lexeme, vec![a0, a2])
    }
    fn visit_grouping_expr(&mut self, a0: &Expr) -> String {
        self.parenthesize("group", vec![a0])
    }
    fn visit_literal_expr(&mut self, a0: &Value) -> String {
        match a0 {
            Value::Nil => String::from("nil"),
            Value::String(s) => s.to_string(),
            Value::Number(n) => format!("{}", n),
            Value::Bool(true) => String::from("true"),
            Value::Bool(false) => String::from("false"),
        }
    }
    fn visit_unary_expr(&mut self, a0: &Token, a1: &Expr) -> String {
        self.parenthesize(&a0.lexeme, vec![a1])
    }
    fn visit_variable_expr(&mut self, a0: &Token) -> String {
        format!("{}", a0.lexeme)
    }
    fn visit_assign_expr(&mut self, a0: &Token, a1: &Expr) -> String {
        todo!()
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
        s.push(')');
        s
    }

    pub fn print(&mut self, expr: Expr) -> String {
        expr.accept(self)
    }
}
