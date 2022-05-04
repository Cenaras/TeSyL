use core::panic;
use std::collections::HashMap;

use crate::ast::BinOp;
use crate::print_program;
use crate::val::Val;
use crate::Exp;

// Map identifiers to their values
type Id = String;
type VarEnv = HashMap<Id, Val>;
type FunEnv = HashMap<Id, Val>;

pub struct Interpreter {
    var_env: VarEnv,
    fun_env: FunEnv,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let var_map: HashMap<Id, Val> = HashMap::new();
        let fun_map: HashMap<Id, Val> = HashMap::new();
        Interpreter { var_env: var_map, fun_env: fun_map }
    }

    // Default value is error
    fn get_or_else(&mut self, key: Id) -> Val {
        let mut temp_map = self.var_env.clone();
        return match temp_map.remove(&key) {
            Some(val) => val,
            None => panic!("{} is not declared with a let", key),
        };
    }

    // Potentially pass environments with eval.
    pub fn eval(&mut self, e: Exp) -> Val {
        // Debugging
        print_program(&e);

        // Match top level expression and recursively compute sub terms.
        return match e {
            Exp::IntExp(v) => Val::IntVal(v),
            Exp::BoolExp(b) => Val::BoolVal(b),
            Exp::BinOpExp(left, op, right) => {
                let left = self.eval(*left);
                let right = self.eval(*right);
                match op {
                    BinOp::PlusBinOp => match (left, right) {
                        (Val::IntVal(v1), Val::IntVal(v2)) => Val::IntVal(v1 + v2),
                        (Val::TupleVal(v1, v2), Val::TupleVal(v3, v4)) => {
                            match (*v1, *v2, *v3, *v4) { // Only support for (a, b) + (c, b) for a, b, c, d being ints.
                                (
                                    Val::IntVal(val1),
                                    Val::IntVal(val2),
                                    Val::IntVal(val3),
                                    Val::IntVal(val4),
                                ) => Val::TupleVal(
                                    Box::new(Val::IntVal(val1 + val3)),
                                    Box::new(Val::IntVal(val2 + val4)),
                                ),
                                _ => panic!("Error"),
                            }
                        }
                        _ => {
                            panic!("Expected two integers for plus")
                        }
                    },
                    BinOp::MinusBinOp => match (left, right) {
                        (Val::IntVal(v1), Val::IntVal(v2)) => Val::IntVal(v1 - v2),
                        _ => {
                            panic!("Expected two integers for minus")
                        }
                    },
                    BinOp::TimesBinOp => match (left, right) {
                        (Val::IntVal(v1), Val::IntVal(v2)) => Val::IntVal(v1 * v2),
                        _ => {
                            panic!("Expected two integers for multiplication")
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
                            panic!("Expected two integers for division")
                        }
                    },
                    BinOp::EqualBinOp => match (left, right) {
                        (Val::IntVal(v1), Val::IntVal(v2)) => Val::BoolVal(v1 == v2),
                        (Val::BoolVal(v1), Val::BoolVal(v2)) => Val::BoolVal(v1 == v2),
                        _ => panic!("Incomparable types used for boolean equality"),
                    },
                    BinOp::NotEqualBinOp => match (left, right) {
                        (Val::IntVal(v1), Val::IntVal(v2)) => Val::BoolVal(v1 != v2),
                        (Val::BoolVal(v1), Val::BoolVal(v2)) => Val::BoolVal(v1 != v2),
                        _ => panic!("Incomparable types used for boolean equality"),
                    },
                    BinOp::GreaterThanBinOp => match (left, right) {
                        (Val::IntVal(v1), Val::IntVal(v2)) => Val::BoolVal(v1 > v2),
                        _ => panic!("Incomparable types used for boolean comparision >"),
                    },
                    BinOp::GreaterThanEqualBinOp => match (left, right) {
                        (Val::IntVal(v1), Val::IntVal(v2)) => Val::BoolVal(v1 >= v2),
                        _ => panic!("Incomparable types used for boolean comparision >="),
                    },
                    BinOp::LessThanBinOp => match (left, right) {
                        (Val::IntVal(v1), Val::IntVal(v2)) => Val::BoolVal(v1 < v2),
                        _ => panic!("Incomparable types used for boolean comparision <"),
                    },
                    BinOp::LessThenEqualBinOp => match (left, right) {
                        (Val::IntVal(v1), Val::IntVal(v2)) => Val::BoolVal(v1 <= v2),
                        _ => panic!("Incomparable types used for boolean comparision <="),
                    },
                }
            }
            // Update environment. LetExp returns Unit
            Exp::LetExp(id, exp) => {
                let val = self.eval(*exp);
                self.var_env.insert(id, val);
                Val::UnitVal
            }
            Exp::AssignmentExp(id, expr) => {
                // Check if defined.
                let temp_id = id.clone();
                self.get_or_else(temp_id);
                let val = self.eval(*expr);
                self.var_env.insert(id, val);
                Val::UnitVal
            }
            // Require defined variaible or throw error.
            Exp::VarExp(id) => self.get_or_else(id),
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
                    _ => panic!("Expected guard to be a boolean value for if statement"),
                };
                // Return the expression in the appropiate branch
                return if val {
                    self.eval(*thn)
                } else {
                    self.eval(*els)
                };
            }
            // If false, return unit. Else eval body and eval while again. Need temps to satisfy safety
            Exp::WhileExp(ref g, ref b) => {
                let temp_guard = g.clone();
                let guard = self.eval(*temp_guard);
                match guard {
                    Val::BoolVal(false) => return Val::UnitVal,
                    Val::BoolVal(true) => {
                        let temp_body = b.clone();
                        let body = self.eval(*temp_body);
                        self.eval(e)
                    }
                    _ => panic!("Expected guard to be a boolean value for while loop"),
                }
            }
            // TODO: TupleAccess expression
            Exp::TupleExp(v1, v2) => {
                let first = self.eval(*v1);
                let second = self.eval(*v2);
                Val::TupleVal(Box::new(first), Box::new(second))
            }
            Exp::FunDefExp(id, args, body) => {
                let cur_var_env = self.var_env.clone();
                let cur_fun_env = self.fun_env.clone();
                let closure = Val::ClosureVal(args, body, cur_var_env, cur_fun_env);
                
                self.fun_env.insert(id, closure);
                Val::UnitVal
            }
            Exp::UnitExp => Val::UnitVal, //_ => Val::Undefined,
        };
    }
}
