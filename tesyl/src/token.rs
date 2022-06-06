use crate::position::Position;

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
}

pub struct Token {
    token: TokenType,
    pos: Position,
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
