use crate::Val::{BoolVal, IntVal};
use crate::{BinOp, Exp, Val};
use std::collections::HashMap;

/*
   This MR: Try and restart on the interpreter, and make the struct own the maps, and
   avoid cloning so much. Hopefully that fixes memory issues

   Maybe also: The Box of expressions - if we * that, we get it to the stack...
   Might want to keep it on the heap?

*/
type Id = String;
type VarEnv = HashMap<Id, Val>;
type FunEnv = HashMap<Id, Val>;

pub struct InterpreterNew {
    var_env: VarEnv,
    fun_env: FunEnv,
}

impl InterpreterNew {
    pub fn new() -> InterpreterNew {
        InterpreterNew {
            var_env: HashMap::new(),
            fun_env: HashMap::new(),
        }
    }

    pub fn eval(&mut self, e: Exp) -> Val {
        match e {
            Exp::IntExp(v) => IntVal(v),
            Exp::BoolExp(b) => BoolVal(b),
            Exp::BinOpExp(left_expr, op, right_expr) => {
                let left = self.eval(*left_expr);
                let right = self.eval(*right_expr);
                match op {
                    BinOp::PlusBinOp => match (left, right) {
                        (IntVal(v1), IntVal(v2)) => IntVal(v1 + v2),
                        _ => panic!("Not implemented"),
                    },
                    BinOp::LessThanBinOp => match (left, right) {
                        (IntVal(v1), IntVal(v2)) => BoolVal(v1 < v2),
                        _ => panic!("Incomparable types used for boolean comparison <"),
                    }
                    _ => panic!("Not implemented"),
                }
            }

            Exp::LetExp(id, exp) => {
                let val = self.eval(*exp);
                self.var_env.insert(id, val);
                Val::UnitVal
            }
            Exp::VarExp(id) => self.var_env.get(&id).unwrap().clone(),
            Exp::AssignmentExp(id, exp) => {
                let res = self.eval(*exp);
                //assert!(self.var_env.get(&id), Some(v), "error for not declared val");
                self.var_env.insert(id, res);
                Val::UnitVal
            }

            Exp::SeqExp(expressions) => {
                let mut result = Val::UnitVal;
                for expr in expressions {
                    let res = self.eval(expr);
                    result = res;
                }
                result
            }

            // TODO: Might wanna look into this mess
            Exp::WhileExp(ref guard, ref body) => {
                let tmp_guard = guard.clone();
                let eval_guard = self.eval(*tmp_guard);
                match eval_guard {
                    BoolVal(false) => Val::UnitVal,
                    BoolVal(true) => {
                        let tmp_body = body.clone();
                        let eval_body = self.eval(*tmp_body);
                        self.eval(e) // placeholder for testing
                    }
                    _ => panic!("Expected guard to be a boolean value"),
                }
            }

            _ => Val::UnitVal,
        }
    }
}
/*
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
 */
