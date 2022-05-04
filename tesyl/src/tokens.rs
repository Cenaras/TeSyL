#[derive(PartialEq)]
pub enum Token {
    PLUS,
    MINUS,
    TIMES,
    DIVIDE,
    GE,
    GEQ,
    LE,
    LEQ,
    EQUAL,
    NEQ,
    NOT,
    SEMICOLON,
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

/*pub fn bin_op_precedence(op: &Token) -> i32 {
    match op {
        Token::PLUS | Token::MINUS => 10,
        Token::TIMES | Token::DIVIDE => 20,
        _ => -1,
    }
}*/

use std::fmt::{self};

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Token::PLUS => write!(f, "[PLUS] "),
            Token::MINUS => write!(f, "[MINUS] "),
            Token::TIMES => write!(f, "[TIMES] "),
            Token::DIVIDE => write!(f, "[DIVIDE] "),
            Token::GE => write!(f, "[GE] "),
            Token::GEQ => write!(f, "[GEQ] "),
            Token::LE => write!(f, "[LE] "),
            Token::LEQ => write!(f, "[LEQ] "),
            Token::EQUAL => write!(f, "[EQUAL] "),
            Token::NEQ => write!(f, "[NEQ] "),
            Token::NOT => write!(f, "[NOT] "),
            Token::SEMICOLON => write!(f, "[SEMICOLON] "),
            Token::IntLit(val) => write!(f, "[IntLit({})] ", val),
            Token::Identifier(name) => write!(f, "[Identifier({})] ", name),
            Token::EOF => write!(f, "[EOF] "),
            Token::LET => write!(f, "[LET] "),
            Token::IF => write!(f, "[IF] "),
            Token::THEN => write!(f, "[THEN] "),
            Token::ELSE => write!(f, "[ELSE] "),
            Token::WHILE => write!(f, "[WHILE] "),
            Token::TRUE => write!(f, "[TRUE] "),
            Token::FALSE => write!(f, "[FALSE] "),
            Token::OpenParen => write!(f, "[OPENPAREN] "),
            Token::CloseParen => write!(f, "[CLOSEPAREN] "),
            Token::OpenBrack => write!(f, "[OPENBRACK] "),
            Token::CloseBrack => write!(f, "[CLOSEBRACK] "),
            Token::COMMA => write!(f, "[COMMA] "),
            Token::FUNDEC => write!(f, "[DEF] "),

            _ => write!(f, "Not Implemented "),
        }
    }
}
