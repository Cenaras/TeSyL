//Possible values of program execution
#[derive(PartialEq, Clone)]
pub enum Val {
    IntVal(i32),
    BoolVal(bool),
    UnitVal,
}

use std::fmt::{self};
impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Val::IntVal(v) => write!(f, "{}", v),
            Val::UnitVal => write!(f, "()"),
            Val::BoolVal(b) => write!(f, "{}", b),
            _ => write!(f, "Value not implemented "),
        }
    }
}
