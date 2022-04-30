#![allow(unused_parens)]
#![allow(unused_variables)]
mod tokens;
mod val;

// Using the module tokens
use tokens::Token; // Shorthanding tokens::TOKENS to just TOKENS

mod lexer;
use lexer::Lexer; // Must be possible to avoid doing this double stuff

mod parser;
use parser::Parser;

mod ast;
use ast::*;

mod interpreter;
use interpreter::Interpreter;

// Path is from current terminal path. Call from root of project

fn main() {
    // Mutable, since the iterator updates the state after each .next call
    let mut args = std::env::args().skip(1);
    let filename = args.next().expect("No file was specified");
    let test_filename = filename.clone();

    let do_test = match args.next() {
        Some(t) => t.as_str() == "--test" || t.as_str() == "-t",
        _ => false,
    };

    assert_correct_format(&filename);

    let lexer = Lexer::new(filename).unwrap();
    let tokens = lexer.lex();

    let mut parser = Parser::new(lexer.lex());
    let program = parser.parse_program();
    let test_program = program.clone();

    let mut interpreter = Interpreter::new();
    let result = interpreter.eval(program.unwrap());

    println!("Program terminated with result: \n{}", result);

    // ##### TEST PROGRAMS IF SPECIFIED #####
    // ToDo: Add --all support to test every sample file.
    if (do_test) {
        let test_type = args
            .next()
            .expect("Please provide either 'lex' or 'par' for testing type");

        let run_all = match args.next() {
            Some(s) => s.as_str() == "--all",
            _ => false,
        };

        match test_type.as_str() {
            "-lex" => test_lex(test_filename, tokens, run_all),
            "-par" => test_par(test_filename, test_program, run_all),
            "-int" => test_int(test_filename, result, run_all),
            _ => panic!("Test type not supported; only -lex and -par are supported"),
        };
    }
}

// ToDo: Support run_all files in directory
fn test_lex(filename: String, tokens: Vec<Token>, run_all: bool) {
    let mut test_name = filename.strip_suffix(".tsl").unwrap().to_owned();
    test_name.push_str(".lex");

    // Read file, split on spaces, compare. Get tokens and push to big string...
    let mut expected =
        std::fs::read_to_string(format!(".\\expected\\lexing\\{}", test_name)).unwrap();
    expected.retain(|c| !c.is_whitespace()); // remove whitespace

    let mut result = String::from("");
    for token in tokens {
        result.push_str(format!("{}", token).as_str());
    }
    result.retain(|c| !c.is_whitespace()); // remove whitespace

    assert!(expected.eq(&result));

    println!("Lexing test successful for {}", test_name);
}

fn test_par(filename: String, program: Result<Exp, &str>, run_all: bool) {
    let mut test_name = filename.strip_suffix(".tsl").unwrap().to_owned();
    test_name.push_str(".par");

    let mut expected =
        std::fs::read_to_string(format!(".\\expected\\parsing\\{}", test_name)).unwrap();
    expected.retain(|c| !c.is_whitespace()); // remove whitespace

    let mut result = format!("{}", program.unwrap());
    result.retain(|c| !c.is_whitespace()); // remove whitespace

    assert!(expected.eq(&result));
    println!("Parsing test successful for {}", test_name);
}

use crate::val::Val;
fn test_int(filename: String, value: Val, run_all: bool) {
    let mut test_name = filename.strip_suffix(".tsl").unwrap().to_owned();
    test_name.push_str(".int");

    let mut expected =
        std::fs::read_to_string(format!(".\\expected\\runtime\\{}", test_name)).unwrap();
    expected.retain(|c| !c.is_whitespace()); // remove whitespace

    let mut result = format!("{}", value);
    result.retain(|c| !c.is_whitespace()); // remove whitespace

    assert!(expected.eq(&result));
    println!("Interpreter test successful for {}", test_name); 
}

pub fn print_tokens(tokens: &Vec<Token>) {
    println!("Tokens are: ");
    for token in tokens {
        print!("{}", token);
    }
    println!("\n");
}

fn print_program(program: &Exp) {
    println!("AST for program is: ");
    println!("{}\n", program);
}

// Assert the format of the arguments are correct. We do not need to own the variable, a slice is sufficient
fn assert_correct_format(file: &String) {
    if (!file.ends_with(".tsl")) {
        panic!("File does not end in .tsl, which is the expected format");
    }
}
