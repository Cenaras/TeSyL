use crate::lexer::Lexer;
use std::env;

mod ast;
mod lexer;
mod parser;
mod position;
mod token;
mod types;

fn main() {
    let mut args = env::args().skip(1);
    let filename = args.next().unwrap();
    let mut lexer = Lexer::new(&filename).unwrap();
    let tokens = lexer.lex();
    lexer.print_tokens(&tokens);
}
