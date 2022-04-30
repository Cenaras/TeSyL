//Possible values of program execution
#[derive(PartialEq)]
pub enum Val {
    IntVal(u32),
    UnitVal,
    //BoolVal(bool),
    Undefined,
}

use std::fmt::{self};
impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Val::IntVal(v) => write!(f, "{}", v),
            Val::UnitVal => write!(f, "()"),
            _ => write!(f, "Value not implemented "),
        }
    }
}
