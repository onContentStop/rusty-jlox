use crate::{literal::Literal, token::Token, token_type::TokenType};

fn keyword(text: &str) -> Option<TokenType> {
    match text {
        "and" => Some(TokenType::AND),
        "class" => Some(TokenType::CLASS),
        "else" => Some(TokenType::ELSE),
        "false" => Some(TokenType::FALSE),
        "for" => Some(TokenType::FOR),
        "fun" => Some(TokenType::FUN),
        "if" => Some(TokenType::IF),
        "nil" => Some(TokenType::NIL),
        "or" => Some(TokenType::OR),
        "print" => Some(TokenType::PRINT),
        "return" => Some(TokenType::RETURN),
        "super" => Some(TokenType::SUPER),
        "this" => Some(TokenType::THIS),
        "true" => Some(TokenType::TRUE),
        "var" => Some(TokenType::VAR),
        "while" => Some(TokenType::WHILE),
        _ => None,
    }
}

/// The Lox lexer!
pub struct Scanner {
    /// the source code, split into chars for easy Unicode handling 🕶
    source: Vec<char>,
    /// the parsed tokens
    tokens: Vec<Token>,

    /// where the current source code token started
    start: usize,
    /// where the lexer is right now
    current: usize,
    line: usize,
}

impl Scanner {
    /// Create a lexer for this source code. Call `scan_tokens` after this if you want the lexer to run.
    pub fn new<S>(source: S) -> Self
    where
        S: AsRef<str>,
    {
        Self {
            source: source.as_ref().chars().collect(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    /// "Do your job!" Returns the tokens it lexed.
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            Literal::Nil,
            self.line,
        ));
        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// Scan one token. This is the meat of the lexer.
    fn scan_token(&mut self) {
        use TokenType::*;
        match self.advance() {
            // basic tokens
            '(' => self.add_token(LEFT_PAREN, Literal::Nil),
            ')' => self.add_token(RIGHT_PAREN, Literal::Nil),
            '{' => self.add_token(LEFT_BRACE, Literal::Nil),
            '}' => self.add_token(RIGHT_BRACE, Literal::Nil),
            ',' => self.add_token(COMMA, Literal::Nil),
            '.' => self.add_token(DOT, Literal::Nil),
            '-' => self.add_token(MINUS, Literal::Nil),
            '+' => self.add_token(PLUS, Literal::Nil),
            ';' => self.add_token(SEMICOLON, Literal::Nil),
            '*' => self.add_token(STAR, Literal::Nil),

            // two-char tokens like <=
            '!' => {
                let kind = if self.matches('=') { BANG_EQUAL } else { BANG };
                self.add_token(kind, Literal::Nil);
            }
            '=' => {
                let kind = if self.matches('=') {
                    EQUAL_EQUAL
                } else {
                    EQUAL
                };
                self.add_token(kind, Literal::Nil);
            }
            '<' => {
                let kind = if self.matches('=') { LESS_EQUAL } else { LESS };
                self.add_token(kind, Literal::Nil);
            }
            '>' => {
                let kind = if self.matches('=') {
                    GREATER_EQUAL
                } else {
                    GREATER
                };
                self.add_token(kind, Literal::Nil);
            }

            // slash token, or line comment? you decide, gamers
            '/' => {
                if self.matches('/') {
                    // line comment
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(SLASH, Literal::Nil);
                }
            }

            // inferior characters, they don't even count LUL
            ' ' | '\r' | '\t' => {}
            '\n' => {
                // well, this one kind of does stuff
                self.line += 1;
            }

            // special shit like strings and other literals
            '"' => self.string(),
            c if c.is_numeric() => self.number(),
            c if c.is_alphabetic() => self.identifier(),

            // oops!
            _ => crate::error(self.line, "Unexpected character."),
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }
        let text = self.source[self.start..self.current]
            .iter()
            .collect::<String>();
        let kind = keyword(&text).unwrap_or(TokenType::IDENTIFIER);
        self.add_token(kind, Literal::Nil);
    }

    fn number(&mut self) {
        while self.peek().is_numeric() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_numeric() {
            // floating-point literal

            // consume the dot
            self.advance();
            // more numbers!
            while self.peek().is_numeric() {
                self.advance();
            }
        }

        self.add_token(
            TokenType::NUMBER,
            Literal::Number(
                self.source[self.start..self.current]
                    .iter()
                    .collect::<String>()
                    .parse::<f64>()
                    // it'll parse for sure, we already validated it
                    .expect("Oh god oh fuck"),
            ),
        );
    }

    fn string(&mut self) {
        // look for closing "
        while self.peek() != '"' && !self.is_at_end() {
            // yeah, still keeping track of line #'s here
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        // you forgot the closing "
        if self.is_at_end() {
            crate::error(self.line, "Unterminated string.");
            return;
        }

        self.advance();

        // the literal here is just the source code with quotes stripped
        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token(TokenType::STRING, Literal::String(value));
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    /// Have a look at the next character without consuming it
    fn peek(&self) -> char {
        if self.is_at_end() {
            // dumy
            '\0'
        } else {
            self.source[self.current]
        }
    }

    /// Have a look at the next character. If it matches the one provided, consume it.
    fn matches(&mut self, c: char) -> bool {
        if self.is_at_end() || self.source[self.current] != c {
            false
        } else {
            // c o n s u m e
            self.current += 1;
            true
        }
    }

    /// Eat one character up. Yum.
    fn advance(&mut self) -> char {
        // c h o n c h
        self.current += 1;
        self.source[self.current - 1]
    }

    /// We finished parsing a token!
    fn add_token(&mut self, kind: TokenType, literal: Literal) {
        let text: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token::new(kind, text, literal, self.line));
    }
}
