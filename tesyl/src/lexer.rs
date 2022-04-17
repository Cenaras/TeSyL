#![allow(unused_parens)]
#![allow(unused_variables)]

use crate::tokens::Tokens;
use std::str::Chars;

// Lexer stores the raw file data
pub struct Lexer {
    pub raw: String,
}

// Read file on creation

impl Lexer {
    // Init new lexer, return error if file not present
    pub fn new(program: String) -> Result<Lexer, std::io::Error> {
        let contents = std::fs::read_to_string(program)?;
        Ok(Lexer { raw: contents })
    }

    pub fn lex(&self) -> Vec<Tokens> {
        let mut tokens_list: Vec<Tokens> = Vec::new();
        let mut iter = self.raw.chars().into_iter();

        // Iterate over everything and create tokens

        // While iter.next is Some: Parse the value, get the tokens
        // If iter.next = None, get the EOF token and return

        /*let next_token = match iter.next() {
        //    Some(char) => Tokens::PLUS, //TODO: Actually parse stuff here
        //    None => Tokens::EOF, // Iterate over tokens instead.
         }; */

        let mut next_token = get_token(&mut iter);

        loop {
            if next_token != Tokens::EOF {
                tokens_list.push(next_token);
                next_token = get_token(&mut iter);
            } else {
                tokens_list.push(Tokens::EOF);
                break;
            }
        }

        return tokens_list;
    }
}

// Parses a token. Skips whitespace, and delegates complicated parsing to other functions
fn get_token(iter: &mut Chars) -> Tokens {
    let current_char = iter.next();

    match current_char {
        None => Tokens::EOF,
        Some(c) => {
            // If char is skipable, call recursively on next char
            if (skipable(c)) {
                return get_token(iter);
            }
            if (c.is_numeric()) {
                return Tokens::IntLit(c.to_digit(10).unwrap());
            }
            // Else return the current token
            return match c {
                '+' => Tokens::PLUS,
                '-' => Tokens::MINUS,
                '*' => Tokens::TIMES,
                '/' => Tokens::DIVIDE,
                ';' => Tokens::SEMICOLON,
                '>' => Tokens::GE,
                '<' => Tokens::LE,
                '=' => Tokens::EQUAL,
                _ => Tokens::PLACEHOLDER_TYPE,
            };
        }
    }
}

// Determine if current char is skipable
fn skipable(c: char) -> bool {
    if c.is_whitespace() {
        return true;
    }
    return false;
}
