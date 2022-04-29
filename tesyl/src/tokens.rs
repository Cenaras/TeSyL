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
    Identifier(String),
    EOF,
    LET,
    IF,
    THEN,
    ELSE,
    OpenParen,
    CloseParen,
    INVALID,
}

/*
        precedence = new HashMap<>();
        precedence.put(TokenType.PLUS, 1);
        precedence.put(TokenType.MINUS, 1);
        precedence.put(TokenType.TIMES, 2);
        precedence.put(TokenType.DIVIDE, 2);

*/

pub fn bin_op_precedence(op: &Token) -> i32 {
    match op {
        Token::PLUS | Token::MINUS => 10,
        Token::TIMES | Token::DIVIDE => 20,
        _ => -1,
    }
}

use std::fmt::{self};

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Token::PLUS => write!(f, "[PLUS] "),
            Token::MINUS => write!(f, "[MINUS] "),
            Token::TIMES => write!(f, "[TIMES] "),
            Token::DIVIDE => write!(f, "[DIVIDE] "),
            Token::GE => write!(f, "[GE] "),
            Token::LE => write!(f, "[LE] "),
            Token::EQUAL => write!(f, "[EQUAL] "),
            Token::SEMICOLON => write!(f, "[SEMICOLON] "),
            Token::IntLit(val) => write!(f, "[IntLit({})] ", val),
            Token::Identifier(name) => write!(f, "[IDENTIFIER({})] ", name),
            Token::EOF => write!(f, "[EOF] "),
            Token::LET => write!(f, "[LET] "),
            Token::IF => write!(f, "[IF] "),
            Token::THEN => write!(f, "[THEN] "),
            Token::ELSE => write!(f, "[ELSE] "),
            Token::OpenParen => write!(f, "( "),
            Token::CloseParen => write!(f, ") "),
            Token::INVALID => write!(f, "[INVALID] "),
            _ => write!(f, "Not Implemented "),
        }
    }
}
