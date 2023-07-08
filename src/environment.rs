use std::collections::HashMap;
use crate::token::DataType;

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Option<DataType>>
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new()
        }
    }
    pub fn define(&mut self, name: String, value: Option<DataType>) {
        self.values.insert(name, value);
    }

    pub fn get(&mut self, name: &str) -> Option<DataType> {
        self.values.remove(name).and_then(|e|e)
    }
}