use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    String(String),
    Number(f64),
    Bool(bool),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, ""),
            Value::String(s) => write!(f, "{}", s),
            Value::Number(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", if *b { "true" } else { "false" }),
        }
    }
}

impl Value {
    pub fn unwrap_nil(&self) {
        match self {
            Value::Nil => {}
            _ => panic!("unwrap_nil called on non-nil value"),
        }
    }

    pub fn unwrap_string(&self) -> &str {
        match self {
            Value::String(s) => s,
            _ => panic!("unwrap_string called on non-string value"),
        }
    }

    pub fn unwrap_number(&self) -> f64 {
        match self {
            Value::Number(n) => *n,
            _ => panic!("unwrap_number called on non-number value"),
        }
    }

    pub fn unwrap_bool(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            _ => panic!("unwrap_bool called on non-bool value"),
        }
    }

    pub fn is_nil(&self) -> bool {
        match self {
            Value::Nil => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Value::String(_) => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Value::Number(_) => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            Value::Bool(_) => true,
            _ => false,
        }
    }
}
