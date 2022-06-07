use crate::token::{Token, TokenType};
use std::collections::HashMap;
use std::error::Error;
use std::iter::Peekable;
use std::str::Chars;
use std::vec::IntoIter;
use std::{fs, result};

pub struct Lexer {
    iter: Peekable<IntoIter<char>>,
}

type IOError = std::io::Error;
type ErrorType = &'static str;

impl Lexer {
    pub fn new(filename: &str) -> Result<Lexer, IOError> {
        let data = &fs::read_to_string(filename)?;
        let iter = data.chars().collect::<Vec<_>>().into_iter().peekable();
        Ok(Lexer { iter })
    }

    pub fn lex(&mut self) -> Vec<TokenType> {
        let mut tokens_list: Vec<TokenType> = vec![];
        let mut next_token = self.get_token();
        loop {
            println!("NEXT TOKEN IS: {}", next_token);
            if next_token != TokenType::EOF {
                tokens_list.push(next_token);
                next_token = self.get_token();
            } else {
                tokens_list.push(TokenType::EOF);
                break;
            }
        }
        tokens_list
    }

    fn get_token(&mut self) -> TokenType {
        while self.iter.peek() != None {
            let mut cur = self.iter.next().unwrap();
            while self.skipable(&cur) {
                if self.iter.peek() != None {
                    cur = self.iter.next().unwrap();
                } else {
                    return TokenType::EOF;
                }
            }

            // Determine next TokenType
            return if cur.is_numeric() {
                // Lex as number, consume until no more
                self.read_number(cur)
            } else if cur.is_alphanumeric() {
                // Else lex identifier
                self.read_identifier(cur)
                // Else return simple token
            } else {
                match cur {
                    '+' => TokenType::PLUS,
                    '-' => TokenType::MINUS,
                    '*' => TokenType::TIMES,
                    '/' => TokenType::DIVIDE,
                    ';' => TokenType::SEMICOLON,
                    '>' => {
                        return match self.iter.peek() {
                            Some('=') => {
                                self.iter.next(); // consume the =
                                TokenType::GEQ
                            }
                            _ => TokenType::GE,
                        };
                    }
                    '<' => {
                        return match self.iter.peek() {
                            Some('=') => {
                                self.iter.next(); // consume the =
                                TokenType::LEQ
                            }
                            _ => TokenType::LE,
                        };
                    }
                    '=' => {
                        return match self.iter.peek() {
                            Some('=') => {
                                self.iter.next();
                                TokenType::EQEQ
                            }
                            _ => TokenType::EQUAL,
                        }
                    }
                    '!' => {
                        return match self.iter.peek() {
                            Some('=') => {
                                self.iter.next(); // consume the =
                                TokenType::NEQ
                            }
                            _ => TokenType::NOT,
                        };
                    }
                    '(' => TokenType::OpenParen,
                    ')' => TokenType::CloseParen,
                    '{' => TokenType::OpenBrack,
                    '}' => TokenType::CloseBrack,
                    _ => TokenType::INVALID,
                }
            };
        }
        TokenType::EOF
    }

    // TODO: Ensure first is not letter
    // TODO: Clean up, delegate to functions, use borrows
    fn read_identifier(&mut self, cur: char) -> TokenType {
        let mut acc = String::from(cur);
        loop {
            match self.iter.peek() {
                Some(c) => {
                    if c.is_alphabetic() || c.is_numeric() {
                        acc.push(*c);
                    } else {
                        // If not identifier symbol, stop.
                        break;
                    }
                }
                None => break,
            }
            // Advance, if we did not break
            self.iter.next();
        }
        // Check for keywords
        match acc.as_str() {
            "let" => TokenType::LET,
            _ => TokenType::Identifier(acc),
        }
    }

    fn read_number(&mut self, cur: char) -> TokenType {
        let mut acc = String::from(cur);
        while self.is_digit() {
            acc.push(self.iter.next().unwrap());
        }

        TokenType::IntLit(acc.parse().unwrap())
    }

    fn is_digit(&mut self) -> bool {
        return match self.iter.peek() {
            None => false,
            Some(i) => i.is_numeric(),
        };
    }

    // Determine if current char is skipable
    fn skipable(&self, c: &char) -> bool {
        if c.is_whitespace() {
            return true;
        }
        return false;
    }

    fn advance(&mut self) -> Result<(), ErrorType> {
        return match self.iter.next() {
            Some(_) => {
                println!("Increase position information");
                Ok(())
            }
            _ => Err("EOF READ"),
        };
    }

    pub fn print_tokens(&self, tokens: &Vec<TokenType>) {
        println!("Resulting tokens: \n");
        for token in tokens {
            print!("{}", token);
        }
        println!("\n");
    }
}
