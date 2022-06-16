use crate::position::Position;
use std::fmt::{write, Display, Formatter};

#[derive(PartialEq, Debug)]
pub enum TokenType {
    PLUS,
    MINUS,
    TIMES,
    DIVIDE,
    GE,
    GEQ,
    LE,
    LEQ,
    EQUAL,
    EQEQ,
    NEQ,
    NOT,
    SEMICOLON,
    IntLit(i64),
    Identifier(String),
    LET,
    OpenParen,
    CloseParen,
    OpenBrack,
    CloseBrack,
    COMMA,
    INVALID,
    EOF,
}

pub struct Token {
    token: TokenType,
    pos: Position,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::PLUS => write!(f, "[PLUS] "),
            TokenType::MINUS => write!(f, "[MINUS] "),
            TokenType::TIMES => write!(f, "[TIMES] "),
            TokenType::DIVIDE => write!(f, "[DIVIDE] "),
            TokenType::SEMICOLON => write!(f, "[SEMICOLON] "),
            TokenType::GE => write!(f, "[GE] "),
            TokenType::GEQ => write!(f, "[GEQ] "),
            TokenType::LE => write!(f, "[LE] "),
            TokenType::LEQ => write!(f, "[LEQ] "),
            TokenType::EQUAL => write!(f, "[EQUAL] "),
            TokenType::IntLit(val) => write!(f, "[IntLit({})] ", val),
            TokenType::Identifier(name) => write!(f, "[Identifier({})] ", name),
            TokenType::LET => write!(f, "[LET] "),
            TokenType::OpenParen => write!(f, "[OPENPAREN] "),
            TokenType::CloseParen => write!(f, "[CLOSEPAREN] "),
            TokenType::OpenBrack => write!(f, "[OPENBRACK] "),
            TokenType::CloseBrack => write!(f, "[CLOSEBRACK] "),
            TokenType::EOF => write!(f, "[EOF] "),
            _ => write!(f, "Not Implemented "),
        }
    }
}

/*
pub enum Token {
    IntLit(i32),
    Identifier(String),
    EOF,
    LET,
    IF,
    THEN,
    ELSE,
    TRUE,
    FALSE,
    WHILE,
    OpenParen,
    CloseParen,
    OpenBrack,
    CloseBrack,
    COMMA,
    FUNDEC,
    INVALID,
}
 */
