use std::process;

use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    EOF,
    Whitespace,
    Comment,
    FunctionCall,
    ParameterList,
    Ident,
    String,
    Assignment,
    OpenGrouper,
    CloseGrouper,
    Separator,
    Terminator,
    Integer,
    Add,
    Sub,
    Mul,
    Div,
    AssignmentOp,
    Return,
    BlockStart,
    BlockEnd,
    If,
    True,
    False,
    FuncDec,
    FuncParams,
    Then,
    Else,
    Change,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: Type,
    pub str: String,
}

pub fn tokenize(program_string: &str) -> Vec<Token> {
    let expressions: [(Regex, Type); 29] = [
        (Regex::new(r"^--.*?\n").unwrap(), Type::Comment),
        (Regex::new(r#"^".*?""#).unwrap(), Type::String),
        (Regex::new(r"^'.*?'").unwrap(), Type::String),
        (Regex::new(r"^[0-9]+").unwrap(), Type::Integer),
        (Regex::new(r"^;").unwrap(), Type::Terminator),
        (Regex::new(r"^(call)[^A-Za-z0-9_\-]").unwrap(), Type::FunctionCall),
        (Regex::new(r"^(set)[^A-Za-z0-9_\-]").unwrap(), Type::Assignment),
        (Regex::new(r"^(change)[^A-Za-z0-9_\-]").unwrap(), Type::Change),
        (Regex::new(r"^(return)[^A-Za-z0-9_\-]").unwrap(), Type::Return),
        (Regex::new(r"^(to)[^A-Za-z0-9_\-]").unwrap(), Type::AssignmentOp),
        (Regex::new(r"^(with)[^A-Za-z0-9_\-]").unwrap(), Type::ParameterList),
        (Regex::new(r"^(if)[^A-Za-z0-9_\-]").unwrap(), Type::If),
        (Regex::new(r"^(then)[^A-Za-z0-9_\-]").unwrap(), Type::Then),
        (Regex::new(r"^(else)[^A-Za-z0-9_\-]").unwrap(), Type::Else),
        (Regex::new(r"^(true)[^A-Za-z0-9_\-]").unwrap(), Type::True),
        (Regex::new(r"^(false)[^A-Za-z0-9_\-]").unwrap(), Type::False),
        (Regex::new(r"^(func)[^A-Za-z0-9_\-]").unwrap(), Type::FuncDec),
        (Regex::new(r"^(takes)[^A-Za-z0-9_\-]").unwrap(), Type::FuncParams),
        (Regex::new(r"^\(").unwrap(), Type::OpenGrouper),
        (Regex::new(r"^\)").unwrap(), Type::CloseGrouper),
        (Regex::new(r"^\{").unwrap(), Type::BlockStart),
        (Regex::new(r"^\}").unwrap(), Type::BlockEnd),
        (Regex::new(r"^,").unwrap(), Type::Separator),
        (Regex::new(r"^\+").unwrap(), Type::Add),
        (Regex::new(r"^-").unwrap(), Type::Sub),
        (Regex::new(r"^\*").unwrap(), Type::Mul),
        (Regex::new(r"^/").unwrap(), Type::Div),
        (Regex::new(r"^[A-Za-z_][A-Za-z0-9_\-]*").unwrap(), Type::Ident),
        (Regex::new(r"^[ \n\t]+").unwrap(), Type::Whitespace),
    ];

    work(program_string, &expressions)
}

fn work(characters: &str, expressions: &[(Regex, Type)]) -> Vec<Token> {
    let mut token_list: Vec<Token> = Vec::new();
    let mut str_index: usize = 0;
    while str_index < characters.len() {
        let mut found = false;
        for (regex, token_type) in expressions {
//            println!("Searching at index: {}", str_index);
            if let Some(groups) = regex.captures(&characters[str_index..]) {
                let re_match = match groups.len() {
                    1 => groups.get(0).unwrap(),
                    _ => groups.get(1).unwrap(),
                };
                let str = String::from(&characters[re_match.start() + str_index..re_match.end() + str_index]);
//                    println!("Found match: {}", str);
                match token_type {
                    Type::Comment | Type::Whitespace => (), // don't add comments and whitespace to token list
                    _ => { // add everything else
                        token_list.push(Token {
                            token_type: token_type.clone(),
                            str,
                        });
                    }
                }
                str_index += re_match.end();
                found = true;
                break;
            }
        };
        if !found {
            tokenize_error(&characters, str_index);
        };
    }
    token_list.push(Token { token_type: Type::EOF, str: String::from("") });
    token_list
}

fn tokenize_error(characters: &str, index: usize) {
    println!("Unrecognized character at index {} ({:?}):\n", index, &characters[index..index + 1]);
    process::exit(1);
}