use crate::scope::Scope;
use crate::parser::{Program, Statement, Expr, TermOp, ExprOp, Term, Factor};
use crate::object::Object;
use core::borrow::Borrow;
use std::mem;

pub struct Interpreter {
    current_scope: Scope,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter { current_scope: Scope::new_root() }
    }

    pub fn eval_program(&mut self, program: &Program) -> Object {
        self.eval_statement(&program.body)
    }

    pub fn extend_scope(&mut self) {
        self.current_scope = mem::replace(&mut self.current_scope, Scope::new_empty()).extend();
    }

    pub fn retrieve_scope(&mut self) {
        self.current_scope = mem::replace(&mut self.current_scope, Scope::new_empty()).retrieve();
    }

    pub fn eval_block_vec(&mut self, statements: &Vec<Statement>) -> Object {
        self.extend_scope();
        for statement in statements {
            match statement {
                Statement::Return { statement: ret_stmt } => {
                    return self.eval_statement(ret_stmt);
                },
                _ => self.eval_statement(statement),
            };
        }
        self.retrieve_scope();
        Object::Null
    }

    pub fn eval_statement(&mut self, statement: &Statement) -> Object {
        match statement {
            Statement::BlockStatement { statements } => {
                self.eval_block_vec(statements)
            },
            Statement::Assign { ident, expr } => {
                self.current_scope.set(ident, &self.eval_expr(expr));
                Object::Null
            },
            Statement::Simple { expr } => {
                self.eval_expr(expr)
            },
            Statement::Return { statement: ret_stmt } => {
                panic!("Attempted to use return outside of block");
            },
            Statement::FunctionDec { params, body } => {
                Object::Function(params.clone(), *(body).clone())
            },
        }
    }

    pub fn eval_expr(&self, expr: &Expr) -> Object {
        match expr {
            Expr::Simple { terms, ops } => {
                match terms.len() {
                    0 => Object::Null,
                    _ => {
                        let mut value = self.eval_term(terms.first().unwrap());
                        let mut current_term = 1; // already eval'd first term
                        while current_term < terms.len() {
                            value = self.eval_exprop(ops.get(current_term - 1).unwrap(), value,
                                                     self.eval_term(terms.get(current_term).unwrap()));
                            current_term += 1;
                        }
                        value
                    },
                }
            },
            Expr::FunctionCall { func, args } => {
                match self.eval_factor(func) {
                    Object::RustFunction(func) => {
                        // evaluate arguments
                        let obj_args = args.iter().map(|expr| self.eval_expr(expr)).collect::<Vec<_>>();
                        func(obj_args)
                    },
                    obj => panic!("Cannot call {:?}", obj),
                }
            },
        }
    }

    pub fn eval_term(&self, term: &Term) -> Object {
        match term.factors.len() {
            0 => Object::Null,
            _ => {
                let mut value = self.eval_factor(term.factors.first().unwrap());
                let mut current_factor = 1; // already eval'd first term
                while current_factor < term.factors.len() {
                    value = self.eval_termop(term.ops.get(current_factor - 1).unwrap(), value,
                                             self.eval_factor(term.factors.get(current_factor).unwrap()));
                    current_factor += 1;
                }
                value
            },
        }
    }

    pub fn eval_factor(&self, factor: &Factor) -> Object {
        match factor {
            Factor::IdentFactor(ident) => self.current_scope.get(ident)
                .unwrap_or_else(|| panic!("Identifier not found in current scope: {}", ident)),
            Factor::StringFactor(string) => Object::String(string.clone()),
            Factor::IntFactor(num) => Object::Integer(num.clone()),
            Factor::ExprFactor(expr) => self.eval_expr(expr),
        }
    }

    pub fn eval_termop(&self, op: &TermOp, left: Object, right: Object) -> Object {
        match (op, &left, &right) {
            (op, Object::Integer(l_num), Object::Integer(r_num)) => {
                Object::Integer(match op {
                    TermOp::Div => l_num / r_num,
                    TermOp::Mul => l_num * r_num,
                })
            },
            (TermOp::Mul, Object::String(string), Object::Integer(amt)) => {
                if *amt < 0 {
                    panic!("Cannot repeat string < 0 times!");
                }
                Object::String(string.repeat(*amt as usize))
            },
            _ => panic!("Unsupported operation {:?} for {:?} and {:?}", op, left, right),
        }
    }

    pub fn eval_exprop(&self, op: &ExprOp, left: Object, right: Object) -> Object {
        match (op, &left, &right) {
            (op, Object::Integer(l_num), Object::Integer(r_num)) => {
                Object::Integer(match op {
                    ExprOp::Add => l_num + r_num,
                    ExprOp::Sub => l_num - r_num,
                })
            },
            (ExprOp::Add, Object::String(l_string), Object::String(r_string)) => {
                let mut new_str = l_string.clone();
                new_str.push_str(r_string.as_str());
                Object::String(new_str)
            },
            _ => panic!("Unsupported operation {:?} for {:?} and {:?}", op, left, right),
        }
    }
}