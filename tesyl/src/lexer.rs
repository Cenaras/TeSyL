use std::env;
use std::fs;
use crate::tokens::TOKENS;


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


    pub fn run(&self) {
        let mut iter = self.raw.chars().into_iter();
        
    }




}
