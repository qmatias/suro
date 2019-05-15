use std::collections::HashMap;
use crate::object::Object;
use crate::builtins::fill_with_builtins;

#[derive(PartialEq, Debug, Clone)]
pub struct Scope {
    memvars: HashMap<String, Object>,
    parent: Option<Box<Scope>>,
}

impl Scope {
    pub fn new_root() -> Scope {
        Scope {
            memvars: {
                let mut map = HashMap::new();
                fill_with_builtins(&mut map);
                map
            },
            parent: None,
        }
    }

    pub fn new_with_parent(parent: Box<Scope>) -> Scope {
        Scope {
            memvars: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn set_memvar(&mut self, name: &str, value: &Object) {
        self.memvars.insert(name.to_string(), value.clone());
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        match self.memvars.get(name) {
            Some(object) => Some(object.clone()),
            None => {
                match &self.parent {
                    Some(parent) => {
                        parent.get(name)
                    },
                    None => None,
                }
            }
        }
    }
}