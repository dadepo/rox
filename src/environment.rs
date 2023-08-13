use crate::token::{DataType, Token};
use anyhow::anyhow;
use anyhow::Result;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Environment {
    pub parent_environment: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Option<DataType>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            parent_environment: None,
            values: HashMap::new(),
        }
    }
    pub fn new_with_parent_environment(parent_environment: Rc<RefCell<Environment>>) -> Self {
        let parent_environment = Some(parent_environment);
        Self {
            parent_environment,
            values: HashMap::new(),
        }
    }
    pub fn define(&mut self, name: String, value: Option<DataType>) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<DataType> {
        if let Some(Some(value)) = self.values.get(name) {
            Some(value.to_owned())
        } else {
            // check parent
            match &self.parent_environment {
                Some(parent_env) => parent_env.borrow().get(name),
                None => None,
            }
        }
    }

    pub fn get_at(&self, distance: usize, name: &str) -> Option<DataType> {
        if distance == 0 {
            self.values.get(&name.to_string()).unwrap().clone()
        } else {
            self.parent_environment
                .as_ref()
                .unwrap()
                .borrow()
                .get_at(distance - 1, name)
        }
    }

    pub fn assign(&mut self, name: String, value: Option<DataType>) -> Result<()> {
        if let std::collections::hash_map::Entry::Occupied(mut e) = self.values.entry(name.clone())
        {
            e.insert(value);
            Ok(())
        } else if self.parent_environment.is_some() {
            self.parent_environment
                .clone()
                .unwrap()
                .borrow_mut()
                .assign(name, value)?;
            Ok(())
        } else {
            Err(anyhow!("variable does not exist"))
        }
    }

    pub fn assign_at(&mut self, distance: usize, name: &Token, value: DataType) -> Result<()> {
        if distance == 0 {
            self.values.insert(name.lexeme.to_string(), Some(value));
            Ok(())
        } else {
            self.parent_environment
                .as_ref()
                .unwrap()
                .borrow_mut()
                .assign_at(distance - 1, name, value)
        }
    }
}
