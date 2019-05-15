use std::fs;

use clap::{App, Arg};

mod token;
mod parser;
mod scope;
mod object;
mod interpreter;
mod builtins;

fn main() {
    let matches = App::new("Suro Interpreter")
        .version("0.0.1")
        .author("Matias Kotlik (mdkotlik) <mdkotlik@gmail.com>")
        .about("Compiler for the suro language")
        .arg(Arg::with_name("FILE")
            .help("The file to run")
            .required(true)
            .index(1))
        .arg(Arg::with_name("v")
            .short("v")
            .help("Sets verbose mode"))
        .get_matches();

    let verbose = matches.is_present("v");

    let tokens = token::tokenize(&fs::read_to_string(matches.value_of("FILE").unwrap()).unwrap());
    if verbose {
        println!("Tokens: {:?}", &tokens);
    }

    let program = parser::Parser::new(tokens).parse();
    if verbose {
        println!("Tree: {:#?}", &program);
    }

    let result = interpreter::Interpreter::new().eval_program(&program);
    if verbose {
        println!("Result: {:?}", &result)
    }
}
