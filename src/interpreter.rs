use std::mem;

use crate::builtins::to_bool;
use crate::object::Object;
use crate::parser::{Expr, ExprOp, Factor, Program, Statement, Term, TermOp};
use crate::scope::Scope;

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

    /// create a new scope with self.current_scope as its parent and set self.current_scope to it
    pub fn extend_scope(&mut self) {
        self.current_scope = mem::replace(&mut self.current_scope, Scope::new_empty()).extend();
    }

    /// replace self.current_scope with its parent
    pub fn retrieve_scope(&mut self) {
        self.current_scope = mem::replace(&mut self.current_scope, Scope::new_empty()).retrieve();
    }

    pub fn eval_block_vec(&mut self, statements: &Vec<Statement>) -> Object {
        self.extend_scope();
        for statement in statements {
            match statement {
                Statement::Return { statement: ret_stmt } => {
                    return self.eval_statement(ret_stmt);
                }
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
            }
            Statement::Assign { ident, expr, change } => {
                let val = self.eval_expr(expr);
                if *change {
                    if !self.current_scope.reassign(ident, &val) {
                        panic!("Variable {} was reassigned but it does not exist.", ident);
                    }
                } else {
                    self.current_scope.set(ident, &val);
                }
                Object::Null
            }
            Statement::Expr { expr } => {
                self.eval_expr(expr)
            }
            Statement::Return { statement: ret_stmt } => {
                // this block will not be called unless there is a
                // return outside of a block
                self.eval_statement(ret_stmt)
            }
            Statement::FunctionDec { params, body } => {
                Object::Function(params.clone(), *(body).clone())
            }
            Statement::FunctionCall { func, args } => {
                match self.eval_statement(func) {
                    Object::RustFunction(func) => { // evaluate arguments
                        let obj_args = args.iter().map(|stmt| self.eval_statement(stmt)).collect::<Vec<_>>();
                        func(obj_args)
                    }
                    obj => panic!("Cannot call {:?}", obj),
                }
            }
            Statement::If { conditions } => {
                for condition in conditions {
                    match condition {
                        (Some(cond_stmt), consequent) => {
                            if to_bool(&self.eval_statement(cond_stmt)) {
                                return self.eval_statement(consequent);
                            }
                        }
                        (None, consequent) => return self.eval_statement(consequent),
                    }
                };
                Object::Null
            }
        }
    }

    pub fn eval_expr(&mut self, expr: &Expr) -> Object {
        match expr.terms.len() {
            0 => Object::Null,
            _ => {
                let mut total = self.eval_term(expr.terms.first().unwrap());
                let mut current_term = 1; // already eval'd first term
                while current_term < expr.terms.len() {
                    let right = self.eval_term(expr.terms.get(current_term).unwrap());
                    total = self.eval_exprop(expr.ops.get(current_term - 1).unwrap(), total, right);
                    current_term += 1;
                }
                total
            }
        }
    }

    pub fn eval_term(&mut self, term: &Term) -> Object {
        match term.factors.len() {
            0 => Object::Null,
            _ => {
                let mut total = self.eval_factor(term.factors.first().unwrap());
                let mut current_factor = 1; // already eval'd first factor
                while current_factor < term.factors.len() {
                    let right = self.eval_factor(term.factors.get(current_factor).unwrap());
                    total = self.eval_termop(term.ops.get(current_factor - 1).unwrap(), total, right);
                    current_factor += 1;
                }
                total
            }
        }
    }

    pub fn eval_factor(&mut self, factor: &Factor) -> Object {
        match factor {
            Factor::IdentFactor(ident) => self.current_scope.get(ident)
                .unwrap_or_else(|| panic!("Identifier not found in current scope: {}", ident)),
            Factor::StringFactor(string) => Object::String(string.clone()),
            Factor::BoolFactor(val) => Object::Boolean(val.clone()),
            Factor::IntFactor(num) => Object::Integer(num.clone()),
            Factor::StmtFactor(statement) => self.eval_statement(statement),
        }
    }

    pub fn eval_termop(&self, op: &TermOp, left: Object, right: Object) -> Object {
        match (op, &left, &right) {
            (op, Object::Integer(l_num), Object::Integer(r_num)) => {
                Object::Integer(match op {
                    TermOp::Div => l_num / r_num,
                    TermOp::Mul => l_num * r_num,
                })
            }
            (TermOp::Mul, Object::String(string), Object::Integer(amt)) => {
                if *amt < 0 {
                    panic!("Cannot repeat string < 0 times!");
                }
                Object::String(string.repeat(*amt as usize))
            }
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
            }
            (ExprOp::Add, Object::String(l_string), Object::String(r_string)) => {
                let mut new_str = l_string.clone();
                new_str.push_str(r_string.as_str());
                Object::String(new_str)
            }
            _ => panic!("Unsupported operation {:?} for {:?} and {:?}", op, left, right),
        }
    }
}