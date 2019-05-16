use crate::token::{Token, Type};

#[derive(Debug, PartialEq, Clone)]
pub enum TermOp {
    Mul,
    Div,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExprOp {
    Add,
    Sub,
}

#[derive(Debug)]
pub struct Program {
    pub body: Statement,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Assign {
        ident: String,
        expr: Expr,
        change: bool,
    },
    FunctionDec {
        params: Vec<String>,
        body: Box<Statement>,
    },
    Return {
        statement: Box<Statement>,
    },
    Expr {
        expr: Expr,
    },
    BlockStatement {
        statements: Vec<Statement>,
    },
    If {
        conditions: Vec<(Option<Statement>, Statement)>,
    },
    FunctionCall {
        func: Box<Statement>,
        args: Vec<Statement>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct Expr {
    pub terms: Vec<Term>,
    pub ops: Vec<ExprOp>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Term {
    pub factors: Vec<Factor>,
    pub ops: Vec<TermOp>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Factor {
    IntFactor(i32),
    StringFactor(String),
    BoolFactor(bool),
    StmtFactor(Box<Statement>),
    IdentFactor(String),
}

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            index: 0,
        }
    }

    pub fn parse(&mut self) -> Program {
        self.parse_program()
    }

    fn parse_program(&mut self) -> Program {
        let program = Program {
            body: self.parse_statement(),
        };
        self.expect_consume(Type::EOF);
        program
    }

    fn parse_block(&mut self) -> Statement {
        self.expect_consume(Type::BlockStart);
        Statement::BlockStatement {
            statements: {
                let mut lst = Vec::new();
                while self.current_unwrap().token_type != Type::BlockEnd {
                    lst.push(self.parse_line());
                };
                self.expect_consume(Type::BlockEnd);
                lst
            }
        }
    }

    fn parse_line(&mut self) -> Statement {
        let statement = self.parse_statement();
        self.expect_consume(Type::Terminator);
        statement
    }

    fn parse_statement(&mut self) -> Statement {
        match self.current_unwrap().token_type {
            Type::Assignment => {
                self.consume_unwrap(); // consume let
                let ident = self.expect_consume(Type::Ident).str; // consume and store ident
                self.expect_consume(Type::AssignmentOp); // consume equals sine
                let expr = self.parse_expr();
                Statement::Assign {
                    ident,
                    expr,
                    change: false,
                }
            }
            Type::Change => {
                self.consume_unwrap(); // consume let
                let ident = self.expect_consume(Type::Ident).str; // consume and store ident
                self.expect_consume(Type::AssignmentOp); // consume equals sine
                let expr = self.parse_expr();
                Statement::Assign {
                    ident,
                    expr,
                    change: true,
                }
            }
            Type::Return => {
                self.consume_unwrap(); // consume return
                Statement::Return { statement: Box::new(self.parse_statement()) }
            }
            Type::BlockStart => {
                self.parse_block()
            }
            Type::If => {
                Statement::If {
                    conditions: {
                        let mut val = Vec::new();
                        val.push(self.parse_condition_tuple());
                        loop {
                            if let Type::Else = self.current_unwrap().token_type {
                                val.push(self.parse_condition_tuple());
                            } else {
                                break;
                            }
                        }
                        val
                    }
                }
            }
            _ => Statement::Expr { expr: self.parse_expr() },
        }
    }

    fn parse_condition_tuple(&mut self) -> (Option<Statement>, Statement) {
        match self.current_unwrap().token_type {
            Type::If => {
                self.consume_unwrap();
                let condition = self.parse_statement();
                self.expect_consume(Type::Then);
                let consequent = self.parse_statement();
                (Some(condition), consequent)
            },
            Type::Else => {
                self.consume_unwrap();
                if self.current_unwrap().token_type == Type::If {
                    self.parse_condition_tuple()
                } else {
                    (None, self.parse_statement())
                }
            },
            _ => panic!("Error parsing condition tuple at token: {:?}, expected If or Else token", self.current_unwrap()),
        }
    }

    fn parse_expr(&mut self) -> Expr {
        let mut terms = vec![self.parse_term()];
        let mut ops = Vec::new();
        loop {
            match self.current_unwrap().token_type {
                Type::Add => {
                    ops.push(ExprOp::Add);
                }
                Type::Sub => {
                    ops.push(ExprOp::Sub);
                }
                _ => break,
            }
            self.consume_unwrap(); // consume operator
            terms.push(self.parse_term());
        };
        Expr {
            terms,
            ops,
        }
    }

    fn parse_term(&mut self) -> Term {
        let mut term = Term {
            factors: vec![self.parse_factor()],
            ops: Vec::new(),
        };
        loop {
            match self.current_unwrap().token_type {
                Type::Mul => {
                    term.ops.push(TermOp::Mul);
                }
                Type::Div => {
                    term.ops.push(TermOp::Div);
                }
                _ => break,
            }
            self.consume_unwrap(); // consume operator
            term.factors.push(self.parse_factor());
        };
        term
    }

    fn parse_factor(&mut self) -> Factor {
        match self.current_unwrap().token_type {
            Type::Integer => Factor::IntFactor(self.consume_unwrap().str.trim().parse::<i32>()
                .unwrap_or_else(|_| panic!("Failed to parse integer at token {:?}", self.current_unwrap()))),
            Type::String => Factor::StringFactor({
                let full = self.consume_unwrap().str;
                String::from(&full[1..full.len() - 1]) // remove start and end quotes
            }),
            Type::True => {
                self.consume_unwrap();
                Factor::BoolFactor(true)
            }
            Type::False => {
                self.consume_unwrap();
                Factor::BoolFactor(false)
            }
            Type::Ident => Factor::IdentFactor(self.consume_unwrap().str),
            Type::OpenGrouper => {
                self.consume_unwrap(); // consume parentheses
                let factor: Factor = Factor::StmtFactor(Box::new(
                    self.parse_statement()
                ));
                self.expect_consume(Type::CloseGrouper);
                factor
            }
            Type::BlockStart => {
                Factor::StmtFactor(Box::new(self.parse_block()))
            }
            Type::FunctionCall => {
                self.consume_unwrap(); // consume FunctionCall
                Factor::StmtFactor(Box::new(Statement::FunctionCall {
                    func: Box::new(self.parse_statement()),
                    args: { // build arguments
                        let mut val = Vec::new();
                        if self.consume_if(Type::ParameterList) {
                            self.expect_consume(Type::OpenGrouper); // consume opening paren of args
                            loop {
                                val.push(self.parse_statement());
                                if !self.consume_if(Type::Separator) {
                                    break;
                                }
                            }
                            self.expect_consume(Type::CloseGrouper); // consume closing paren
                        }
                        val
                    },
                }))
            }
            _ => panic!("Tried to parse factor but token {:?} is not of type Num, String, Ident, OpenGrouper, or FunctionCall", self.current_unwrap()),
        }
    }

    fn expect_consume(&mut self, consume_type: Type) -> Token {
        let got = self.consume_unwrap();
        if got.token_type != consume_type {
            panic!("Expected {:?}, got {:?}", consume_type, got);
        }
        got
    }

    fn consume_if(&mut self, consume_type: Type) -> bool {
        if self.current_unwrap().token_type == consume_type {
            self.consume_unwrap();
            true
        } else {
            false
        }
    }

    fn consume(&mut self) -> Option<Token> {
        let val = self.current();
        self.increment();
        val
    }

    fn consume_unwrap(&mut self) -> Token {
        // dummy token after calling self.error (self.error will panic)
        let val = self.current_unwrap();
        self.increment();
        val
    }

    fn current(&self) -> Option<Token> {
        if self.index < self.tokens.len() {
            Some(self.tokens[self.index].clone()) // return a copy of the enum
        } else {
            None
        }
    }

    fn current_unwrap(&self) -> Token {
        self.current().unwrap_or_else(|| panic!("Ran out of tokens to parse"))
    }

    fn increment(&mut self) {
        self.index += 1;
    }
}
