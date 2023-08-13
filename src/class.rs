use crate::expr::{Expr, VarExpr};
use crate::functions::{LoxCallable, LoxFunction};
use crate::interpreter::Interpreter;
use crate::token::{DataType, Token};
use anyhow::anyhow;
use anyhow::Result;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct LoxClass {
    pub name: String,
    pub super_class: Option<Box<LoxClass>>,
    pub methods: HashMap<String, LoxFunction>,
}

impl LoxClass {
    pub fn find_method(&self, name: String) -> Option<LoxFunction> {
        if self.methods.contains_key(&name) {
            return Some(self.methods.get(&name).unwrap().clone());
        }

        if let Some(superclass) = &self.super_class {
            return superclass.find_method(name);
        }

        None
    }
}

#[derive(Debug, Clone)]
pub struct LoxInstance {
    class: LoxClass,
    fields: RefCell<HashMap<String, DataType>>,
}

impl LoxInstance {
    pub fn get(&self, name: &Token) -> Result<DataType> {
        if self.fields.borrow().contains_key(&name.lexeme) {
            return Ok(self
                .fields
                .borrow()
                .get(&name.lexeme)
                .ok_or(anyhow!("Can't find property"))?
                .clone());
        }

        let method = self.class.find_method(name.lexeme.clone());

        if method.is_some() {
            return Ok(DataType::Function(method.unwrap().bind(self.clone())));
        }

        Err(anyhow!("Undefined property"))
    }

    pub fn set(&self, name: &Token, value: DataType) {
        self.fields.borrow_mut().insert(name.lexeme.clone(), value);
    }
}

impl fmt::Display for LoxClass {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "<Class {}>", self.name)
    }
}

impl fmt::Display for LoxInstance {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "<Instance {}>", self.class.name)
    }
}

impl LoxCallable for LoxClass {
    fn arity(&self) -> usize {
        if let Some(initializer) = self.find_method("init".to_string()) {
            initializer.arity()
        } else {
            0
        }
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<DataType>) -> Result<DataType> {
        let lox_instance = LoxInstance {
            class: self.clone(),
            fields: RefCell::new(HashMap::new()),
        };
        if let Some(initializer) = self.find_method("init".to_string()) {
            initializer
                .bind(lox_instance.clone())
                .call(interpreter, arguments)?;
        }

        Ok(DataType::Instance(lox_instance))
    }
}
