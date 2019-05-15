use crate::parser::Statement;
use crate::scope::Scope;

#[derive(PartialEq, Debug, Clone)]
pub enum Object {
    String(String),
    Integer(i32),
    RustFunction(fn(Vec<Object>) -> Object),
    Function(Vec<String>, Statement),
    Null,
}

