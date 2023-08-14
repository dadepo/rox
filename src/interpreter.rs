use crate::class::LoxClass;
use crate::environment::Environment;
use crate::expr::{
    AssignExpr, BinaryExpr, CallExpr, Expr, GetExpr, GroupingExpr, LiteralExpr, LogicalExpr,
    SetExpr, SuperExpr, ThisExpr, UnaryExpr, VarExpr,
};
use crate::functions::{Clock, LoxCallable, LoxFunction, LoxNative};
use crate::stmt::{
    BlockStmt, ClassStmt, ExprStmt, FunctionStmt, IfStmt, PrintStmt, ReturnStmt, Stmt, VarStmt,
    WhileStmt,
};
use crate::token::TokenType::OR;
use crate::token::{DataType, Token, TokenType};
use crate::visitor::{ExprVisitor, StmtVisitor};
use anyhow::{anyhow, Result};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    pub environment: RefCell<Rc<RefCell<Environment>>>,
    pub locals: RefCell<HashMap<String, usize>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new()));

        let clock = DataType::NativeFunction(LoxNative {
            function: Rc::new(Clock::new("Clock".to_string())),
        });
        globals
            .borrow_mut()
            .define("clock".to_string(), Some(clock));

        Self {
            globals: Rc::clone(&globals),
            environment: RefCell::new(Rc::clone(&globals)),
            locals: RefCell::new(HashMap::new()),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Rc<dyn Stmt>>) -> Result<()> {
        for statement in statements {
            self.execute(statement)?;
        }
        Ok(())
    }

    pub fn execute_block(
        &mut self,
        statements: &Rc<Vec<Rc<dyn Stmt>>>,
        environment: Environment,
    ) -> Result<DataType> {
        let previous = self.environment.replace(Rc::new(RefCell::new(environment)));
        for statement in statements.as_ref() {
            let returned = self.execute(statement.clone())?;
            match returned {
                DataType::Nil => continue,
                _ => {
                    self.environment.replace(previous);
                    return Ok(returned);
                }
            }
        }
        self.environment.replace(previous);
        Ok(DataType::Nil)
    }

    fn evaluate(&mut self, expression: Rc<dyn Expr>) -> DataType {
        expression.accept(self)
    }

    fn execute(&mut self, statement: Rc<dyn Stmt>) -> Result<DataType> {
        statement.accept(self)
    }

    fn is_truthy(&self, value: &DataType) -> bool {
        match value {
            DataType::String(_) => true,
            DataType::Number(_) => true,
            DataType::Bool(_) => true,
            DataType::Nil => false,
            _ => false,
        }
    }

    fn is_equal(&self, left: DataType, right: DataType) -> bool {
        match (left, right) {
            (DataType::Nil, DataType::Nil) => true,
            (DataType::Nil, _) => false,
            (DataType::Bool(l), DataType::Bool(r)) => l == r,
            (DataType::Bool(_), _) => false,
            (DataType::Number(l), DataType::Number(r)) => l == r,
            (DataType::Number(_), _) => false,
            (DataType::String(l), DataType::String(r)) => l == r,
            (DataType::String(_), _) => false,
            _ => false,
        }
    }

    fn get_hash_key(&self, expr: Rc<dyn Expr>) -> Result<String> {
        if let Ok(var) = self.get_var_expr_hash(Rc::clone(&expr)) {
            Ok(var)
        } else if let Ok(assign) = self.get_assign_expr_hash(Rc::clone(&expr)) {
            Ok(assign)
        } else if let Ok(this) = self.get_this_expr_hash(Rc::clone(&expr)) {
            Ok(this)
        } else if let Ok(super_expr) = self.get_super_expr_hash(Rc::clone(&expr)) {
            Ok(super_expr)
        } else {
            return Err(anyhow!("could not find hash of expr"));
        }
    }

    pub fn resolve(&self, expr: Rc<dyn Expr>, depth: usize) -> Result<DataType> {
        let hash: String = self.get_hash_key(expr)?;
        self.locals.borrow_mut().insert(hash, depth);
        Ok(DataType::Nil)
    }

    pub fn get_var_expr_hash(&self, expr: Rc<dyn Expr>) -> Result<String> {
        if let Some(var) = expr.as_any().downcast_ref::<VarExpr>() {
            let token = &var.var_name;
            Ok(format!(
                "{}-{}-{:?}",
                token.lexeme, token.line, token.literal
            ))
        } else {
            Err(anyhow!("Not a VarExpr"))
        }
    }

    pub fn get_assign_expr_hash(&self, expr: Rc<dyn Expr>) -> Result<String> {
        if let Some(var) = expr.as_any().downcast_ref::<AssignExpr>() {
            let token = &var.var_name;
            Ok(format!(
                "{}-{}-{:?}",
                token.lexeme, token.line, token.literal
            ))
        } else {
            Err(anyhow!("Not a AssignExpr"))
        }
    }

    pub fn get_this_expr_hash(&self, expr: Rc<dyn Expr>) -> Result<String> {
        if let Some(var) = expr.as_any().downcast_ref::<ThisExpr>() {
            let token = &var.keyword;
            Ok(format!(
                "{}-{}-{:?}",
                token.lexeme, token.line, token.literal
            ))
        } else {
            Err(anyhow!("Not a AssignExpr"))
        }
    }

    pub fn get_super_expr_hash(&self, expr: Rc<dyn Expr>) -> Result<String> {
        if let Some(var) = expr.as_any().downcast_ref::<SuperExpr>() {
            let token = &var.keyword;
            Ok(format!(
                "{}-{}-{:?}",
                token.lexeme, token.line, token.literal
            ))
        } else {
            Err(anyhow!("Not a SuperExpr"))
        }
    }

    fn look_up_variable(&self, name: &Token, expr: &Rc<dyn Expr>) -> Result<DataType> {
        let local: String = self.get_hash_key(Rc::clone(expr))?;
        let option = if let Some(distance) = self.locals.borrow().get(&local) {
            self.environment
                .borrow()
                .borrow()
                .get_at(*distance, &name.lexeme)
        } else {
            self.globals.borrow().get(&name.lexeme)
        };

        option.ok_or(anyhow!("var not found"))
    }
}

impl ExprVisitor for Interpreter {
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Result<DataType> {
        match expr.value.as_ref() {
            None => Ok(DataType::Nil),
            Some(value) => Ok(value.clone()),
        }
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Result<DataType> {
        let right = self.evaluate(Rc::clone(&expr.right));
        match expr.operator.token_type {
            TokenType::MINUS => match right {
                DataType::Number(s) => Ok(DataType::Number(-1f64 + s)),
                _ => Err(anyhow!("Can only negate numbers")),
            },
            TokenType::BANG => {
                let value = !self.is_truthy(&right);
                Ok(DataType::Bool(value))
            }
            _ => Err(anyhow!("Can only negate numbers or truthy values")),
        }
    }

    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Result<DataType> {
        let left = self.evaluate(Rc::clone(&expr.left));
        let right = self.evaluate(Rc::clone(&expr.right));

        match expr.operator.token_type {
            TokenType::MINUS => {
                let left = match left {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!("Can only use - with numbers")),
                };
                let right = match right {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!("")),
                };
                Ok(DataType::Number(left - right))
            }
            TokenType::SLASH => {
                let left = match left {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!("Can only use / with numbers")),
                };
                let right = match right {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!("")),
                };
                Ok(DataType::Number(left / right))
            }
            TokenType::STAR => {
                let left = match left {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!("Can only use / with numbers")),
                };
                let right = match right {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!("")),
                };
                Ok(DataType::Number(left * right))
            }
            TokenType::PLUS => {
                let left = match left {
                    DataType::Number(_) | DataType::String(_) => left,
                    _ => return Err(anyhow!("Can only use * with numbers and strings")),
                };
                let right = match right {
                    DataType::Number(_) | DataType::String(_) => right,
                    _ => return Err(anyhow!("")),
                };

                match (left, right) {
                    (DataType::String(l), DataType::String(r)) => {
                        Ok(DataType::String(format!("{}{}", l, r)))
                    }
                    (DataType::Number(l), DataType::Number(r)) => Ok(DataType::Number(l + r)),
                    _ => Err(anyhow!("Both left and right should be number/string")),
                }
            }
            TokenType::GREATER => {
                let left = match left {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!("Can only use > with numbers")),
                };
                let right = match right {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!("")),
                };
                Ok(DataType::Bool(left > right))
            }
            TokenType::GREATEREQUAL => {
                let left = match left {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!("Can only use >= with numbers")),
                };
                let right = match right {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!("")),
                };
                Ok(DataType::Bool(left >= right))
            }
            TokenType::LESS => {
                let left = match left {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!("Can only use < with numbers")),
                };
                let right = match right {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!("")),
                };
                Ok(DataType::Bool(left < right))
            }
            TokenType::LESSEQUAL => {
                let left = match left {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!("Can only use <= with numbers")),
                };
                let right = match right {
                    DataType::Number(n) => n,
                    _ => return Err(anyhow!("")),
                };
                Ok(DataType::Bool(left <= right))
            }
            TokenType::BANGEQUAL => Ok(DataType::Bool(!self.is_equal(left, right))),
            TokenType::EQUALEQUAL => Ok(DataType::Bool(self.is_equal(left, right))),
            _ => Err(anyhow!("Unsupported operator")),
        }
    }

    fn visit_call_expr(&mut self, expr: &CallExpr) -> Result<DataType> {
        let callee = self.evaluate(Rc::clone(&expr.callee));
        let mut arguments = vec![];

        for argument in &expr.arguments {
            arguments.push(self.evaluate(Rc::clone(argument)))
        }

        let function: Rc<dyn LoxCallable> = match callee {
            DataType::Function(f) => Rc::new(f),
            DataType::Class(class) => Rc::new(class),
            DataType::NativeFunction(nf) => nf.function,
            _ => return Err(anyhow!("Can only call functions and classes.")),
        };

        if function.arity() != arguments.len() {
            let msg = format!(
                "Expected {} arguments but got {}.",
                function.arity(),
                arguments.len()
            );
            return Err(anyhow!(msg));
        };

        function.call(self, arguments)
    }

    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> Result<DataType> {
        Ok(self.evaluate(Rc::clone(&expr.expression)))
    }

    fn visit_var_expr(&mut self, expr: &VarExpr) -> Result<DataType> {
        let var_name = expr.var_name.clone();
        let expr: Rc<dyn Expr> = Rc::new(VarExpr {
            var_name: var_name.clone(),
        });
        self.look_up_variable(&var_name, &expr)
        // self.environment
        //     .borrow()
        //     .borrow()
        //     .get(&expr.var_name.lexeme)
        //     .ok_or(anyhow!("var does not exist"))
    }

    fn visit_assign_expr(&mut self, expr: &AssignExpr) -> Result<DataType> {
        let expr_rc: Rc<dyn Expr> = Rc::new(AssignExpr {
            var_name: expr.var_name.clone(),
            var_value: expr.var_value.clone(),
        });
        let value = self.evaluate(Rc::clone(expr.var_value.as_ref().unwrap()));
        let local: String = self.get_hash_key(Rc::clone(&expr_rc))?;
        if let Some(distance) = self.locals.borrow().get(&local) {
            self.environment.borrow().borrow_mut().assign_at(
                *distance,
                &expr.var_name,
                value.clone(),
            )?;
        } else {
            self.globals
                .borrow_mut()
                .assign(expr.var_name.lexeme.clone(), Some(value.clone()))?;
        }

        Ok(value)
    }

    fn visit_logical_expr(&mut self, expr: &LogicalExpr) -> Result<DataType> {
        let left = self.evaluate(Rc::clone(&expr.left));
        if expr.operator.token_type == OR {
            if self.is_truthy(&left) {
                return Ok(left);
            }
        } else if !self.is_truthy(&left) {
            return Ok(left);
        }

        Ok(self.evaluate(Rc::clone(&expr.right)))
    }

    fn visit_get_expr(&mut self, expr: &GetExpr) -> Result<DataType> {
        let object = self.evaluate(Rc::clone(&expr.object));
        match object {
            DataType::Instance(instance) => instance.get(&expr.name),
            _ => Err(anyhow!("Only instances have properties.")),
        }
    }

    fn visit_set_expr(&mut self, expr: &SetExpr) -> Result<DataType> {
        let object = self.evaluate(Rc::clone(&expr.object));

        return match object {
            DataType::Instance(instance) => {
                let value = self.evaluate(Rc::clone(&expr.value));
                instance.set(&expr.name, value.clone());
                let cloned = expr.object.clone();
                let var_expr = cloned.as_any().downcast_ref::<VarExpr>().unwrap();
                self.globals.borrow_mut().assign(
                    var_expr.var_name.lexeme.clone(),
                    Some(DataType::Instance(instance)),
                )?;
                Ok(value)
            }
            _ => Err(anyhow!("Only instances have fields.")),
        };
    }

    fn visit_this_expr(&mut self, expr: &ThisExpr) -> Result<DataType> {
        let keyword = expr.keyword.clone();

        let expr: Rc<dyn Expr> = Rc::new(ThisExpr {
            keyword: expr.keyword.clone(),
        });
        self.look_up_variable(&keyword, &expr)
    }

    fn visit_super_expr(&mut self, expr: &SuperExpr) -> Result<DataType> {

        let expr_rc: Rc<dyn Expr> = Rc::new(SuperExpr {
            keyword: expr.keyword.clone(),
            method: expr.method.clone(),
        });

        let local: String = self.get_hash_key(Rc::clone(&expr_rc))?;
        return if let Some(distance) = self.locals.borrow().get(&local) {
            let super_class = match self
                .environment
                .borrow()
                .borrow()
                .get_at(*distance, "super")
            {
                Some(DataType::Class(lox_super_class)) => lox_super_class,
                _ => return Err(anyhow!("Lox super class not found")),
            };

            let object = match self
                .environment
                .borrow()
                .borrow()
                .get_at(*distance - 1, "this")
            {
                Some(DataType::Instance(lox_instance)) => lox_instance,
                _ => return Err(anyhow!("Lox instance not found")),
            };

            let found_method = super_class.find_method(expr.method.lexeme.clone());
            if let Some(found_method) = found_method {
                Ok(DataType::Function(found_method.bind(object)))
            } else {
                return Err(anyhow!("Undefined property {}", expr.method.lexeme));
            }
        } else {
            return Err(anyhow!("Unexpected error"));
        };
    }
}

impl StmtVisitor for Interpreter {
    fn visit_print_statement(&mut self, stmt: &PrintStmt) -> Result<DataType> {
        let value = self.evaluate(Rc::clone(&stmt.expression));
        println!("{}", value.to_string());
        Ok(DataType::Nil)
    }

    fn visit_expr_statement(&mut self, stmt: &ExprStmt) -> Result<DataType> {
        self.evaluate(Rc::clone(&stmt.expression));
        Ok(DataType::Nil)
    }

    fn visit_var_statement(&mut self, stmt: &VarStmt) -> Result<DataType> {
        match stmt.var_value.as_ref() {
            None => self
                .environment
                .borrow()
                .borrow_mut()
                .define(stmt.var_name.lexeme.clone(), None),
            Some(stmt_line) => {
                let value = self.evaluate(stmt_line.clone());
                self.environment
                    .borrow()
                    .borrow_mut()
                    .define(stmt.var_name.lexeme.clone(), Some(value))
            }
        }
        Ok(DataType::Nil)
    }

    fn visit_block_statement(&mut self, stmt: &BlockStmt) -> Result<DataType> {
        let env = Environment::new_with_parent_environment(self.environment.borrow().clone());
        let statements = Rc::new(stmt.statements.clone());
        self.execute_block(&statements, env)
    }

    fn visit_if_statement(&mut self, stmt: &IfStmt) -> Result<DataType> {
        let condition = self.evaluate(Rc::clone(&stmt.condition));
        let mut return_value: DataType = DataType::Nil;
        match condition {
            DataType::Bool(value) => {
                if value {
                    return_value = self.execute(Rc::clone(&stmt.then_branch))?
                } else if let Some(else_branch) = stmt.else_branch.as_ref() {
                    return_value = self.execute(Rc::clone(else_branch))?
                } else {
                    return_value = DataType::Nil
                }
            }
            _ => Err(anyhow!("condition not boolean"))?,
        };
        Ok(return_value)
    }

    fn visit_while_statement(&mut self, stmt: &WhileStmt) -> Result<DataType> {
        let mut condition = true;

        while condition {
            condition = match &self.evaluate(Rc::clone(&stmt.condition)) {
                DataType::Bool(true_value) => *true_value,
                _ => return Err(anyhow!("condition should be boolean")),
            };

            if condition {
                self.execute(Rc::clone(&stmt.body))?;
            }
        }

        Ok(DataType::Nil)
    }

    fn visit_function_statement(&mut self, stmt: &FunctionStmt) -> Result<DataType> {
        let function = LoxFunction::new(stmt, &self.environment.borrow(), false);
        self.environment
            .borrow()
            .borrow_mut()
            .define(stmt.name.lexeme.clone(), Some(DataType::Function(function)));
        Ok(DataType::Nil)
    }

    fn visit_return_statement(&mut self, stmt: &ReturnStmt) -> Result<DataType> {
        if stmt.value.is_some() {
            Ok(self.evaluate(stmt.value.clone().unwrap()))
        } else {
            Err(anyhow!("return error"))
        }
    }

    fn visit_class_statement(&mut self, stmt: &ClassStmt) -> Result<DataType> {
        let mut super_class: Option<LoxClass> = None;

        if let Some(class) = &stmt.super_class {
            match self.evaluate(Rc::clone(class)) {
                DataType::Class(evaluated_class) => super_class = Some(evaluated_class),
                _ => return Err(anyhow!("Superclass must be a class.")),
            }
        }

        self.environment
            .borrow()
            .borrow_mut()
            .define(stmt.name.lexeme.clone(), None);

        if stmt.super_class.is_some() {
            let environment: Environment = Environment::new_with_parent_environment(self.environment.borrow().clone());
            self.environment.replace(
                Rc::clone(&Rc::new(RefCell::new(environment)))
            );

            self.environment
                .borrow()
                .borrow_mut()
                .define("super".to_string(), super_class.clone().map(DataType::Class));
        }

        let mut methods: HashMap<String, LoxFunction> = HashMap::new();

        for method in &stmt.methods {
            let function = method.as_any().downcast_ref::<FunctionStmt>().unwrap();
            let m = LoxFunction::new(
                function,
                &self.environment.borrow(),
                function.name.lexeme.eq_ignore_ascii_case("init"),
            );
            methods.insert(function.name.lexeme.clone(), m);
        }

        let lox_class: LoxClass = LoxClass {
            name: stmt.name.lexeme.clone(),
            super_class: super_class.clone().map(Box::new),
            methods,
        };

        if super_class.is_some() {
            let parent_environment: Rc<RefCell<Environment>> = self.environment.borrow().borrow().parent_environment.clone().unwrap();
            self.environment.replace(parent_environment);
        }

        self.environment
            .borrow()
            .borrow_mut()
            .assign(stmt.name.lexeme.clone(), Some(DataType::Class(lox_class)))?;

        Ok(DataType::Nil)
    }
}
