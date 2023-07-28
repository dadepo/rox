use std::cell::RefCell;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;
use std::time::SystemTime;
use crate::environment::Environment;
use crate::stmt::{FunctionStmt, Stmt};
use crate::token::{DataType, Token};
use crate::visitor::Interpreter;

pub trait LoxCallable: Debug + Display {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<DataType>) -> anyhow::Result<DataType>;
}

pub enum Kind {
    Function
}

#[derive(Clone)]
pub struct LoxFunction {
    pub body: Rc<Vec<Rc<dyn Stmt>>>,
    pub params: Rc<Vec<Token>>,
    name: Box<Token>,
    closure: Rc<RefCell<Environment>>,
    is_init: bool,
}

impl LoxFunction {
    pub fn new(
        declaration: &FunctionStmt,
        closure: &Rc<RefCell<Environment>>,
        is_init: bool,
    ) -> LoxFunction {
        LoxFunction {
            body: Rc::new(declaration.body.clone()),
            params: Rc::new(declaration.params.clone()),
            name: Box::new(declaration.name.clone()),
            closure: Rc::clone(closure),
            is_init,
        }
    }
}

impl Display for LoxFunction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "<Function {}>", self.name.lexeme)
    }
}

impl Debug for LoxFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = format!("<Function {}>", self.name.lexeme);
        f.debug_struct("LoxFunction")
            .field("name:", &value)
            .finish()
    }
}

impl LoxCallable for LoxFunction {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<DataType>) -> anyhow::Result<DataType> {
        let mut environment = Environment::new_with_parent_environment(Rc::clone(&interpreter.globals));
        for (i, token) in self.params.iter().enumerate() {
            let value = match arguments.get(i) {
                Some(d) => d.clone(),
                None => DataType::Nil,
            };
            environment.define(token.lexeme.to_string(), Some(value));
        }
        let statements = self.clone().body;
        interpreter.execute_block(statements, environment)?;
        Ok(DataType::Nil)
    }
}

#[derive(Debug, Clone)]
pub struct LoxNative {
    pub function: Rc<dyn LoxCallable>,
}
impl fmt::Display for LoxNative {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.function)
    }
}

#[derive(Debug)]
pub struct Clock {
    name: String,
}

impl Clock {
    pub fn new(name: String) -> Clock {
        Clock { name }
    }
}

impl LoxCallable for Clock {
    fn call(&self, _: &mut Interpreter, _: Vec<crate::token::DataType>) -> anyhow::Result<DataType> {
        Ok(match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => DataType::Number(n.as_millis() as f64),
            Err(_) => DataType::Nil,
        })
    }

    fn arity(&self) -> usize {
        0
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "<Native-Function {}>", self.name)
    }
}