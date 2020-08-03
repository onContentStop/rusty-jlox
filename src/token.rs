use crate::{literal::Value, token_type::TokenType};
use std::fmt::Debug;

/// A single token, taken straight out of the source code by the Scanner.
#[derive(Debug)]
pub struct Token {
    /// The kind of token that this is.
    pub kind: TokenType,
    /// The source code text that corresponds to this Token.
    pub lexeme: String,
    /// Literals are anything printable. Could be a numeric value or the contents of a string.
    pub literal: Value,
    /// The line that this token was found on in the source code
    pub line: usize,
}

// yes, they're copyable! it might cost you precious CPU cycles though...
impl Clone for Token {
    fn clone(&self) -> Self {
        Self {
            kind: self.kind,
            // possibly expensive for long string tokens and the like
            lexeme: self.lexeme.clone(),
            literal: self.literal.clone(),
            line: self.line,
        }
    }
}

impl Token {
    pub fn new(kind: TokenType, lexeme: String, literal: Value, line: usize) -> Self {
        Self {
            kind,
            lexeme,
            literal,
            line,
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match &self.literal {
            Value::Nil => format!("{} {}", self.kind, self.lexeme),
            _ => format!("{} {} {}", self.kind, self.lexeme, self.literal),
        }
    }
}
