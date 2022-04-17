#![allow(unused_parens)]
#![allow(unused_variables)]

use std::env;
use std::fs;
use crate::tokens::Tokens;


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


    pub fn lex(&self) -> Vec<Tokens>{
        let mut tokenList: Vec<Tokens> = Vec::new();
        let mut iter = self.raw.chars().into_iter();

        // Iterate over everything and create tokens

        // While iter.next is Some: Parse the value, get the tokens
        // If iter.next = None, get the EOF token and return

        let next_token = match iter.next() {
            Some(char) => Tokens::PLUS, //TODO: Actually parse stuff here
            None => Tokens::EOF, // Iterate over tokens instead.
        };

        tokenList.push(next_token);
        return tokenList;
    }




}
