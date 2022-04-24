//TODO: Add sequence to grammar?
use crate::{Token, print_tokens};

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {

        print_tokens(&tokens);
        Parser {
            tokens: tokens,
        }
    }
}
