#![allow(unused_parens)]
#![allow(unused_variables)]
use std::path::Path;
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
    let test_filename = filename.clone();

    let do_test = match args.next() {
        Some(t) => t.as_str() == "--test" || t.as_str() == "-t",
        _ => false
    };

    assert_correct_format(&filename);

    let lexer = Lexer::new(filename).unwrap();
    let tokens = lexer.lex();


    let mut parser = Parser::new(lexer.lex());
    let program = parser.parse_program();
    let test_program = program.as_ref().unwrap();
    print_program(&program.unwrap());

    // Support lexing and parsing tests with 3rd argument lex/par. For now we only lex
    
    // ##### TEST PROGRAMS IF SPECIFIED #####
    if (do_test) {
        let test_type = args.next().expect("Please provide either 'lex' or 'par' for testing type");
        match test_type.as_str() {
            "-lex" => test_lex(test_filename, tokens),
            "-par" => test_par(test_filename),
            _ => panic!("Test type not supported; only -lex and -par are supported"),
        };
    }


}


// Important - all tokens in test files must have a whitespace preceding them.
fn test_lex(filename: String, tokens: Vec<Token>) {
    let mut test_name = filename.strip_suffix(".tsl").unwrap().to_owned();
    test_name.push_str(".lex");
    
    // Read file, split on spaces, compare. Get tokens and push to big string...
    let expected = std::fs::read_to_string(format!(".\\expected\\lexing\\{}", test_name)).unwrap();
        
    let mut result = String::from("");
    for token in tokens {
        result.push_str(format!("{}", token).as_str());
    }

    assert!(expected.eq(&result));

    println!("Test successful for {}", test_name);
}

// Give program also - fix this later...
fn test_par(filename: String) {

}


pub fn print_tokens(tokens: &Vec<Token>) {
    println!("Tokens are: ");
    for token in tokens {
        print!("{}", token);
    }
    println!();
}

fn print_program(program: &Exp) {
    println!("AST for program is: ");
    println!("{}", program);
}

// Assert the format of the arguments are correct. We do not need to own the variable, a slice is sufficient
fn assert_correct_format(file: &String) {
    if (!file.ends_with(".tsl")) {
        panic!("File does not end in .tsl, which is the expected format");
    }
}
