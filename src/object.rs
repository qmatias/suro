use crate::parser::Statement;

#[derive(PartialEq, Debug, Clone)]
pub enum Object {
    String(String),
    Integer(i32),
    Boolean(bool),
    RustFunction(fn(Vec<Object>) -> Object),
    Function(Vec<String>, Statement),
    Null,
}

