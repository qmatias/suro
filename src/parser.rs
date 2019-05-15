use crate::token::{Token, Type};

#[derive(Debug)]
pub enum TermOp {
    Mul,
    Div,
}

#[derive(Debug)]
pub enum ExprOp {
    Add,
    Sub,
}

#[derive(Debug)]
pub enum VarType {
    Memvar,
}

#[derive(Debug)]
pub struct Program {
    pub block: BlockStatement,
}

#[derive(Debug)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Assign {
        ident: String,
        var_type: VarType,
        expr: Expr,
    },
    Return {
        expr: Expr,
    },
    Simple {
        expr: Expr,
    },
}

#[derive(Debug)]
pub enum Expr {
    FunctionCall {
        func: Factor,
        args: Vec<Expr>,
    },
    Simple {
        terms: Vec<Term>,
        ops: Vec<ExprOp>,
    },
}

#[derive(Debug)]
pub struct Term {
    pub factors: Vec<Factor>,
    pub ops: Vec<TermOp>,
}

#[derive(Debug)]
pub enum Factor {
    IntFactor(i32),
    StringFactor(String),
    ExprFactor(Box<Expr>),
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

    // PROGRAM = BLOCK
    fn parse_program(&mut self) -> Program {
        let program = Program {
            block: self.parse_block(),
        };
        self.expect_consume(Type::EOF);
        program
    }

    // BLOCK = '{' ( LINE )* '}'
    fn parse_block(&mut self) -> BlockStatement {
        self.expect_consume(Type::BlockStart);
        let mut block = BlockStatement {
            statements: Vec::new(),
        };
        while self.current_unwrap().token_type != Type::BlockEnd {
            block.statements.push(self.parse_line());
        };
        self.expect_consume(Type::BlockEnd);
        block
    }

    // LINE = STATEMENT ';'
    fn parse_line(&mut self) -> Statement {
        let statement = self.parse_statement();
        self.expect_consume(Type::Terminator);
        statement
    }

    // STATEMENT = 'let' 'memvar' <IDENT> '=' EXPR
    //             | return EXPR
    //             | EXPR
    fn parse_statement(&mut self) -> Statement {
        match self.current_unwrap().token_type {
            Type::Assignment => {
                self.consume_unwrap(); // consume let
                let var_token_type = self.consume_unwrap().token_type;
                let var_type = match var_token_type { // consume vartype
                    Type::Memvar => VarType::Memvar,
                    _ => panic!("Expected VarType, got {:?}", self.current_unwrap()),
                };
                let ident = self.expect_consume(Type::Ident).str;
                self.expect_consume(Type::AssignmentOp); // consume =
                let expr = self.parse_expr();
                Statement::Assign {
                    var_type,
                    ident,
                    expr,
                }
            }
            Type::Return => {
                self.consume_unwrap(); // consume return
                Statement::Return { expr: self.parse_expr() }
            }
            _ => Statement::Simple { expr: self.parse_expr() },
        }
    }

    // EXPR = TERM ( ( '+' | '-' ) TERM )*
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
        Expr::Simple {
            terms,
            ops,
        }
    }

    // TERM = FACTOR ( ( '*' | '/' ) FACTOR )*
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

    // FACTOR = <NUMBER> | STRING | <IDENT> | '(' EXPR ')'
    //          | 'call' <FACTOR> ( 'with' '(' EXPR ( ',' EXPR )* ')' )?
    fn parse_factor(&mut self) -> Factor {
        match self.current_unwrap().token_type {
            Type::Integer => Factor::IntFactor(self.consume_unwrap().str.trim().parse::<i32>()
                .unwrap_or_else(|_| panic!("Failed to parse integer at token {:?}", self.current_unwrap()))),
            Type::String => Factor::StringFactor({
                let full = self.consume_unwrap().str;
                String::from(&full[1..full.len() - 1]) // remove start and end quotes
            }),
            Type::Ident => Factor::IdentFactor(self.consume_unwrap().str),
            Type::OpenGrouper => {
                self.consume_unwrap(); // consume parentheses
                let factor: Factor = Factor::ExprFactor(Box::new(
                    self.parse_expr()
                ));
                self.expect_consume(Type::CloseGrouper);
                factor
            }, Type::FunctionCall => {
                self.consume_unwrap(); // consume FunctionCall
                Factor::ExprFactor(Box::new(Expr::FunctionCall {
                    func: self.parse_factor(),
                    args: { // build arguments
                        let mut val = Vec::new();
                        if self.consume_if(Type::ParameterList) {
                            self.expect_consume(Type::OpenGrouper); // consume opening paren of args
                            loop {
                                val.push(self.parse_expr());
                                if !self.consume_if(Type::Separator) {
                                    break;
                                }
                            }
                            self.expect_consume(Type::CloseGrouper);
                        }
                        val
                    },
                }))
            },
            _ => panic!("Tried to parse factor but token {:?} is not of type Num, String, Ident, or OpenGrouper", self.current_unwrap()),
        }
    }

    fn expect_consume(&mut self, consume_type: Type) -> Token {
        let got = self.consume_unwrap();
        if got.token_type != consume_type {
            panic!("Expected {:?}, got {:?}", consume_type, got.token_type);
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
