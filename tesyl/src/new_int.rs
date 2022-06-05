use crate::Val::{BoolVal, IntVal};
use crate::{BinOp, Exp, Val};
use std::collections::HashMap;
use std::iter::zip;

/*
   This MR: Try and restart on the interpreter, and make the struct own the maps, and
   avoid cloning so much. Hopefully that fixes memory issues

   Maybe also: The Box of expressions - if we * that, we get it to the stack...
   Might want to keep it on the heap?

*/
type Id = String;
type VarEnv = HashMap<Id, Val>;
type FunEnv = HashMap<Id, Val>;

pub struct InterpreterNew {}

impl InterpreterNew {
    pub fn new() -> InterpreterNew {
        InterpreterNew {}
    }

    pub fn eval(&mut self, e: Exp, var_env: &mut VarEnv) -> Val {
        match e {
            Exp::IntExp(v) => IntVal(v),
            Exp::BoolExp(b) => BoolVal(b),
            Exp::BinOpExp(left_expr, op, right_expr) => {
                let left = self.eval(*left_expr, var_env);
                let right = self.eval(*right_expr, var_env);
                match op {
                    BinOp::PlusBinOp => match (left, right) {
                        (IntVal(v1), IntVal(v2)) => IntVal(v1 + v2),
                        _ => panic!("Not implemented"),
                    },
                    BinOp::LessThanBinOp => match (left, right) {
                        (IntVal(v1), IntVal(v2)) => BoolVal(v1 < v2),
                        _ => panic!("Incomparable types used for boolean comparison <"),
                    },
                    _ => panic!("Not implemented"),
                }
            }

            Exp::LetExp(id, exp) => {
                let val = self.eval(*exp, var_env);
                var_env.insert(id, val);
                Val::UnitVal
            }
            Exp::VarExp(id) => var_env.get(&id).unwrap().clone(),
            Exp::AssignmentExp(id, exp) => {
                let res = self.eval(*exp, var_env);
                //assert!(self.var_env.get(&id), Some(v), "error for not declared val");
                var_env.insert(id, res);
                Val::UnitVal
            }

            Exp::SeqExp(expressions) => {
                let mut result = Val::UnitVal;
                for expr in expressions {
                    let res = self.eval(expr, var_env);
                    result = res;
                }
                result
            }

            Exp::IfExp(g, thn, els) => {
                let eval_guard = self.eval(*g, var_env);
                let val = match eval_guard {
                    BoolVal(b) => b,
                    _ => panic!("Expected guard to be a boolean"),
                };

                return if val {
                    self.eval(*thn, var_env)
                } else {
                    self.eval(*els, var_env)
                };
            }

            // TODO: Might wanna look into this mess
            Exp::WhileExp(ref guard, ref body) => {
                let tmp_guard = guard.clone();
                let eval_guard = self.eval(*tmp_guard, var_env);
                match eval_guard {
                    BoolVal(false) => Val::UnitVal,
                    BoolVal(true) => {
                        let tmp_body = body.clone();
                        let eval_body = self.eval(*tmp_body, var_env);
                        self.eval(e, var_env) // placeholder for testing
                    }
                    _ => panic!("Expected guard to be a boolean value"),
                }
            }
            _ => Val::UnitVal,
        }
    }
}
/*
Exp::IfExp(g, thn, els) => {
                println!(
                    "In If Statement: Values are:\n Guard: {},\n Then: {},\n Else: {},\n",
                    g, thn, els
                );

                println!("Got to the if, with var env {:?}\n", var_env);
                // This gets false, i.e. n <= 1 gets false, even though n --> 1. Error in BinOp for <=?
                let (guard, venv, fenv) = self.eval(*g, var_env, fun_env);
                println!("Guard: {},\n venv: {:?},\nfenv: {:?}", guard, venv, fenv);
                let val = match guard {
                    Val::BoolVal(b) => b,
                    _ => panic!("Expected guard to be a boolean value for if statement"),
                };
                // Return the expression in the appropiate branch
                return if val {
                    self.eval(*thn, venv, fenv)
                } else {
                    self.eval(*els, venv, fenv)
                };
            }
 */
