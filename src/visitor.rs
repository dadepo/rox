use std::cell::RefCell;
use std::rc::Rc;

use anyhow::anyhow;
use anyhow::Result;

use crate::environment::Environment;
use crate::expr::{AssignExpr, BinaryExpr, CallExpr, Expr, GroupingExpr, LiteralExpr, LogicalExpr, UnaryExpr, VarExpr};
use crate::functions::{Clock, LoxCallable, LoxFunction, LoxNative};
use crate::stmt::{BlockStmt, ExprStmt, FunctionStmt, IfStmt, PrintStmt, ReturnStmt, Stmt, VarStmt, WhileStmt};
use crate::token::{DataType, TokenType};
use crate::token::TokenType::OR;

pub trait Visitor {
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Result<DataType>;
    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Result<DataType>;
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Result<DataType>;
    fn visit_call_expr(&mut self, expr: &CallExpr) -> Result<DataType>;
    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> Result<DataType>;
    fn visit_var_expr(&mut self, expr: &VarExpr) -> Result<DataType>;
    fn visit_assign_expr(&mut self, expr: &AssignExpr) -> Result<DataType>;
    fn visit_logical_expr(&mut self, expr: &LogicalExpr) -> Result<DataType>;
}

pub trait StmtVisitor {
    fn visit_print_statement(&mut self, stmt: &PrintStmt) -> Result<DataType>;
    fn visit_expr_statement(&mut self, stmt: &ExprStmt) -> Result<DataType>;
    fn visit_var_statement(&mut self, stmt: &VarStmt) -> Result<DataType>;
    fn visit_block_statement(&mut self, stmt: &BlockStmt) -> Result<DataType>;
    fn visit_if_statement(&mut self, stmt: &IfStmt) -> Result<DataType>;
    fn visit_while_statement(&mut self, stmt: &WhileStmt) -> Result<DataType>;
    fn visit_function_statement(&mut self, stmt: &FunctionStmt) -> Result<DataType>;
    fn visit_return_statement(&mut self, stmt: &ReturnStmt) -> Result<DataType>;
}
