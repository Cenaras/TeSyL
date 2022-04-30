use core::panic;
use std::collections::HashMap;

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
        let map: HashMap<Id, Val> = HashMap::new();
        Interpreter { env: map }
    }

    pub fn eval(&mut self, e: Exp) -> Val {
        // Debugging
        print_program(&e);

        return match e {
            Exp::IntExp(v) => Val::IntVal(v),
            Exp::BoolExp(b) => Val::BoolVal(b),
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
                    BinOp::DivideBinOp => match (left, right) {
                        (Val::IntVal(v1), Val::IntVal(v2)) => {
                            if (v2 == 0) {
                                panic!("Division by 0 error")
                            }
                            Val::IntVal(v1 / v2)
                        }
                        _ => {
                            panic!("Error divide")
                        }
                    },
                }
            }
            // Update environment. LetExp returns Unit
            Exp::LetExp(id, exp) => {
                let val = self.eval(*exp);
                self.env.insert(id, val);
                Val::UnitVal
            }
            Exp::VarExp(id) => {
                // Copy map, remove entry to gain ownership.
                let temp_map = &mut self.env; //Maybe find better way than map copy
                temp_map.remove(&id).unwrap()
            }
            Exp::SeqExp(expressions) => {
                let mut result = Val::UnitVal; // If empty, return unit
                for expr in expressions {
                    result = self.eval(expr); // eval each expression, possibly updating the environment. Potential optimizer to only safe last...
                }
                return result;
            }
            _ => Val::Undefined,
        };
    }
}
