use core::panic;
use std::collections::HashMap;
use std::iter::Map;

use crate::ast::BinOp;
use crate::print_program;
use crate::val::Val;
use crate::Exp;

// Make env = map[id, val]
// make val type
type Id = String;
type Env = HashMap<Id, Val>;

pub struct Interpreter {
    env: Env,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut map: HashMap<Id, Val> = HashMap::new();
        Interpreter { env: map }
    }

    pub fn eval(&mut self, e: Exp) -> Val {
        // Debugging
        print_program(&e);

        return match e {
            Exp::IntExp(v) => Val::IntVal(v),
            Exp::BinOpExp(left, op, right) => {
                let left = self.eval(*left);
                let right = self.eval(*right);
                match op {
                    BinOp::PlusBinOp => match (left, right) {
                        (Val::IntVal(v1), Val::IntVal(v2)) => Val::IntVal(v1 + v2),
                        _ => {
                            panic!("Error plus")
                        }
                    },
                    BinOp::MinusBinOp => match (left, right) {
                        (Val::IntVal(v1), Val::IntVal(v2)) => Val::IntVal(v1 - v2),
                        _ => {
                            panic!("Error minus")
                        }
                    },
                    BinOp::TimesBinOp => match (left, right) {
                        (Val::IntVal(v1), Val::IntVal(v2)) => Val::IntVal(v1 * v2),
                        _ => {
                            panic!("Error times")
                        }
                    },
                    BinOp::DivideBinOp => {
                        match (left, right) {
                            (Val::IntVal(v1), Val::IntVal(v2)) => {
                                /*if (right == Val::IntVal(0)) {
                                    panic!("Division by 0 error")
                                }*/
                                Val::IntVal(v1 / v2)
                            }
                            _ => {
                                panic!("Error divide")
                            }
                        }
                    }
                    _ => {
                        panic!("Undefined, remove me when done")
                    }
                }
            }

            _ => Val::Undefined,
        };
    }
}
