#![allow(unused_parens)]
#![allow(unused_variables)]

mod tokens;

// Using the module tokens
use tokens::Token; // Shorthanding tokens::TOKENS to just TOKENS

mod lexer;
use lexer::Lexer; // Must be possible to avoid doing this double stuff

mod parser;
use parser::Parser;
mod ast;
use ast::*;


// Path is from current terminal path. Call from root of project

fn main() {
    // Mutable, since the iterator updates the state after each .next call
    let mut args = std::env::args().skip(1);
    let filename = args.next().expect("No file was specified");

    assert_correct_format(&filename);

    let lexer = Lexer::new(filename).unwrap();
    let tokens = lexer.lex();
    // ### TEST ENVIRONMENT ###

    print_tokens(&tokens);

    test_stuff();

    let parser = Parser::new(lexer.lex());

}

fn test_stuff() {

    let test = Exp::BinOpExp(Box::new(Exp::IntExp(2)), BinOp::PlusBinOp, Box::new(Exp::IntExp(2)));

}

pub fn print_tokens(tokens: &Vec<Token>) {
    println!("Tokens are: ");
    for token in tokens {
        print!("{}", token);
    }
    println!();
}

// Assert the format of the arguments are correct. We do not need to own the variable, a slice is sufficient
fn assert_correct_format(file: &String) {
    if (!file.ends_with(".tsl")) {
        panic!("File does not end in .tsl, which is the expected format");
    }
}
