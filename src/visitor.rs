use crate::expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr};
use crate::token::DataType;
use std::rc::Rc;

pub trait Visitor {
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> DataType;
    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> DataType;
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> DataType;
    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> DataType;
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
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> DataType {
        if let Some(value) = expr.value.as_ref() {
            match value {
                DataType::String(s) => DataType::String(s.to_string()),
                DataType::Number(num) => DataType::String(num.to_string()),
                DataType::Bool(true) => DataType::String("true".to_string()),
                DataType::Bool(false) => DataType::String("false".to_string()),
                DataType::Nil => DataType::String("nil".to_string()),
            }
        } else {
            DataType::String("nil".to_string())
        }
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> DataType {
        self.parenthesize(&expr.operator.lexeme, vec![expr.right.as_ref()])
    }

    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> DataType {
        self.parenthesize(
            &expr.operator.lexeme,
            vec![expr.left.as_ref(), expr.right.as_ref()],
        )
    }

    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> DataType {
        self.parenthesize("group", vec![expr.expression.as_ref()])
    }
}
