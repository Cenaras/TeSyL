use crate::lexer::Lexer;
use std::env;
use crate::parser::Parser;

mod ast;
mod lexer;
mod parser;
mod position;
mod token;
mod types;
mod semantic;

fn main() {
    let mut args = env::args().skip(1);
    let filename = args.next().unwrap();
    let mut lexer = Lexer::new(&filename).unwrap();
    let tokens = lexer.lex();
    lexer.print_tokens(&tokens);

    let mut parser = Parser::new(tokens);
    let result = parser.parse_program().unwrap();
    parser.print_result(&result);

}
