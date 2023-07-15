use crate::token::DataType;
use anyhow::anyhow;
use anyhow::Result;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Environment {
    parent_environment: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Option<DataType>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            parent_environment: None,
            values: HashMap::new(),
        }
    }
    pub fn new_with_parent_environment(parent_environment: Environment) -> Self {
        let parent_environment = Some(Rc::new(RefCell::new(parent_environment)));
        Self {
            parent_environment,
            values: HashMap::new(),
        }
    }
    pub fn define(&mut self, name: String, value: Option<DataType>) {
        self.values.insert(name, value);
    }

    pub fn get(&mut self, name: &str) -> Option<DataType> {
        if let Some(Some(value)) = self.values.get(name) {
            Some(value.to_owned())
        } else {
            // check parent
            match &self.parent_environment {
                Some(parent_env) => parent_env.borrow_mut().get(name),
                None => None
            }
        }
    }

    pub fn assign(&mut self, name: String, value: Option<DataType>) -> Result<()> {
        if let std::collections::hash_map::Entry::Occupied(mut e) = self.values.entry(name.clone()) {
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
}
