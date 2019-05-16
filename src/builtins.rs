use std::collections::HashMap;

use crate::object::Object;

pub fn get_builtins() -> Vec<(String, Object)> {
    vec![
        get_builtin("print", s_print),
        get_builtin("to_bool", s_to_bool),
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
            Object::Boolean(val) => println!("{}", val),
            obj => panic!("Invalid argument for s_print: {:?}", obj),
        }
    }
    Object::Null
}

fn s_to_bool(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        panic!("Must supply only one argument to s_to_bool");
    }
    Object::Boolean(to_bool(args.get(0).unwrap()))
}

pub fn to_bool(arg: &Object) -> bool {
    match arg {
        Object::Boolean(val) => *val,
        Object::Integer(num) => *num != 0,
        Object::String(string) => string.len() != 0,
        obj => panic!("Cannot convert {:?} to boolean", obj),
    }
}