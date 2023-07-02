use crate::expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr};
use crate::token::{DataType, TokenType};
use std::rc::Rc;
use anyhow::Result;
use anyhow::anyhow;

pub trait Visitor {
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Result<DataType>;
    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Result<DataType>;
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Result<DataType>;
    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> Result<DataType>;
}

pub struct AstPrinter {}

impl AstPrinter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn print(&mut self, expr: Rc<dyn Expr>) -> String {
        match expr.accept(self) {
            DataType::String(s) => s,
            _ => "Error".to_string(),
        }
    }

    fn parenthesize(&mut self, name: &str, exprs: Vec<&dyn Expr>) -> DataType {
        let mut s = String::new();
        s.push('(');
        s.push_str(name);
        for expr in exprs {
            s.push(' ');
            let expr_str = match expr.accept(self) {
                DataType::String(s) => s,
                _ => "Incorrect expression".to_string(),
            };
            s.push_str(expr_str.as_str());
        }
        s.push(')');
        DataType::String(s)
    }
}

impl Visitor for AstPrinter {
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Result<DataType> {
        if let Some(value) = expr.value.as_ref() {
            let result = match value {
                DataType::String(s) => DataType::String(s.to_string()),
                DataType::Number(num) => DataType::String(num.to_string()),
                DataType::Bool(true) => DataType::String("true".to_string()),
                DataType::Bool(false) => DataType::String("false".to_string()),
                DataType::Nil => DataType::String("nil".to_string()),
            };
            Ok(result)
        } else {
            Ok(DataType::String("nil".to_string()))
        }
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Result<DataType> {
        Ok(self.parenthesize(&expr.operator.lexeme, vec![expr.right.as_ref()]))
    }

    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Result<DataType> {
        Ok(self.parenthesize(
            &expr.operator.lexeme,
            vec![expr.left.as_ref(), expr.right.as_ref()],
        ))
    }

    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> Result<DataType> {
        Ok(self.parenthesize("group", vec![expr.expression.as_ref()]))
    }
}

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }
    pub fn evaluate(&mut self, expression: Rc<dyn Expr>) -> DataType {
        expression.accept(self)
    }
    fn is_truthy(&self, value: DataType) -> bool {
        match value {
            DataType::String(_) => true,
            DataType::Number(_) => true,
            DataType::Bool(_) => true,
            DataType::Nil => false
        }
    }

    fn is_equal(&self, left: DataType, right: DataType) -> bool {
        match (left, right) {
            (DataType::Nil, DataType::Nil) => true,
            (DataType::Nil, _) => false,
            (DataType::Bool(l), DataType::Bool(r)) => l == r,
            (DataType::Bool(_), _) => false,
            (DataType::Number(l), DataType::Number(r)) => l == r,
            (DataType::Number(_), _) => false,
            (DataType::String(l), DataType::String(r)) => l == r,
            (DataType::String(_), _) => false,
        }
    }
}

impl Visitor for Interpreter {
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Result<DataType> {
        match expr.value.as_ref() {
            None => Ok(DataType::Nil),
            Some(value) => Ok(value.clone())
        }
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Result<DataType> {
        let right = self.evaluate(Rc::clone(&expr.right));
        match expr.operator.token_type {
            TokenType::MINUS => {
                match right {
                    DataType::Number(s) => Ok(DataType::Number(-1f64 + s)),
                    _ => Err(anyhow!("Can only negate numbers"))
                }
            },
            TokenType::BANG => {
                let value =!self.is_truthy(right);
                Ok(DataType::Bool(value))
            },
            _ => Err(anyhow!("Can only negate numbers or truthy values"))
        }
    }

    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Result<DataType> {
        let left = self.evaluate(Rc::clone(&expr.left));
        let right = self.evaluate(Rc::clone(&expr.right));

        match expr.operator.token_type {
            TokenType::MINUS => {
                let left = match left {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!("Can only use - with numbers"))
                };
                let right = match right {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!(""))
                };
                Ok(DataType::Number(left - right))
            },
            TokenType::SLASH => {
                let left = match left {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!("Can only use / with numbers"))
                };
                let right = match right {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!(""))
                };
                Ok(DataType::Number(left/right))
            },
            TokenType::STAR => {
                let left = match left {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!("Can only use / with numbers"))
                };
                let right = match right {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!(""))
                };
                Ok(DataType::Number(left * right))
            },
            TokenType::PLUS => {
                let left = match left {
                    DataType::Number(_) | DataType::String(_) => left,
                    _ => return Err(anyhow!("Can only use * with numbers and strings"))
                };
                let right = match right {
                    DataType::Number(_) | DataType::String(_) => right,
                    _ => return Err(anyhow!(""))
                };

                match (left, right) {
                    (DataType::String(l), DataType::String(r)) => Ok(DataType::String(format!("{}{}", l, r))),
                    (DataType::Number(l), DataType::Number(r)) => Ok(DataType::Number(l + r)),
                    _ => Err(anyhow!("Both left and right should be number/string"))
                }
            },
            TokenType::GREATER => {
                let left = match left {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!("Can only use > with numbers"))
                };
                let right = match right {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!(""))
                };
                Ok(DataType::Bool(left > right))
            }
            TokenType::GREATEREQUAL => {
                let left = match left {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!("Can only use >= with numbers"))
                };
                let right = match right {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!(""))
                };
                Ok(DataType::Bool(left >= right))
            },
            TokenType::LESS => {
                let left = match left {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!("Can only use < with numbers"))
                };
                let right = match right {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!(""))
                };
                Ok(DataType::Bool(left < right))
            },
            TokenType::LESSEQUAL => {
                let left = match left {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!("Can only use <= with numbers"))
                };
                let right = match right {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!(""))
                };
                Ok(DataType::Bool(left <= right))
            }
            TokenType::BANGEQUAL => {
                Ok(DataType::Bool(!self.is_equal(left, right)))
            },
            TokenType::EQUALEQUAL => {
                Ok(DataType::Bool(self.is_equal(left, right)))
            }
            _ =>  Err(anyhow!("Unsupported operator"))
        }
    }

    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> Result<DataType> {
        Ok(self.evaluate(Rc::clone(&expr.expression)))
    }
}
