use crate::literal::Value;
use crate::token::Token;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Environment<'enc> {
    values: HashMap<String, Value>,
    enclosing: Option<&'enc mut Environment<'enc>>,
}

impl<'enc> Environment<'enc> {
    pub fn new(enclosing: Option<&'enc mut Environment<'enc>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing,
        }
    }
    pub fn define<S: Into<String>>(&mut self, name: S, value: Value) {
        self.values.insert(name.into(), value);
    }

    pub fn get(&mut self, name: &Token) -> Result<Value, (Token, String)> {
        if self.values.contains_key(&name.lexeme) {
            Ok(self.values.get(&name.lexeme).unwrap().clone())
        } else if let Some(ref mut enclosing) = self.enclosing {
            enclosing.get(name)
        } else {
            Err((
                name.clone(),
                format!("Undefined variable '{}'.", name.lexeme),
            ))
        }
    }

    pub fn assign(&mut self, name: &Token, value: &Value) -> Result<(), (Token, String)> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value.clone());
            Ok(())
        } else if let Some(ref mut enclosing) = self.enclosing {
            enclosing.assign(name, value)
        } else {
            Err((
                name.clone(),
                format!("Undefined variable '{}'.", name.lexeme),
            ))
        }
    }
}

impl<'enc> Default for Environment<'enc> {
    fn default() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }
}
