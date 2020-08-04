use crate::{
    ast::{expr::Expr, stmt::Stmt},
    literal::Value,
    token::Token,
    token_type::TokenType,
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ()> {
        let mut statements: Vec<Stmt> = vec![];
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt, ()> {
        let res = {
            if self.matches(vec![TokenType::VAR]) {
                self.var_declaration()
            } else {
                self.statement()
            }
        };
        if res.is_ok() {
            res
        } else {
            self.synchronize();
            Err(())
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, ()> {
        let name = self.consume(TokenType::IDENTIFIER, "Expect variable name.")?;

        let mut initializer: Option<Expr> = None;
        if self.matches(vec![TokenType::EQUAL]) {
            initializer = Some(self.expression()?);
        }

        self.consume(
            TokenType::SEMICOLON,
            "Expect ';' after variable declaration.",
        )?;
        Ok(Stmt::Var(name, initializer))
    }

    fn statement(&mut self) -> Result<Stmt, ()> {
        if self.matches(vec![TokenType::PRINT]) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Result<Stmt, ()> {
        let value = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect ';' after value.")?;
        Ok(Stmt::Print(value))
    }

    fn expression_statement(&mut self) -> Result<Stmt, ()> {
        let expr = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect ';' after expression.")?;
        Ok(Stmt::Expression(expr))
    }

    fn expression(&mut self) -> Result<Expr, ()> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ()> {
        let expr = self.equality()?;

        if self.matches(vec![TokenType::EQUAL]) {
            let equals = self.previous();
            let value = self.assignment()?;

            if let Expr::Variable(name) = expr {
                return Ok(Expr::Assign(name, Box::new(value)));
            }

            crate::error_token(equals, "Invalid assignment target.");
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ()> {
        use TokenType::*;

        let mut expr = self.comparison()?;
        while self.matches(vec![BANG_EQUAL, EQUAL_EQUAL]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ()> {
        use TokenType::*;

        let mut expr = self.addition()?;

        while self.matches(vec![GREATER, GREATER_EQUAL, LESS, LESS_EQUAL]) {
            let operator = self.previous();
            let right = self.addition()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn addition(&mut self) -> Result<Expr, ()> {
        use TokenType::*;

        let mut expr = self.multiplication()?;

        while self.matches(vec![MINUS, PLUS]) {
            let operator = self.previous();
            let right = self.multiplication()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn multiplication(&mut self) -> Result<Expr, ()> {
        use TokenType::*;

        let mut expr = self.unary()?;

        while self.matches(vec![SLASH, STAR]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ()> {
        use TokenType::*;

        if self.matches(vec![BANG, MINUS]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary(operator, Box::new(right)));
        }

        Ok(self.primary()?)
    }

    fn primary(&mut self) -> Result<Expr, ()> {
        use TokenType::*;

        if self.matches(vec![FALSE]) {
            Ok(Expr::Literal(Value::Bool(false)))
        } else if self.matches(vec![TRUE]) {
            Ok(Expr::Literal(Value::Bool(true)))
        } else if self.matches(vec![NIL]) {
            Ok(Expr::Literal(Value::Nil))
        } else if self.matches(vec![NUMBER, STRING]) {
            Ok(Expr::Literal(self.previous().literal))
        } else if self.matches(vec![IDENTIFIER]) {
            Ok(Expr::Variable(self.previous()))
        } else if self.matches(vec![LEFT_PAREN]) {
            let expr = self.expression()?;
            self.consume(RIGHT_PAREN, "Expect ')' after expression.")?;
            Ok(Expr::Grouping(Box::new(expr)))
        } else {
            error(self.peek(), "Expect expression.");
            Err(())
        }
    }

    fn consume(&mut self, expected: TokenType, message: &str) -> Result<Token, ()> {
        if self.check(expected) {
            Ok(self.advance())
        } else {
            error(self.peek(), message);
            Err(())
        }
    }

    fn matches(&mut self, types: Vec<TokenType>) -> bool {
        for ty in types {
            if self.check(ty) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, ty: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().kind == ty
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenType::EOF
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().kind == TokenType::SEMICOLON {
                return;
            }
            match self.peek().kind {
                TokenType::CLASS
                | TokenType::FUN
                | TokenType::VAR
                | TokenType::FOR
                | TokenType::IF
                | TokenType::WHILE
                | TokenType::PRINT
                | TokenType::RETURN => return,
                _ => {}
            }
            self.advance();
        }
    }
}

fn error(token: Token, message: &str) {
    crate::error_token(token, message);
}
