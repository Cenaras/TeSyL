#[derive(PartialEq)]
pub enum Tokens {
    PLUS,
    MINUS,
    TIMES,
    DIVIDE,
    GE,
    LE,
    EQUAL,
    SEMICOLON,
    IntLit(u32),
    EOF,
    PLACEHOLDER_TYPE,
}

use std::fmt::{self};

impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Tokens::PLUS => write!(f, " [PLUS] "),
            Tokens::MINUS => write!(f, " [MINUS] "),
            Tokens::IntLit(val) => write!(f, "[IntLit({})]", val),
            Tokens::EOF => write!(f, "[EOF]"),
            Tokens::PLACEHOLDER_TYPE => write!(f, "_"),
            _ => write!(f, "Not Implemented"),
        }
    }
}
