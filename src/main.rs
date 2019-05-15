use std::fs;

mod token;
mod parser;
mod scope;
mod object;
mod interpreter;
mod builtins;

fn main() {
    let tokens = token::tokenize(&fs::read_to_string("program.suro").unwrap());
//    println!("Tokens: {:?}", &tokens);

    let program = parser::Parser::new(tokens).parse();
//    println!("Tree: {:#?}", &program);

    let result = interpreter::Interpreter::new().eval_program(&program);
    println!("Result: {:?}", &result)
}
