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

    // Default value is 0 - either this or error depending on what we want.
    fn get_or_else(&mut self, key: Id) -> Val {
        let mut temp_map = self.env.clone();
        return match temp_map.remove(&key) {
            Some(val) => val,
            None => Val::IntVal(0),
        };
    }

    pub fn eval(&mut self, e: Exp) -> Val {
        // Debugging
        println!("Program is: ");
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
                    BinOp::EqualBinOp => match (left, right) {
                        (Val::IntVal(v1), Val::IntVal(v2)) => Val::BoolVal(v1 == v2),
                        (Val::BoolVal(v1), Val::BoolVal(v2)) => Val::BoolVal(v1 == v2),
                        _ => panic!("Error for equals"),
                    },
                    BinOp::NotEqualBinOp => match (left, right) {
                        (Val::IntVal(v1), Val::IntVal(v2)) => Val::BoolVal(v1 != v2),
                        (Val::BoolVal(v1), Val::BoolVal(v2)) => Val::BoolVal(v1 != v2),
                        _ => panic!("Error for not equals"),
                    },
                    BinOp::GreaterThanBinOp => match (left, right) {
                        (Val::IntVal(v1), Val::IntVal(v2)) => Val::BoolVal(v1 > v2),
                        _ => panic!("Error for >"),
                    },
                    BinOp::GreaterThanEqualBinOp => match (left, right) {
                        (Val::IntVal(v1), Val::IntVal(v2)) => Val::BoolVal(v1 >= v2),
                        _ => panic!("Error for >="),
                    },
                    BinOp::LessThanBinOp => match (left, right) {
                        (Val::IntVal(v1), Val::IntVal(v2)) => Val::BoolVal(v1 < v2),
                        _ => panic!("Error for <"),
                    },
                    BinOp::LessThenEqualBinOp => match (left, right) {
                        (Val::IntVal(v1), Val::IntVal(v2)) => Val::BoolVal(v1 <= v2),
                        _ => panic!("Error for <="),
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
                self.get_or_else(id) // Default value for undeclared is 0.
            }
            Exp::SeqExp(expressions) => {
                let mut result = Val::UnitVal; // If empty, return unit
                for expr in expressions {
                    result = self.eval(expr); // eval each expression, possibly updating the environment. Potential optimizer to only save last...
                }
                return result;
            }
            Exp::IfExp(g, thn, els) => {
                let guard = self.eval(*g);
                let val = match guard {
                    Val::BoolVal(b) => b,
                    _ => panic!("Guard was not a boolean value"),
                };
                // Return the expression in the appropiate branch
                return if val {
                    self.eval(*thn)
                } else {
                    self.eval(*els)
                };
            }
            Exp::UnitExp => Val::UnitVal, //_ => Val::Undefined,
        };
    }
}
