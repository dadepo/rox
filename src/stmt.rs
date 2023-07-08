use std::rc::Rc;
use crate::expr::Expr;
use crate::visitor::StmtVisitor;
use anyhow::Result;
use crate::token::{Token, TokenType};

pub trait Stmt {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> Result<()>;
}

pub struct PrintStmt {
    pub expression: Rc<dyn Expr>
}
impl Stmt for PrintStmt {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> Result<()> {
        visitor.visit_print_statement(self)
    }
}

pub struct ExprStmt {
    pub expression: Rc<dyn Expr>
}

impl Stmt for ExprStmt {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> Result<()> {
        visitor.visit_expr_statement(self)
    }
}

pub struct VarStmt {
    pub var_name: Token,
    pub var_value: Option<Rc<dyn Expr>>
}

impl Stmt for VarStmt {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> Result<()> {
        visitor.visit_var_statement(self)
    }
}

pub struct BlockStmt {
    pub statements: Vec<Rc<dyn Stmt>>
}

impl Stmt for BlockStmt {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> Result<()> {
        visitor.visit_block_statement(self)
    }
}