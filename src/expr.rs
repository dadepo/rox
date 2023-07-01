use std::fmt::{Debug, Formatter};
use std::rc::Rc;

use crate::token::{DataType, Token};
use crate::visitor::Visitor;

pub trait Expr {
    fn accept(&self, visitor: &mut dyn Visitor) -> DataType;
}

impl Debug for dyn Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "self")
    }
}

pub struct LiteralExpr {
    pub value: Option<DataType>,
}
impl Expr for LiteralExpr {
    fn accept(&self, visitor: &mut dyn Visitor) -> DataType {
        visitor.visit_literal_expr(self)
    }
}

pub struct UnaryExpr {
    pub operator: Token,
    pub right: Rc<dyn Expr>,
}
impl Expr for UnaryExpr {
    fn accept(&self, visitor: &mut dyn Visitor) -> DataType {
        visitor.visit_unary_expr(self)
    }
}

pub struct BinaryExpr {
    pub left: Rc<dyn Expr>,
    pub operator: Token,
    pub right: Rc<dyn Expr>,
}
impl Expr for BinaryExpr {
    fn accept(&self, visitor: &mut dyn Visitor) -> DataType {
        visitor.visit_binary_expr(self)
    }
}

pub struct GroupingExpr {
    pub expression: Rc<dyn Expr>,
}
impl Expr for GroupingExpr {
    fn accept(&self, visitor: &mut dyn Visitor) -> DataType {
        visitor.visit_grouping_expr(self)
    }
}
