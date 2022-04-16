use std::env;
use std::fs;

pub struct Lexer {
    pub value: u8,

}

// Read file on creation

impl Lexer {
    pub fn new(program: String) -> Lexer {
        Lexer {
            value: 0,
        }
    }


}
