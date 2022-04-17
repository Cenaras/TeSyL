pub enum Tokens {
    PLUS,
    MINUS,
    TIMES,
    DIVIDE,
    GE,
    LE,
    EQUAL,
    SEMICOLON,
    IntLit(i32),
    EOF,
}

use std::fmt::{self, write};


impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Tokens::PLUS => write!(f, " [PLUS] "),
            Tokens::MINUS => write!(f, " [MINUS] "),
            Tokens::IntLit(val) => write!(f, "[IntLit({})]", val),
            Tokens::EOF => write!(f, "[EOF]"),
            _ => write!(f, "Not Implemented"),
        }
    }
}