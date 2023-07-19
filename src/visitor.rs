use std::cell::RefCell;
use std::rc::Rc;

use anyhow::anyhow;
use anyhow::Result;

use crate::environment::Environment;
use crate::expr::{AssignExpr, BinaryExpr, Expr, GroupingExpr, LiteralExpr, LogicalExpr, UnaryExpr, VarExpr};
use crate::stmt::{BlockStmt, ExprStmt, IfStmt, PrintStmt, Stmt, VarStmt, WhileStmt};
use crate::token::{DataType, TokenType};
use crate::token::TokenType::OR;

pub trait Visitor {
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Result<DataType>;
    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Result<DataType>;
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Result<DataType>;
    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> Result<DataType>;
    fn visit_var_expr(&mut self, expr: &VarExpr) -> Result<DataType>;
    fn visit_assign_expr(&mut self, expr: &AssignExpr) -> Result<DataType>;
    fn visit_logical_expr(&mut self, expr: &LogicalExpr) -> Result<DataType>;
}

pub trait StmtVisitor {
    fn visit_print_statement(&mut self, stmt: &PrintStmt) -> Result<()>;
    fn visit_expr_statement(&mut self, stmt: &ExprStmt) -> Result<()>;
    fn visit_var_statement(&mut self, stmt: &VarStmt) -> Result<()>;
    fn visit_block_statement(&mut self, stmt: &BlockStmt) -> Result<()>;
    fn visit_if_statement(&mut self, stmt: &IfStmt) -> Result<()>;
    fn visit_while_statement(&mut self, stmt: &WhileStmt) -> Result<()>;
}

pub struct AstPrinter {}

#[allow(dead_code)]
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

    fn visit_var_expr(&mut self, _expr: &VarExpr) -> Result<DataType> {
        todo!()
    }

    fn visit_assign_expr(&mut self, _expr: &AssignExpr) -> Result<DataType> {
        todo!()
    }

    fn visit_logical_expr(&mut self, _expr: &LogicalExpr) -> Result<DataType> {
        todo!()
    }
}

pub struct Interpreter {
    pub environment: RefCell<Rc<RefCell<Environment>>>,
}

impl Interpreter {
    pub fn new(environment: RefCell<Rc<RefCell<Environment>>>) -> Self {
        Self { environment }
    }

    pub fn interpret(&mut self, statements: Vec<Rc<dyn Stmt>>) -> Result<()> {
        for statement in statements {
            self.execute(statement)?
        }
        Ok(())
    }

    fn execute_block(
        &mut self,
        statements: Vec<Rc<dyn Stmt>>,
        block_environment: Environment,
    ) -> Result<()> {
        let previous = self.environment.replace(Rc::new(RefCell::new(block_environment)));
        for statement in statements {
            self.execute(statement)?;
        }
        self.environment.replace(previous);
        Ok(())
    }

    fn evaluate(&mut self, expression: Rc<dyn Expr>) -> DataType {
        expression.accept(self)
    }

    fn execute(&mut self, statement: Rc<dyn Stmt>) -> Result<()> {
        statement.accept(self)
    }

    fn is_truthy(&self, value: &DataType) -> bool {
        match value {
            DataType::String(_) => true,
            DataType::Number(_) => true,
            DataType::Bool(_) => true,
            DataType::Nil => false,
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
            Some(value) => Ok(value.clone()),
        }
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Result<DataType> {
        let right = self.evaluate(Rc::clone(&expr.right));
        match expr.operator.token_type {
            TokenType::MINUS => match right {
                DataType::Number(s) => Ok(DataType::Number(-1f64 + s)),
                _ => Err(anyhow!("Can only negate numbers")),
            },
            TokenType::BANG => {
                let value = !self.is_truthy(&right);
                Ok(DataType::Bool(value))
            }
            _ => Err(anyhow!("Can only negate numbers or truthy values")),
        }
    }

    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Result<DataType> {
        let left = self.evaluate(Rc::clone(&expr.left));
        let right = self.evaluate(Rc::clone(&expr.right));

        match expr.operator.token_type {
            TokenType::MINUS => {
                let left = match left {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!("Can only use - with numbers")),
                };
                let right = match right {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!("")),
                };
                Ok(DataType::Number(left - right))
            }
            TokenType::SLASH => {
                let left = match left {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!("Can only use / with numbers")),
                };
                let right = match right {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!("")),
                };
                Ok(DataType::Number(left / right))
            }
            TokenType::STAR => {
                let left = match left {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!("Can only use / with numbers")),
                };
                let right = match right {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!("")),
                };
                Ok(DataType::Number(left * right))
            }
            TokenType::PLUS => {
                let left = match left {
                    DataType::Number(_) | DataType::String(_) => left,
                    _ => return Err(anyhow!("Can only use * with numbers and strings")),
                };
                let right = match right {
                    DataType::Number(_) | DataType::String(_) => right,
                    _ => return Err(anyhow!("")),
                };

                match (left, right) {
                    (DataType::String(l), DataType::String(r)) => {
                        Ok(DataType::String(format!("{}{}", l, r)))
                    }
                    (DataType::Number(l), DataType::Number(r)) => {
                        Ok(DataType::Number(l + r))
                    },
                    _ => Err(anyhow!("Both left and right should be number/string")),
                }
            }
            TokenType::GREATER => {
                let left = match left {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!("Can only use > with numbers")),
                };
                let right = match right {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!("")),
                };
                Ok(DataType::Bool(left > right))
            }
            TokenType::GREATEREQUAL => {
                let left = match left {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!("Can only use >= with numbers")),
                };
                let right = match right {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!("")),
                };
                Ok(DataType::Bool(left >= right))
            }
            TokenType::LESS => {
                let left = match left {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!("Can only use < with numbers")),
                };
                let right = match right {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!("")),
                };
                Ok(DataType::Bool(left < right))
            }
            TokenType::LESSEQUAL => {
                let left = match left {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!("Can only use <= with numbers")),
                };
                let right = match right {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!("")),
                };
                Ok(DataType::Bool(left <= right))
            }
            TokenType::BANGEQUAL => Ok(DataType::Bool(!self.is_equal(left, right))),
            TokenType::EQUALEQUAL => Ok(DataType::Bool(self.is_equal(left, right))),
            _ => Err(anyhow!("Unsupported operator")),
        }
    }

    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> Result<DataType> {
        Ok(self.evaluate(Rc::clone(&expr.expression)))
    }

    fn visit_var_expr(&mut self, expr: &VarExpr) -> Result<DataType> {
        self.environment
            .borrow()
            .borrow()
            .get(&expr.var_name.lexeme)
            .ok_or(anyhow!("var does not exist"))
    }

    fn visit_assign_expr(&mut self, expr: &AssignExpr) -> Result<DataType> {
        let value = self.evaluate(Rc::clone(expr.var_value.as_ref().unwrap()));
        self.environment
            .borrow()
            .borrow_mut()
            .assign(expr.var_name.lexeme.clone(), Some(value.clone()))?;
        Ok(value)
    }

    fn visit_logical_expr(&mut self, expr: &LogicalExpr) -> Result<DataType> {
        let left = self.evaluate(Rc::clone(&expr.left));
        if expr.operator.token_type == OR {
            if self.is_truthy(&left) {
                return Ok(left);
            }
        } else if !self.is_truthy(&left) {
            return Ok(left);
        }

        Ok(self.evaluate(Rc::clone(&expr.right)))
    }
}

impl StmtVisitor for Interpreter {
    fn visit_print_statement(&mut self, stmt: &PrintStmt) -> Result<()> {
        let value = self.evaluate(Rc::clone(&stmt.expression));
        println!("{}", value.to_string());
        Ok(())
    }

    fn visit_expr_statement(&mut self, stmt: &ExprStmt) -> Result<()> {
        self.evaluate(Rc::clone(&stmt.expression));
        Ok(())
    }

    fn visit_var_statement(&mut self, stmt: &VarStmt) -> Result<()> {
        match stmt.var_value.as_ref() {
            None => self.environment.borrow().borrow_mut().define(stmt.var_name.lexeme.clone(), None),
            Some(stmt_line) => {
                let value = self.evaluate(stmt_line.clone());
                self.environment
                    .borrow()
                    .borrow_mut()
                    .define(stmt.var_name.lexeme.clone(), Some(value))
            }
        }
        Ok(())
    }

    fn visit_block_statement(&mut self, stmt: &BlockStmt) -> Result<()> {
        let env = Environment::new_with_parent_environment(self.environment.borrow().clone());
        self.execute_block(
            stmt.statements.clone(),
            env,
        )
    }

    fn visit_if_statement(&mut self, stmt: &IfStmt) -> Result<()> {
        let condition = self.evaluate(Rc::clone(&stmt.condition));
        match condition {
            DataType::Bool(value) => {
                if value {
                    self.execute(Rc::clone(&stmt.then_branch))?;
                } else if let Some(else_branch) = stmt.else_branch.as_ref() {
                    self.execute(Rc::clone(else_branch))?
                }
            }
            _ => Err(anyhow!("condition not boolean"))?,
        };
        Ok(())
    }

    fn visit_while_statement(&mut self, stmt: &WhileStmt) -> Result<()> {
        let mut condition = true;

        while condition {
            condition = match &self.evaluate(Rc::clone(&stmt.condition)) {
                DataType::Bool(true_value) => *true_value,
                _ => return Err(anyhow!("condition should be boolean"))
            };

            if condition {
                self.execute(Rc::clone(&stmt.body))?;
            }
        }

        Ok(())
    }
}
