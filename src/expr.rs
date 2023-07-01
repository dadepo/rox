use std::fmt::{Debug, Formatter};
use std::rc::Rc;

use crate::token::{DataType, Token};

pub trait Expr {}

impl Debug for dyn Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "self")
    }
}

pub struct LiteralExpr {
    pub value: Option<DataType>,
}
impl Expr for LiteralExpr {}

pub struct UnaryExpr {
    pub operator: Token,
    pub right: Rc<dyn Expr>,
}
impl Expr for UnaryExpr {}

pub struct BinaryExpr {
    pub left: Rc<dyn Expr>,
    pub operator: Token,
    pub right: Rc<dyn Expr>,
}
impl Expr for BinaryExpr {}

pub struct GroupingExpr {
    pub expression: Rc<dyn Expr>,
}
impl Expr for GroupingExpr {}