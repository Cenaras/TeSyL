#![allow(unused_parens)]
#![allow(unused_variables)]

use crate::tokens::Token; //add ::* and remove TOKEN::?
use std::{collections::HashMap, iter::Peekable, str::Chars};

// TODO: Store Iterator instead of raw data, every method using iter is then in impl Lexer.

// Lexer stores the raw file data
pub struct Lexer {
    pub raw: String,
}

// Read file on creation

impl Lexer {
    // Init new lexer, return error if file not present
    pub fn new(program: String) -> Result<Lexer, std::io::Error> {
        let contents = std::fs::read_to_string(format!(".\\samples\\{}", program))?;
        Ok(Lexer { raw: contents })
    }

    pub fn real(program: String) -> Result<Lexer, std::io::Error> {
        let contents = std::fs::read_to_string(format!(".\\programs\\{}", program))?;
        Ok(Lexer { raw: contents })
    }

    pub fn lex(&self) -> Vec<Token> {
        let mut tokens_list: Vec<Token> = Vec::new();
        let mut iter = self.raw.chars().into_iter().peekable();

        // TODO: Create vec of keywords so we know if token is keyword or not.

        // Iterate over everything and create tokens

        let mut next_token = get_token(&mut iter);

        loop {
            if next_token != Token::EOF {
                tokens_list.push(next_token);
                next_token = get_token(&mut iter);
            } else {
                tokens_list.push(Token::EOF);
                break;
            }
        }

        return tokens_list;
    }
}

// Parses a token. Skips whitespace, and delegates complicated parsing to other functions
fn get_token(iter: &mut Peekable<Chars>) -> Token {
    // Code is not very well written. Benchmark bad lexer, return and optimize to see difference.

    // Peek next: If None, return EOF, else lex next char
    while (iter.peek() != None) {
        let mut cur = iter.next().unwrap();

        // Disregard whitespaces - if last is whitespace, return
        while (skipable(&cur)) {
            if (iter.peek() != None) {
                cur = iter.next().unwrap();
            } else {
                return Token::EOF;
            }
        }

        // If we see number, keep consuming numbers.
        if (cur.is_numeric()) {
            return read_number(cur, iter);

        // Parse identifier here
        } else if (cur.is_alphanumeric()) {
            return read_identifier(cur, iter);
        } else {
            return match cur {
                '+' => Token::PLUS,
                '-' => Token::MINUS,
                '*' => Token::TIMES,
                '/' => Token::DIVIDE,
                ';' => Token::SEMICOLON,
                '>' => {
                    match iter.peek() {
                        Some('=') => {
                            iter.next(); // consume the =
                            return Token::GEQ;
                        }
                        _ => return Token::GE,
                    }
                }
                '<' => {
                    match iter.peek() {
                        Some('=') => {
                            iter.next(); // consume the =
                            return Token::LEQ;
                        }
                        _ => return Token::LE,
                    }
                }
                '=' => Token::EQUAL,
                '!' => {
                    match iter.peek() {
                        Some('=') => {
                            iter.next(); // consume the =
                            return Token::NEQ;
                        }
                        _ => return Token::NOT,
                    }
                }
                '(' => Token::OpenParen,
                ')' => Token::CloseParen,
                '{' => Token::OpenBrack,
                '}' => Token::CloseBrack,
                _ => Token::INVALID,
            };
        }
    }
    return Token::EOF;
}

// Determine if current char is skipable
fn skipable(c: &char) -> bool {
    if c.is_whitespace() {
        return true;
    }
    return false;
}

/* ##### Create pretty way to avoid code duplication with Option vs unwrapped value for these ##### */

fn is_digit(c: Option<&char>) -> bool {
    return match c {
        None => false,
        Some(i) => i.is_numeric(),
    };
}

fn is_identifier_symbol(c: Option<&char>) -> bool {
    return match c {
        None => false,
        Some(val) => val.is_alphabetic() || val.is_numeric(),
    };
}

fn read_number(cur: char, iter: &mut Peekable<Chars>) -> Token {
    let mut acc = String::from(cur);

    while (is_digit(iter.peek())) {
        let val = iter.next().unwrap();
        if (val.is_numeric()) {
            acc.push(val)
        }
    }
    // acc is always a valid formed number, so unwrap is safe
    return Token::IntLit(acc.parse().unwrap());
}

fn read_identifier(cur: char, iter: &mut Peekable<Chars>) -> Token {
    let mut acc = String::from(cur);

    //Benchmark creating everytime vs only once - bad, but on purpose for benchmarking
    let mut keywords = HashMap::from([
        (String::from("let"), Token::LET),
        (String::from("if"), Token::IF),
        (String::from("then"), Token::THEN),
        (String::from("else"), Token::ELSE),
        (String::from("true"), Token::TRUE),
        (String::from("false"), Token::FALSE),
        (String::from("while"), Token::WHILE),
    ]);

    while (is_identifier_symbol(iter.peek())) {
        let next = iter.next().unwrap();
        if (next.is_alphabetic() || next.is_numeric()) {
            acc.push(next);
        }
    }

    // Creating every time and removing entry - benchmark property
    return match keywords.remove(&acc) {
        Some(token) => token,
        None => Token::Identifier(acc),
    };
}
