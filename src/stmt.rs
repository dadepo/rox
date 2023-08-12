use std::any::Any;
use std::rc::Rc;

use anyhow::Result;

use crate::expr::Expr;
use crate::token::{DataType, Token};
use crate::visitor::StmtVisitor;

pub trait Stmt {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> Result<DataType>;
    fn as_any(&self) -> &dyn Any;
}

pub struct PrintStmt {
    pub expression: Rc<dyn Expr>,
}
impl Stmt for PrintStmt {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> Result<DataType> {
        visitor.visit_print_statement(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ExprStmt {
    pub expression: Rc<dyn Expr>,
}

impl Stmt for ExprStmt {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> Result<DataType> {
        visitor.visit_expr_statement(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct VarStmt {
    pub var_name: Token,
    pub var_value: Option<Rc<dyn Expr>>,
}

impl Stmt for VarStmt {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> Result<DataType> {
        visitor.visit_var_statement(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct BlockStmt {
    pub statements: Vec<Rc<dyn Stmt>>,
}

impl Stmt for BlockStmt {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> Result<DataType> {
        visitor.visit_block_statement(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct IfStmt {
    pub condition: Rc<dyn Expr>,
    pub then_branch: Rc<dyn Stmt>,
    pub else_branch: Option<Rc<dyn Stmt>>,
}

impl Stmt for IfStmt {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> Result<DataType> {
        visitor.visit_if_statement(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct WhileStmt {
    pub condition: Rc<dyn Expr>,
    pub body: Rc<dyn Stmt>
}

impl Stmt for WhileStmt {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> Result<DataType> {
        visitor.visit_while_statement(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct FunctionStmt {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Rc<dyn Stmt>>,
}

impl Stmt for FunctionStmt {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> Result<DataType> {
        visitor.visit_function_statement(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ReturnStmt {
    pub keyword: Token,
    pub value: Option<Rc<dyn Expr>>,
}

impl Stmt for ReturnStmt {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> Result<DataType> {
        visitor.visit_return_statement(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ClassStmt {
    pub name: Token,
    pub methods: Vec<Rc<dyn Stmt>>
}

impl Stmt for ClassStmt {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> Result<DataType> {
        visitor.visit_class_statement(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}