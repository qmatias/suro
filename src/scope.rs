use std::collections::HashMap;

use crate::builtins::fill_with_builtins;
use crate::object::Object;

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

    pub fn new_empty() -> Scope {
        Scope {
            memvars: HashMap::new(),
            parent: None,
        }
    }

    /// returns a new scope this scope as the parent
    pub fn extend(self) -> Scope {
        Scope {
            memvars: HashMap::new(),
            parent: Some(Box::new(self)),
        }
    }

    /// returns the parent scope and destroys this one
    pub fn retrieve(self) -> Scope {
        *self.parent.unwrap()
    }

    pub fn set(&mut self, name: &str, value: &Object) {
        self.memvars.insert(name.to_string(), value.clone());
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        match self.memvars.get(name) {
            Some(object) => Some(object.clone()),
            None => {
                match &self.parent {
                    Some(parent) => {
                        parent.get(name)
                    }
                    None => None,
                }
            }
        }
    }
}