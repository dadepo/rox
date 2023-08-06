use anyhow::Result;

use crate::expr::{AssignExpr, BinaryExpr, CallExpr, GroupingExpr, LiteralExpr, LogicalExpr, UnaryExpr, VarExpr};
use crate::stmt::{BlockStmt, ExprStmt, FunctionStmt, IfStmt, PrintStmt, ReturnStmt, VarStmt, WhileStmt};
use crate::token::DataType;

pub trait ExprVisitor {
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
