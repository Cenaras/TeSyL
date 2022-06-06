use crate::token::Token;
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

    fn get_current_char(&mut self) -> Result<char, ErrorType> {
        let next = self.iter.peek();
        match next {
            Some(c) => Ok(*c),
            _ => Err("Error")
        }
    }
}
