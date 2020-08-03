use crate::token_type::TokenType;
use std::{fmt::Display, sync::Arc};

/// A single token, taken straight out of the source code by the Scanner.
pub struct Token {
    /// The kind of token that this is.
    pub kind: TokenType,
    /// The source code text that corresponds to this Token.
    pub lexeme: String,
    /// Literals are anything printable. Could be a numeric value or the contents of a string.
    pub literal: Option<Arc<dyn Display>>,
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
            // not expensive due to this being Arc::clone() and not T::clone()
            literal: self.literal.clone(),
            line: self.line,
        }
    }
}

impl Token {
    pub fn new(
        kind: TokenType,
        lexeme: String,
        literal: Option<Arc<dyn Display>>,
        line: usize,
    ) -> Self {
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
            None => format!("{} {}", self.kind, self.lexeme),
            Some(literal) => format!("{} {} {}", self.kind, self.lexeme, literal),
        }
    }
}
