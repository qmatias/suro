use crate::object::Object;
use std::collections::HashMap;

pub fn get_builtins() -> Vec<(String, Object)> {
    vec![
        get_builtin("print", s_print)
    ]
}

pub fn fill_with_builtins(map: &mut HashMap<String, Object>) {
    for (name, obj) in get_builtins() {
        map.insert(name, obj);
    }
}

fn get_builtin(name: &str, func: fn(Vec<Object>) -> Object) -> (String, Object) {
    (name.to_string(), Object::RustFunction(func))
}

fn s_print(args: Vec<Object>) -> Object {
    for obj in args {
        match obj {
            Object::String(string) => println!("{}", string),
            Object::Integer(num) => println!("{}", num),
            _ => panic!("Invalid argument for print"),
        }
    }
    Object::Null
}