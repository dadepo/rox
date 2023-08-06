use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use anyhow::anyhow;
use crate::expr::{AssignExpr, BinaryExpr, CallExpr, Expr, GroupingExpr, LiteralExpr, LogicalExpr, UnaryExpr, VarExpr};
use crate::interpreter::Interpreter;
use crate::stmt::{BlockStmt, ExprStmt, FunctionStmt, IfStmt, PrintStmt, ReturnStmt, VarStmt, WhileStmt};
use crate::token::{DataType, Token};
use crate::visitor::{ExprVisitor, StmtVisitor};

#[derive(PartialEq)]
enum FunctionType {
    None,
    Function,
    Method,
    Initializer,
}
#[derive(PartialEq)]
enum ClassType {
    None,
    Class,
}

pub struct Resolver {
    interpreter: Rc<Interpreter>,
    scopes: RefCell<Vec<RefCell<HashMap<String, bool>>>>,
}

impl Resolver {
    pub fn new(interpreter: Rc<Interpreter>) -> Self {
        Self {
            interpreter,
            scopes: RefCell::new(Vec::new()),
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.borrow_mut().push(RefCell::new(HashMap::new()));
    }

    fn end_scope(&mut self) {
        self.scopes.borrow_mut().pop();
    }

    fn declare(&mut self, name: &Token) -> anyhow::Result<DataType>  {
        if let Some(scope) = self.scopes.borrow().last() {
            if scope.borrow().contains_key(&name.lexeme) {
                return Err(anyhow!("Already a variable with this name in this scope."))
            }
            scope.borrow_mut().insert(name.lexeme.to_string(), false);
        }
        Ok(DataType::Nil)
    }

    fn define(&mut self, name: &Token) -> anyhow::Result<DataType> {
        if let Some(scope) = self.scopes.borrow().last() {
            scope.borrow_mut().insert(name.lexeme.to_string(), true);
        }
        Ok(DataType::Nil)
    }

    fn resolve_function(&mut self, stmt: &FunctionStmt) -> anyhow::Result<DataType> {
        self.begin_scope();
        for param in stmt.params.iter() {
            self.declare(param)?;
            self.define(param)?;
        }
        for body in &stmt.body {
            body.accept(self)?;
        }
        self.end_scope();
        Ok(DataType::Nil)
    }

    fn resolve_local(&mut self, expr: Rc<dyn Expr>, name: &Token) -> anyhow::Result<DataType> {
        for (scope, map) in self.scopes.borrow().iter().rev().enumerate() {
            if map.borrow().contains_key(&name.lexeme) {
                self.interpreter.resolve(expr, scope)?;
                return Ok(DataType::Nil)
            }
        }
        Ok(DataType::Nil)
    }
}

impl ExprVisitor for Resolver {
    fn visit_literal_expr(&mut self, _expr: &LiteralExpr) -> anyhow::Result<DataType> {
        Ok(DataType::Nil)
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> anyhow::Result<DataType> {
        expr.right.accept(self);
        Ok(DataType::Nil)
    }

    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> anyhow::Result<DataType> {
        expr.left.accept(self);
        expr.right.accept(self);
        Ok(DataType::Nil)
    }

    fn visit_call_expr(&mut self, expr: &CallExpr) -> anyhow::Result<DataType> {
        expr.callee.accept(self);
        for arguments in &expr.arguments {
            arguments.accept(self);
        }
        Ok(DataType::Nil)
    }

    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> anyhow::Result<DataType> {
        expr.expression.accept(self);
        Ok(DataType::Nil)
    }

    fn visit_var_expr(&mut self, expr: &VarExpr) -> anyhow::Result<DataType> {
        let token = &expr.var_name;
        if !self.scopes.borrow().is_empty()
            && self
            .scopes
            .borrow()
            .last()
            .unwrap()
            .borrow()
            .get(&token.lexeme)
            == Some(&false)
        {
            return Err(anyhow!("Can't read local variable in its own initializer."));
        } else {
            let expr: Rc<dyn Expr> = Rc::new(VarExpr {
                var_name: expr.var_name.clone(),
            });
            self.resolve_local(expr, &token)?;
        }
        Ok(DataType::Nil)
    }

    fn visit_assign_expr(&mut self, expr: &AssignExpr) -> anyhow::Result<DataType> {
        expr.accept(self);

        let rc_expr: Rc<dyn Expr> = Rc::new(AssignExpr {
            var_name: expr.var_name.clone(),
            var_value: expr.var_value.clone(),
        });

        self.resolve_local(rc_expr, &expr.var_name)?;
        Ok(DataType::Nil)
    }

    fn visit_logical_expr(&mut self, expr: &LogicalExpr) -> anyhow::Result<DataType> {
        expr.left.accept(self);
        expr.right.accept(self);
        Ok(DataType::Nil)
    }
}


impl StmtVisitor for Resolver {
    fn visit_print_statement(&mut self, stmt: &PrintStmt) -> anyhow::Result<DataType> {
        stmt.expression.accept(self);
        Ok(DataType::Nil)
    }

    fn visit_expr_statement(&mut self, stmt: &ExprStmt) -> anyhow::Result<DataType> {
        stmt.expression.accept(self);
        Ok(DataType::Nil)
    }

    fn visit_var_statement(&mut self, stmt: &VarStmt) -> anyhow::Result<DataType> {
        self.declare(&stmt.var_name)?;
        if let Some(initializer) = &stmt.var_value {
            initializer.accept(self);
        }
        self.define(&stmt.var_name)?;
        Ok(DataType::Nil)
    }

    fn visit_block_statement(&mut self, stmt: &BlockStmt) -> anyhow::Result<DataType> {
        self.begin_scope();
        for statement in &stmt.statements {
            let _ = statement.accept(self)?;
        }
        self.end_scope();
        Ok(DataType::Nil)
    }

    fn visit_if_statement(&mut self, stmt: &IfStmt) -> anyhow::Result<DataType> {
        stmt.condition.accept(self);
        stmt.then_branch.accept(self)?;
        if let Some(else_branch) = &stmt.else_branch {
            else_branch.accept(self)?;
        }
        Ok(DataType::Nil)
    }

    fn visit_while_statement(&mut self, stmt: &WhileStmt) -> anyhow::Result<DataType> {
        stmt.condition.accept(self);
        stmt.body.accept(self)?;
        Ok(DataType::Nil)
    }

    fn visit_function_statement(&mut self, stmt: &FunctionStmt) -> anyhow::Result<DataType> {
        self.declare(&stmt.name)?;
        self.define(&stmt.name)?;
        self.resolve_function(stmt)?;
        Ok(DataType::Nil)
    }

    fn visit_return_statement(&mut self, stmt: &ReturnStmt) -> anyhow::Result<DataType> {
        if let Some(return_value) = &stmt.value {
            return_value.accept(self);
        }
        Ok(DataType::Nil)
    }
}