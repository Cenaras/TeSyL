#![allow(unused_parens)]
#![allow(unused_variables)]

use crate::tokens::Token;
use std::{iter::Peekable, str::Chars};

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

    pub fn lex(&self) -> Vec<Token> {
        let mut tokens_list: Vec<Token> = Vec::new();
        let mut iter = self.raw.chars().into_iter().peekable();

        // Iterate over everything and create tokens

        // While iter.next is Some: Parse the value, get the tokens
        // If iter.next = None, get the EOF token and return

        /*let next_token = match iter.next() {
        //    Some(char) => Tokens::PLUS, //TODO: Actually parse stuff here
        //    None => Tokens::EOF, // Iterate over tokens instead.
         }; */

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

        } else {
            return match cur {
                '+' => Token::PLUS,
                '-' => Token::MINUS,
                '*' => Token::TIMES,
                '/' => Token::DIVIDE,
                ';' => Token::SEMICOLON,
                '>' => Token::GE,
                '<' => Token::LE,
                '=' => Token::EQUAL,
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

fn is_digit(c: Option<&char>) -> bool {
    return match c {
        None => false,
        Some(i) => i.is_numeric(),
    };
}


fn read_number(cur: char, iter: &mut Peekable<Chars>) -> Token{
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

//fn read_number(iter: &mut Peekable<Chars>) -> Token {
//let mut val = iter.next().unwrap();
// loop {
//     match iter.next() {
//         Some(char) => {
//             if c.is_numeric() {
//                 println!("Char is {}", char);
//                 val.push(char)
//             } else {
//                 println!("Result is {}", val);
//                 return Token::IntLit(val.parse().unwrap());
//             }
//         }
//         // Always expcet a valid formed intlit here.
//         None => {
//             println!("Val is {}", val);
//             return Token::IntLit(val.parse().unwrap());
//         }
//     }
// }
//return Token::PLACEHOLDER_TYPE;
//}
