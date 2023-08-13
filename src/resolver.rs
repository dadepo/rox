use crate::expr::{
    AssignExpr, BinaryExpr, CallExpr, Expr, GetExpr, GroupingExpr, LiteralExpr, LogicalExpr,
    SetExpr, SuperExpr, ThisExpr, UnaryExpr, VarExpr,
};
use crate::functions::Kind::Function;
use crate::interpreter::Interpreter;
use crate::stmt::{
    BlockStmt, ClassStmt, ExprStmt, FunctionStmt, IfStmt, PrintStmt, ReturnStmt, Stmt, VarStmt,
    WhileStmt,
};
use crate::token::{DataType, Token};
use crate::visitor::{ExprVisitor, StmtVisitor};
use anyhow::anyhow;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

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

pub struct Resolver<'a> {
    interpreter: &'a Interpreter,
    scopes: RefCell<Vec<RefCell<HashMap<String, bool>>>>,
    current_function: RefCell<FunctionType>,
    current_class: RefCell<ClassType>,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a Interpreter) -> Self {
        Self {
            interpreter,
            scopes: RefCell::new(Vec::new()),
            current_function: RefCell::new(FunctionType::None),
            current_class: RefCell::new(ClassType::None),
        }
    }

    pub fn resolve(&mut self, statements: Vec<Rc<dyn Stmt>>) -> anyhow::Result<()> {
        for stmt in statements.iter() {
            stmt.accept(self)?;
        }
        Ok(())
    }

    fn begin_scope(&mut self) {
        self.scopes.borrow_mut().push(RefCell::new(HashMap::new()));
    }

    fn end_scope(&mut self) {
        self.scopes.borrow_mut().pop();
    }

    fn declare(&mut self, name: &Token) -> anyhow::Result<DataType> {
        if let Some(scope) = self.scopes.borrow().last() {
            if scope.borrow().contains_key(&name.lexeme) {
                return Err(anyhow!("Already a variable with this name in this scope."));
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

    fn resolve_function(
        &mut self,
        stmt: &FunctionStmt,
        function_type: FunctionType,
    ) -> anyhow::Result<DataType> {
        let enclosing_function = self.current_function.replace(function_type);
        self.begin_scope();
        for param in stmt.params.iter() {
            self.declare(param)?;
            self.define(param)?;
        }
        for body in &stmt.body {
            body.accept(self)?;
        }
        self.end_scope();
        self.current_function.replace(enclosing_function);
        Ok(DataType::Nil)
    }

    fn resolve_local(&mut self, expr: Rc<dyn Expr>, name: &Token) -> anyhow::Result<DataType> {
        for (scope, map) in self.scopes.borrow().iter().rev().enumerate() {
            if map.borrow().contains_key(&name.lexeme) {
                self.interpreter.resolve(expr, scope)?;
                return Ok(DataType::Nil);
            }
        }
        Ok(DataType::Nil)
    }
}

impl<'a> ExprVisitor for Resolver<'a> {
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

    fn visit_get_expr(&mut self, expr: &GetExpr) -> anyhow::Result<DataType> {
        expr.object.accept(self);
        Ok(DataType::Nil)
    }

    fn visit_set_expr(&mut self, expr: &SetExpr) -> anyhow::Result<DataType> {
        expr.value.accept(self);
        expr.object.accept(self);
        Ok(DataType::Nil)
    }

    fn visit_this_expr(&mut self, expr: &ThisExpr) -> anyhow::Result<DataType> {
        if *self.current_class.borrow() == ClassType::None {
            return Err(anyhow!("Can't use 'this' outside of a class."));
        }

        let rc_expr: Rc<dyn Expr> = Rc::new(ThisExpr {
            keyword: expr.keyword.clone(),
        });

        self.resolve_local(rc_expr, &expr.keyword)?;
        Ok(DataType::Nil)
    }

    fn visit_super_expr(&mut self, expr: &SuperExpr) -> anyhow::Result<DataType> {
        let rc_expr: Rc<dyn Expr> = Rc::new(SuperExpr {
            keyword: expr.keyword.clone(),
            method: expr.method.clone(),
        });
        self.resolve_local(rc_expr, &expr.keyword)
    }
}

impl<'a> StmtVisitor for Resolver<'a> {
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
        self.resolve_function(stmt, FunctionType::Function)?;
        Ok(DataType::Nil)
    }

    fn visit_return_statement(&mut self, stmt: &ReturnStmt) -> anyhow::Result<DataType> {
        if *self.current_function.borrow() == FunctionType::None {
            return Err(anyhow!("Can't return from top-level code."));
        }
        if let Some(return_value) = &stmt.value {
            if *self.current_function.borrow() == FunctionType::Initializer {
                return Err(anyhow!("Can't return a value from an initializer."));
            }
            return_value.accept(self);
        }
        Ok(DataType::Nil)
    }

    fn visit_class_statement(&mut self, stmt: &ClassStmt) -> anyhow::Result<DataType> {
        let enclosing_class = self.current_class.replace(ClassType::Class);
        self.declare(&stmt.name)?;
        self.define(&stmt.name)?;

        if let Some(super_class) = &stmt.super_class {
            let super_class = super_class.as_any().downcast_ref::<VarExpr>().unwrap();
            if stmt
                .name
                .lexeme
                .eq_ignore_ascii_case(&super_class.var_name.lexeme.to_string())
            {
                return Err(anyhow!("A class can't inherit from itself."));
            }
            super_class.accept(self);
        }

        if stmt.super_class.is_some() {
            self.begin_scope();
            self.scopes
                .borrow()
                .last()
                .borrow_mut()
                .unwrap()
                .borrow_mut()
                .insert("super".to_string(), true);
        }

        self.begin_scope();

        self.scopes
            .borrow()
            .last()
            .borrow_mut()
            .unwrap()
            .borrow_mut()
            .insert("this".to_string(), true);

        for method in &stmt.methods {
            let method = method.as_any().downcast_ref::<FunctionStmt>().unwrap();
            let mut declaration = FunctionType::Method;
            if method.name.lexeme.eq_ignore_ascii_case("init") {
                declaration = FunctionType::Initializer;
            }
            self.resolve_function(method, declaration)?;
        }

        self.end_scope();

        if stmt.super_class.is_some() {
            self.end_scope();
        }

        self.current_class.replace(enclosing_class);
        Ok(DataType::Nil)
    }
}
