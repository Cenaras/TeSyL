//Possible values of program execution


use crate::Exp;
use std::collections::HashMap;
type Id = String;
type VarEnv = HashMap<Id, Val>;
type FunEnv = HashMap<Id, Val>;

#[derive(PartialEq, Clone)]
pub enum Val {
    IntVal(i32),
    BoolVal(bool),
    TupleVal(Box<Val>, Box<Val>),
    ClosureVal(Vec<Id>, Box<Exp>, VarEnv, FunEnv),
    UnitVal,
}

use std::fmt::{self};
impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Val::IntVal(v) => write!(f, "{}", v),
            Val::UnitVal => write!(f, "()"),
            Val::BoolVal(b) => write!(f, "{}", b),
            Val::TupleVal(v1, v2) => write!(f, "({}, {})", v1, v2),
            _ => write!(f, "Value not implemented "),
        }
    }
}
