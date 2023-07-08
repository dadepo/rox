use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use anyhow::anyhow;
use anyhow::Result;
use crate::token::DataType;

#[derive(Debug, Clone)]
pub struct Environment {
    parent_environment: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Option<DataType>>
}

impl Environment {
    pub fn new() -> Self {
        Self {
            parent_environment: None,
            values: HashMap::new()
        }
    }
    pub fn new_with_parent_environment(parent_environment: Environment) -> Self {
        let parent_environment = Some(Rc::new(RefCell::new(parent_environment)));
        Self {
            parent_environment,
            values: HashMap::new()
        }
    }
    pub fn define(&mut self, name: String, value: Option<DataType>) {
        self.values.insert(name, value);
    }

    pub fn get(&mut self, name: &str) -> Option<DataType> {
        let value = self.values.get(name).and_then(|e| e.to_owned());
        if value.is_none() && self.parent_environment.is_some() {
            self.parent_environment.clone().unwrap().borrow_mut().get(name)
        } else {
            value
        }
    }

    pub fn assign(&mut self, name: String, value: Option<DataType>) -> Result<()> {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
            return Ok(());
        }
        if self.parent_environment.is_some() {
            self.parent_environment.clone().unwrap().borrow_mut().assign(name, value)?;
            return Ok(());
        }
        Err(anyhow!("variable does not exist"))
    }
}