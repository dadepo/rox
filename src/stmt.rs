use std::rc::Rc;
use crate::expr::Expr;
use crate::visitor::StmtVisitor;
use anyhow::Result;

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
