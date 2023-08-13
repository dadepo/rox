use std::any::Any;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::token::{DataType, Token};
use crate::visitor::ExprVisitor;

pub trait Expr {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> DataType;
    fn as_any(&self) -> &dyn Any;
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
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> DataType {
        visitor.visit_literal_expr(self).unwrap()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct UnaryExpr {
    pub operator: Token,
    pub right: Rc<dyn Expr>,
}
impl Expr for UnaryExpr {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> DataType {
        visitor.visit_unary_expr(self).unwrap()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct BinaryExpr {
    pub left: Rc<dyn Expr>,
    pub operator: Token,
    pub right: Rc<dyn Expr>,
}
impl Expr for BinaryExpr {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> DataType {
        visitor.visit_binary_expr(self).unwrap()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct GroupingExpr {
    pub expression: Rc<dyn Expr>,
}
impl Expr for GroupingExpr {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> DataType {
        visitor.visit_grouping_expr(self).unwrap()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct VarExpr {
    // Will be of IDENTIFIER type
    // We don't save the value here, value is saved in env
    pub var_name: Token,
}

impl Expr for VarExpr {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> DataType {
        visitor.visit_var_expr(self).unwrap()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct AssignExpr {
    pub var_name: Token,
    pub var_value: Option<Rc<dyn Expr>>,
}

impl Expr for AssignExpr {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> DataType {
        visitor.visit_assign_expr(self).unwrap()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct LogicalExpr {
    pub left: Rc<dyn Expr>,
    pub operator: Token,
    pub right: Rc<dyn Expr>,
}

impl Expr for LogicalExpr {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> DataType {
        visitor.visit_logical_expr(self).unwrap()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct CallExpr {
    pub callee: Rc<dyn Expr>,
    pub paren: Token,
    pub arguments: Vec<Rc<dyn Expr>>,
}

impl Expr for CallExpr {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> DataType {
        visitor.visit_call_expr(self).unwrap()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct GetExpr {
    pub object: Rc<dyn Expr>,
    pub name: Token,
}

impl Expr for GetExpr {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> DataType {
        visitor.visit_get_expr(self).unwrap()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct SetExpr {
    pub object: Rc<dyn Expr>,
    pub name: Token,
    pub value: Rc<dyn Expr>,
}

impl Expr for SetExpr {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> DataType {
        visitor.visit_set_expr(self).unwrap()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ThisExpr {
    pub keyword: Token,
}

impl Expr for ThisExpr {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> DataType {
        visitor.visit_this_expr(self).unwrap()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct SuperExpr {
    pub keyword: Token,
    pub method: Token,
}

impl Expr for SuperExpr {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> DataType {
        visitor.visit_super_expr(self).unwrap()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
