use crate::{
    ast::{
        expr::{self, Expr},
        stmt,
    },
    environment::Environment,
    literal::Value,
    token::Token,
    token_type::TokenType,
};
use stmt::Stmt;

pub struct Interpreter<'i> {
    environment: Environment<'i>,
}

impl<'i> expr::Visitor<Result<Value, (Token, String)>> for Interpreter<'i> {
    fn visit_binary_expr(
        &mut self,
        a0: &Expr,
        a1: &Token,
        a2: &Expr,
    ) -> Result<Value, (Token, String)> {
        let left = self.evaluate(a0)?;
        let right = self.evaluate(a2)?;

        match a1.kind {
            TokenType::GREATER => {
                check_number_operands(a1, &left, &right)?;
                Ok(Value::Bool(left.unwrap_number() > right.unwrap_number()))
            }
            TokenType::GREATER_EQUAL => {
                check_number_operands(a1, &left, &right)?;
                Ok(Value::Bool(left.unwrap_number() >= right.unwrap_number()))
            }
            TokenType::LESS => {
                check_number_operands(a1, &left, &right)?;
                Ok(Value::Bool(left.unwrap_number() < right.unwrap_number()))
            }
            TokenType::LESS_EQUAL => {
                check_number_operands(a1, &left, &right)?;
                Ok(Value::Bool(left.unwrap_number() <= right.unwrap_number()))
            }
            TokenType::BANG_EQUAL => Ok(Value::Bool(!is_equal(left, right))),
            TokenType::EQUAL_EQUAL => Ok(Value::Bool(is_equal(left, right))),
            TokenType::MINUS => {
                check_number_operands(a1, &left, &right)?;
                Ok(Value::Number(left.unwrap_number() - right.unwrap_number()))
            }
            TokenType::SLASH => {
                check_number_operands(a1, &left, &right)?;
                Ok(Value::Number(left.unwrap_number() / right.unwrap_number()))
            }
            TokenType::STAR => {
                check_number_operands(a1, &left, &right)?;
                Ok(Value::Number(left.unwrap_number() * right.unwrap_number()))
            }
            TokenType::PLUS => {
                if left.is_number() && right.is_number() {
                    Ok(Value::Number(left.unwrap_number() + right.unwrap_number()))
                } else if left.is_string() && right.is_string() {
                    Ok(Value::String(
                        left.unwrap_string().to_string() + right.unwrap_string(),
                    ))
                } else {
                    Err((
                        a1.clone(),
                        "Operands must be two numbers or two strings.".to_string(),
                    ))
                }
            }
            _ => unreachable!(),
        }
    }
    fn visit_grouping_expr(&mut self, a0: &Expr) -> Result<Value, (Token, String)> {
        self.evaluate(a0)
    }
    fn visit_literal_expr(&mut self, a0: &Value) -> Result<Value, (Token, String)> {
        Ok(a0.clone())
    }
    fn visit_unary_expr(&mut self, a0: &Token, a1: &Expr) -> Result<Value, (Token, String)> {
        let right = self.evaluate(a1)?;
        match a0.kind {
            TokenType::MINUS => {
                check_number_operand(a0, &right)?;
                Ok(Value::Number(-right.unwrap_number()))
            }
            TokenType::BANG => Ok(Value::Bool(!is_truthy(right))),
            _ => unreachable!(),
        }
    }
    fn visit_assign_expr(&mut self, a0: &Token, a1: &Expr) -> Result<Value, (Token, String)> {
        let value = self.evaluate(a1)?;

        self.environment.assign(a0, &value)?;
        Ok(value)
    }
    fn visit_variable_expr(&mut self, a0: &Token) -> Result<Value, (Token, String)> {
        self.environment.get(a0)
    }
}

impl<'i> stmt::Visitor<Result<Value, (Token, String)>> for Interpreter<'i> {
    fn visit_expression_stmt(&mut self, a0: &Expr) -> Result<Value, (Token, String)> {
        self.evaluate(a0)
    }
    fn visit_print_stmt(&mut self, a0: &Expr) -> Result<Value, (Token, String)> {
        let value = self.evaluate(a0)?;
        println!("{}", value);
        Ok(Value::Nil)
    }
    fn visit_var_stmt(&mut self, a0: &Token, a1: &Option<Expr>) -> Result<Value, (Token, String)> {
        let mut value = Value::Nil;
        if let Some(initializer) = a1 {
            value = self.evaluate(initializer)?;
        }

        self.environment.define(&a0.lexeme, value);
        // returns nil here because assignment is a statement with no value
        Ok(Value::Nil)
    }
}

fn is_truthy(value: Value) -> bool {
    match value {
        Value::Nil => false,
        Value::Bool(b) => b,
        _ => true,
    }
}

fn is_equal(v: Value, w: Value) -> bool {
    if v.is_nil() && w.is_nil() {
        true
    } else if v.is_nil() {
        false
    } else if v.is_string() && w.is_string() {
        v.unwrap_string() == w.unwrap_string()
    } else if v.is_number() && w.is_number() {
        v.unwrap_number() - w.unwrap_number() < f64::EPSILON
    } else if v.is_bool() && w.is_bool() {
        v.unwrap_bool() == w.unwrap_bool()
    } else {
        false
    }
}

fn check_number_operand(operator: &Token, operand: &Value) -> Result<(), (Token, String)> {
    if operand.is_number() {
        Ok(())
    } else {
        Err((operator.clone(), "Operand must be a number.".to_string()))
    }
}

fn check_number_operands(operator: &Token, v: &Value, w: &Value) -> Result<(), (Token, String)> {
    if v.is_number() && w.is_number() {
        Ok(())
    } else {
        Err((operator.clone(), "Operand must be a number.".to_string()))
    }
}

impl<'i> Interpreter<'i> {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(None),
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, (Token, String)> {
        expr.accept(self)
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<Value, (Token, String)> {
        stmt.accept(self)
    }

    pub fn interpret(&mut self, statements: &[Stmt]) {
        for statement in statements {
            if let Err(e) = self.execute(statement) {
                crate::runtime_error(e);
                break;
            }
        }
    }
}

impl<'i> Default for Interpreter<'i> {
    fn default() -> Self {
        Self::new()
    }
}
