#![allow(unused_parens)]
#![allow(unused_variables)]

use crate::tokens::Token;
use std::{str::Chars, iter::Peekable};

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
    while(iter.peek() != None) {
        
        let mut cur = iter.next().unwrap();
        println!("Saw token {}", cur);
        // Disregard whitespaces - if last is whitespace, return
        while (skipable(&cur)) {
            println!("Skip");
            if (iter.peek() != None) {
                cur = iter.next().unwrap();
            } else {
                return Token::EOF;
            }
        }

        // If we see number, keep consuming numbers.
        if (cur.is_numeric()) {
            let mut acc = String::from(cur);
            while(isDigit(iter.peek())) {
                let val = iter.next().unwrap();
                println!("Val is {}", val);
                if (val.is_numeric()) {
                    acc.push(val)
                }
            }

            return Token::IntLit(acc.parse().unwrap());
        }
        else {
            return match cur {
                '+' => Token::PLUS,
                '-' => Token::MINUS,
                '*' => Token::TIMES,
                '/' => Token::DIVIDE,
                ';' => Token::SEMICOLON,
                '>' => Token::GE,
                '<' => Token::LE,
                '=' => Token::EQUAL,
                _ => Token::PLACEHOLDER_TYPE,
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


fn isDigit(c: Option<&char>) -> bool {
    return match c {
        None => false,
        Some(i) => i.is_numeric(),
    }
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
