use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Literal {
    Nil,
    String(String),
    Number(f64),
    True,
    False,
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Nil => write!(f, ""),
            Literal::String(s) => write!(f, "{}", s),
            Literal::Number(n) => write!(f, "{}", n),
            Literal::True => write!(f, "true"),
            Literal::False => write!(f, "false"),
        }
    }
}
