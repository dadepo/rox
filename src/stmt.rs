use std::rc::Rc;

use anyhow::Result;

use crate::expr::Expr;
use crate::token::Token;
use crate::visitor::StmtVisitor;

pub trait Stmt {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> Result<()>;
}

pub struct PrintStmt {
    pub expression: Rc<dyn Expr>,
}
impl Stmt for PrintStmt {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> Result<()> {
        visitor.visit_print_statement(self)
    }
}

pub struct ExprStmt {
    pub expression: Rc<dyn Expr>,
}

impl Stmt for ExprStmt {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> Result<()> {
        visitor.visit_expr_statement(self)
    }
}

pub struct VarStmt {
    pub var_name: Token,
    pub var_value: Option<Rc<dyn Expr>>,
}

impl Stmt for VarStmt {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> Result<()> {
        visitor.visit_var_statement(self)
    }
}

pub struct BlockStmt {
    pub statements: Vec<Rc<dyn Stmt>>,
}

impl Stmt for BlockStmt {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> Result<()> {
        visitor.visit_block_statement(self)
    }
}

pub struct IfStmt {
    pub condition: Rc<dyn Expr>,
    pub then_branch: Rc<dyn Stmt>,
    pub else_branch: Option<Rc<dyn Stmt>>,
}

impl Stmt for IfStmt {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> Result<()> {
        visitor.visit_if_statement(self)
    }
}

pub struct WhileStmt {
    pub condition: Rc<dyn Expr>,
    pub body: Rc<dyn Stmt>
}

impl Stmt for WhileStmt {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> Result<()> {
        visitor.visit_while_statement(self)
    }
}