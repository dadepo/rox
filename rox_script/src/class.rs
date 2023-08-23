use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Formatter};

use anyhow::anyhow;
use anyhow::Result;

use crate::functions::{LoxCallable, LoxFunction};
use crate::interpreter::Interpreter;
use crate::token::{DataType, Token};

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

        if let Some(method) = method {
            return Ok(DataType::Function(method.bind(self.clone())));
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
