use crate::Val::{BoolVal, IntVal};
use crate::{BinOp, Exp, Val};
use std::collections::HashMap;
use std::env::var;
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

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    pub fn eval(&mut self, e: Exp, var_env: &mut VarEnv, fun_env: &mut FunEnv) -> Val {
        match e {
            Exp::IntExp(v) => IntVal(v),
            Exp::BoolExp(b) => BoolVal(b),
            Exp::BinOpExp(left_expr, op, right_expr) => {
                let left = self.eval(*left_expr, var_env, fun_env);
                let right = self.eval(*right_expr, var_env, fun_env);
                match op {
                    BinOp::PlusBinOp => match (left, right) {
                        (IntVal(v1), IntVal(v2)) => IntVal(v1 + v2),
                        _ => panic!("Not implemented"),
                    },
                    BinOp::MinusBinOp => match (left, right) {
                        (IntVal(v1), (IntVal(v2))) => IntVal(v1 - v2),
                        _ => panic!("Expect ints"),
                    },
                    BinOp::TimesBinOp => match (left, right) {
                        (IntVal(v1), (IntVal(v2))) => IntVal(v1 * v2),
                        _ => panic!("Expect ints"),
                    },
                    BinOp::DivideBinOp => match (left, right) {
                        (IntVal(v1), (IntVal(v2))) => {
                            if v2 == 0 {
                                panic!("Div by 0");
                            }
                            IntVal(v1 / v2)
                        }
                        _ => panic!("Expect ints"),
                    },
                    BinOp::EqualBinOp => match (left, right) {
                        (IntVal(v1), (IntVal(v2))) => BoolVal(v1 == v2),
                        (BoolVal(b1), BoolVal(b2)) => BoolVal(b1 == b2),
                        _ => panic!("Expect ints"),
                    },
                    BinOp::NotEqualBinOp => match (left, right) {
                        (IntVal(v1), (IntVal(v2))) => BoolVal(v1 != v2),
                        (BoolVal(b1), BoolVal(b2)) => BoolVal(b1 != b2),
                        _ => panic!("Expect ints"),
                    },
                    BinOp::LessThanBinOp => match (left, right) {
                        (IntVal(v1), IntVal(v2)) => BoolVal(v1 < v2),
                        _ => panic!("Incomparable types used for boolean comparison <"),
                    },
                    BinOp::LessThenEqualBinOp => match (left, right) {
                        (IntVal(v1), IntVal(v2)) => BoolVal(v1 <= v2),
                        _ => panic!("Incomparable types used for boolean comparison <="),
                    },
                    BinOp::GreaterThanBinOp => match (left, right) {
                        (IntVal(v1), IntVal(v2)) => BoolVal(v1 > v2),
                        _ => panic!("Incomparable types used for boolean comparison >"),
                    },
                    BinOp::GreaterThanEqualBinOp => match (left, right) {
                        (IntVal(v1), IntVal(v2)) => BoolVal(v1 >= v2),
                        _ => panic!("Incomparable types used for boolean comparison >="),
                    },
                    _ => panic!("Not implemented"),
                }
            }

            Exp::LetExp(id, exp) => {
                let val = self.eval(*exp, var_env, fun_env);
                var_env.insert(id, val);
                Val::UnitVal
            }
            Exp::VarExp(id) => var_env.get(&id).unwrap().clone(),
            Exp::AssignmentExp(id, exp) => {
                let res = self.eval(*exp, var_env, fun_env);
                //assert!(self.var_env.get(&id), Some(v), "error for not declared val");
                var_env.insert(id, res);
                Val::UnitVal
            }

            Exp::SeqExp(expressions) => {
                let mut result = Val::UnitVal;
                for expr in expressions {
                    let res = self.eval(expr, var_env, fun_env);
                    result = res;
                }
                result
            }

            Exp::IfExp(g, thn, els) => {
                let eval_guard = self.eval(*g, var_env, fun_env);
                let val = match eval_guard {
                    BoolVal(b) => b,
                    _ => panic!("Expected guard to be a boolean"),
                };

                return if val {
                    self.eval(*thn, var_env, fun_env)
                } else {
                    self.eval(*els, var_env, fun_env)
                };
            }
            Exp::TupleExp(v1, v2) => {
                let first = self.eval(*v1, var_env, fun_env);
                let second = self.eval(*v2, var_env, fun_env);
                Val::TupleVal(Box::new(first), Box::new(second))
            }

            // TODO: Might wanna look into this mess
            Exp::WhileExp(ref guard, ref body) => {
                let tmp_guard = guard.clone();
                let eval_guard = self.eval(*tmp_guard, var_env, fun_env);
                match eval_guard {
                    BoolVal(false) => Val::UnitVal,
                    BoolVal(true) => {
                        let tmp_body = body.clone();
                        let eval_body = self.eval(*tmp_body, var_env, fun_env);
                        self.eval(e, var_env, fun_env) // placeholder for testing
                    }
                    _ => panic!("Expected guard to be a boolean value"),
                }
            }
            Exp::FunDefExp(id, args, body) => {
                let cur_venv = var_env.clone();
                let cur_fenv = fun_env.clone();
                let closure = Val::ClosureVal(args, body, cur_venv, cur_fenv);
                fun_env.insert(id, closure);
                Val::UnitVal
            }

            //TODO: Clean this up, copied from old impl
            Exp::CallExp(fun_id, args) => {
                let tmp_fenv = fun_env.clone();
                let closure = match tmp_fenv.get(&fun_id).unwrap() {
                    Val::ClosureVal(params, body, venv, fenv) => (params, body, venv, fenv),
                    _ => panic!("Error"),
                };

                if args.len() != closure.0.len() {
                    panic!("Error")
                }
                let mut eval_args: Vec<Val> = vec![];
                for arg in args {
                    let e = self.eval(arg, var_env, fun_env);
                    eval_args.push(e);
                }
                eval_args.reverse();

                let fun_params = closure.0.clone();
                let mut loc_venv = var_env.clone();
                let mut loc_fenv = fun_env.clone();

                // Update local vars with new bindings from params to eval'd exps.
                for (param_id, arg) in zip(fun_params, eval_args) {
                    loc_venv.insert(param_id, arg);
                }
                let body = closure.1.clone();
                loc_fenv.insert(
                    fun_id,
                    Val::ClosureVal(
                        closure.0.clone(),
                        closure.1.clone(),
                        closure.2.clone(),
                        closure.3.clone(),
                    ),
                );

                let res = self.eval(*body, &mut loc_venv, &mut loc_fenv);
                res
            }
            Exp::UnitExp => Val::UnitVal,
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
