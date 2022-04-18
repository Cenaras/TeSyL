#[derive(PartialEq)]
pub enum Token {
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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Token::PLUS => write!(f, " [PLUS] "),
            Token::MINUS => write!(f, " [MINUS] "),
            Token::IntLit(val) => write!(f, "[IntLit({})]", val),
            Token::EOF => write!(f, "[EOF]"),
            Token::PLACEHOLDER_TYPE => write!(f, "_"),
            _ => write!(f, "Not Implemented"),
        }
    }
}
