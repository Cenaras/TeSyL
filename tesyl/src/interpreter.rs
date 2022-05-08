use crate::ast::BinOp;
use crate::print_program;
use crate::val::Val;
use crate::Exp;
use core::panic;
use std::collections::HashMap;
use std::iter::zip;

// https://stopa.io/post/222
// Probably need environments as references instead to reduce stack

// Map identifiers to their values
type Id = String;
type VarEnv = HashMap<Id, Val>;
type FunEnv = HashMap<Id, Val>;

// TODO: Determine if some sub expressions should actually update environments (e.g. for tuples and others...)
pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    fn get_closure(&mut self, key: Id, map: FunEnv) -> Val {
        let mut tmp_map = map.clone();
        return match tmp_map.remove(&key) {
            Some(cls) => cls,
            None => panic!("{} is not a declared function", key),
        };
    }

    // Potentially pass environments with eval.
    pub fn eval(
        &mut self,
        e: Exp,
        mut var_env: VarEnv,
        mut fun_env: FunEnv,
    ) -> (Val, VarEnv, FunEnv) {
        // Debugging
        //print_program(&e);

        // Match top level expression and recursively compute sub terms.
        return match e {
            Exp::IntExp(v) => (Val::IntVal(v), var_env, fun_env),
            Exp::BoolExp(b) => (Val::BoolVal(b), var_env, fun_env),
            Exp::BinOpExp(left, op, right) => {
                let tmp_venv = var_env.clone();
                let tmp_fenv = fun_env.clone();

                let left = self.eval(*left, var_env, fun_env);

                let right = self.eval(*right, tmp_venv, tmp_fenv);
                match op {
                    BinOp::PlusBinOp => match (left, right) {
                        ((Val::IntVal(v1), _, _), (Val::IntVal(v2), venv, fenv)) => {
                            (Val::IntVal(v1 + v2), venv, fenv)
                        }
                        ((Val::TupleVal(v1, v2), _, _), (Val::TupleVal(v3, v4), venv, fenv)) => {
                            match (*v1, *v2, *v3, *v4) {
                                // Only support for (a, b) + (c, b) for a, b, c, d being ints.
                                (
                                    Val::IntVal(val1),
                                    Val::IntVal(val2),
                                    Val::IntVal(val3),
                                    Val::IntVal(val4),
                                ) => (
                                    Val::TupleVal(
                                        Box::new(Val::IntVal(val1 + val3)),
                                        Box::new(Val::IntVal(val2 + val4)),
                                    ),
                                    venv,
                                    fenv,
                                ),
                                _ => panic!("Error"),
                            }
                        }
                        _ => {
                            panic!("Expected two integers for plus")
                        }
                    },
                    BinOp::MinusBinOp => match (left, right) {
                        ((Val::IntVal(v1), _, _), (Val::IntVal(v2), venv, fenv)) => {
                            (Val::IntVal(v1 - v2), venv, fenv)
                        }
                        _ => {
                            panic!("Expected two integers for minus")
                        }
                    },
                    BinOp::TimesBinOp => match (left, right) {
                        ((Val::IntVal(v1), _, _), (Val::IntVal(v2), venv, fenv)) => {
                            (Val::IntVal(v1 * v2), venv, fenv)
                        }
                        _ => {
                            panic!("Expected two integers for multiplication")
                        }
                    },
                    BinOp::DivideBinOp => match (left, right) {
                        ((Val::IntVal(v1), _, _), (Val::IntVal(v2), venv, fenv)) => {
                            if (v2 == 0) {
                                panic!("Division by 0 error")
                            }
                            (Val::IntVal(v1 / v2), venv, fenv)
                        }
                        _ => {
                            panic!("Expected two integers for division")
                        }
                    },
                    BinOp::EqualBinOp => match (left, right) {
                        ((Val::IntVal(v1), _, _), (Val::IntVal(v2), venv, fenv)) => {
                            (Val::BoolVal(v1 == v2), venv, fenv)
                        }
                        ((Val::BoolVal(v1), _, _), (Val::BoolVal(v2), venv, fenv)) => {
                            (Val::BoolVal(v1 == v2), venv, fenv)
                        }
                        _ => panic!("Incomparable types used for boolean equality"),
                    },
                    BinOp::NotEqualBinOp => match (left, right) {
                        ((Val::IntVal(v1), _, _), (Val::IntVal(v2), venv, fenv)) => {
                            (Val::BoolVal(v1 != v2), venv, fenv)
                        }
                        ((Val::BoolVal(v1), _, _), (Val::BoolVal(v2), venv, fenv)) => {
                            (Val::BoolVal(v1 != v2), venv, fenv)
                        }
                        _ => panic!("Incomparable types used for boolean equality"),
                    },
                    BinOp::GreaterThanBinOp => match (left, right) {
                        ((Val::IntVal(v1), _, _), (Val::IntVal(v2), venv, fenv)) => {
                            (Val::BoolVal(v1 > v2), venv, fenv)
                        }
                        _ => panic!("Incomparable types used for boolean comparision >"),
                    },
                    BinOp::GreaterThanEqualBinOp => match (left, right) {
                        ((Val::IntVal(v1), _, _), (Val::IntVal(v2), venv, fenv)) => {
                            (Val::BoolVal(v1 >= v2), venv, fenv)
                        }
                        _ => panic!("Incomparable types used for boolean comparision >="),
                    },
                    BinOp::LessThanBinOp => match (left, right) {
                        ((Val::IntVal(v1), _, _), (Val::IntVal(v2), venv, fenv)) => {
                            (Val::BoolVal(v1 < v2), venv, fenv)
                        }
                        _ => panic!("Incomparable types used for boolean comparision <"),
                    },
                    BinOp::LessThenEqualBinOp => match (left, right) {
                        ((Val::IntVal(v1), _, _), (Val::IntVal(v2), venv, fenv)) => {
                            (Val::BoolVal(v1 <= v2), venv, fenv)
                        }
                        _ => panic!("Incomparable types used for boolean comparision <="),
                    },
                }
            }
            // Update environment. LetExp returns Unit
            Exp::LetExp(id, exp) => {
                let mut local_venv = var_env.clone();
                let (val, venv, fenv) = self.eval(*exp, var_env, fun_env);
                local_venv.insert(id, val);
                //self.var_env.insert(id, val);
                (Val::UnitVal, local_venv, fenv)
            }
            Exp::AssignmentExp(id, expr) => {
                // Check if defined.
                let temp_id = id.clone();
                let mut local_venv = var_env.clone();
                //self.get_or_else(temp_id);
                if (var_env.get(&id) == None) {
                    panic!("Variable not defined with let binding")
                }
                let (val, venv, fenv) = self.eval(*expr, var_env, fun_env);
                //self.var_env.insert(id, val);
                local_venv.insert(id, val);
                (Val::UnitVal, local_venv, fenv)
            }
            // Require defined variaible or throw error.
            Exp::VarExp(id) => {
                let mut temp_env = var_env.clone();
                let val = temp_env.remove(&id).unwrap();
                (val, var_env, fun_env)
            }
            Exp::SeqExp(expressions) => {
                let mut loc_venv = var_env.clone();
                let mut loc_fenv = fun_env.clone();
                let mut result = Val::UnitVal;

                for expr in expressions {
                    let (res, venv, fenv) = self.eval(expr, loc_venv, loc_fenv);
                    result = res;
                    loc_venv = venv;
                    loc_fenv = fenv;
                }

                /*let mut result = (Val::UnitVal, var_env, fun_env); // If empty, return unit
                for expr in expressions {
                    result = self.eval(expr); // eval each expression, possibly updating the environment. Potential optimizer to only save last...
                }*/

                return (result, loc_venv, loc_fenv);
            }
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
            // If false, return unit. Else eval body and eval while again. Need temps to satisfy safety
            Exp::WhileExp(ref g, ref b) => {
                let temp_guard = g.clone();
                let (guard, venv, fenv) = self.eval(*temp_guard, var_env, fun_env);
                match guard {
                    Val::BoolVal(false) => return (Val::UnitVal, venv, fenv),
                    Val::BoolVal(true) => {
                        let temp_body = b.clone();
                        let (body, venv1, fenv1) = self.eval(*temp_body, venv, fenv);
                        self.eval(e, venv1, fenv1)
                    }
                    _ => panic!("Expected guard to be a boolean value for while loop"),
                }
            }
            // TODO: TupleAccess expression
            Exp::TupleExp(v1, v2) => {
                let (first, v1, f1) = self.eval(*v1, var_env, fun_env);
                let (second, v2, f2) = self.eval(*v2, v1, f1);
                ((Val::TupleVal(Box::new(first), Box::new(second)), v2, f2))
            }
            Exp::FunDefExp(id, args, body) => {
                let cur_var_env = var_env.clone();
                let cur_fun_env = fun_env.clone();
                let mut fenv = cur_fun_env.clone();
                let closure = Val::ClosureVal(args, body, cur_var_env, cur_fun_env);

                //self.fun_env.insert(id, closure);

                fenv.insert(id, closure);

                (Val::UnitVal, var_env, fenv)
            }
            Exp::CallExp(fun_id, args) => {
                let tmp_fun_id = fun_id.clone();
                let tmp_fenv = fun_env.clone();
                println!("Current fun environment is: {:?}\n", fun_env);

                let closure = match self.get_closure(tmp_fun_id, tmp_fenv) {
                    Val::ClosureVal(params, body, var_env, fun_env) => {
                        (params, body, var_env, fun_env)
                    }
                    _ => panic!("Not a closure value"),
                };

                if (args.len() != closure.0.len()) {
                    panic!("Incompatible length of arguments")
                }
                // Eval all args and reverse list to retin order
                let mut eval_args: Vec<Val> = vec![];

                for arg in args {
                    let venv = var_env.clone();
                    let fenv = fun_env.clone();
                    // Dies here
                    let (e, venv, fenv) = self.eval(arg, venv, fenv);
                    eval_args.push(e);
                }
                eval_args.reverse();
                println!("Evaluated arguments in function call: \n{:?}", eval_args);

                let fun_params = closure.0.clone();

                // Update local vars with new bindings from params to eval'd exps.
                for (param_id, arg) in zip(fun_params, eval_args) {
                    var_env.insert(param_id, arg);
                }

                let body = closure.1.clone();

                fun_env.insert(
                    fun_id,
                    Val::ClosureVal(closure.0, closure.1, closure.2, closure.3),
                );

                println!(
                    "Current bindinds in environments are {:?}\n{:?}\n",
                    var_env, fun_env
                );

                // Add environments to eval function, to give new envs to this...
                println!("Evaluating the body: \n{}\n", body);
                return self.eval(*body, var_env, fun_env);

                // Stuff breaks for fibrec because the recursive call updates the "global n"
                // Fixed above - works for fib 9, but fib 10 gives stack overflow. Maybe too many clones...
                // Look into reducing clones - or maybe read on clone to see if it pushes to stack..
            }
            Exp::UnitExp => (Val::UnitVal, var_env, fun_env), //_ => Val::Undefined,
        };
    }
}
