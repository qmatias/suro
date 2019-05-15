#[derive(PartialEq, Debug, Clone)]
pub enum Object {
    String(String),
    Integer(i32),
    RustFunction(fn(Vec<Object>) -> Object),
    Null,
}

