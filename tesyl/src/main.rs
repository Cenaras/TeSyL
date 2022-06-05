#![allow(unused_parens)]
#![allow(unused_variables)]
extern crate core;

mod tokens;
mod val;

use core::panic;
use std::collections::HashMap;

// Using the module tokens
use tokens::Token; // Shorthanding tokens::TOKENS to just TOKENS

mod lexer;
use lexer::Lexer; // Must be possible to avoid doing this double stuff

mod parser;
use parser::Parser;

mod ast;
use ast::*;

mod interpreter;

use crate::interpreter::Interpreter;

// Path is from current terminal path. Call from root of project

type Id = String;

fn real_test(file: String) {
    assert_correct_format(&file);
    let mut init_venv: HashMap<Id, Val> = HashMap::new();
    let mut init_venf: HashMap<Id, Val> = HashMap::new();
    let mut interpreter = Interpreter::new();
    let result = interpreter.eval(
        Parser::new(Lexer::real(file).unwrap().lex())
            .parse_program()
            .unwrap(),
        &mut init_venv,
        &mut init_venf,
    );
    println!("Program terminated with result: \n{}\n", result);
}

fn main() {
    // Mutable, since the iterator updates the state after each .next call
    let mut args = std::env::args().skip(1);
    let filename = args.next().expect("No file was specified");
    let test_filename = filename.clone();

    if filename == "--run" {
        let file = args.next().expect("No file specified");
        real_test(file);
        return;
    }

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

    let mut init_venv: HashMap<Id, Val> = HashMap::new();
    let mut init_fenv: HashMap<Id, Val> = HashMap::new();

    let mut interpreter = Interpreter::new();
    let result = interpreter.eval(program.unwrap(), &mut init_venv, &mut init_fenv);

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
            "-lex" => {
                if (run_all) {
                    test_all_lex();
                } else {
                    test_lex(test_filename, tokens, true)
                }
            }
            "-par" => {
                if (run_all) {
                    test_all_parse();
                } else {
                    test_par(test_filename, test_program, true);
                }
            }
            "-int" => {
                if (run_all) {
                    test_all_int();
                } else {
                    test_int(test_filename, result, false)
                }
            }
            _ => panic!("Test type not supported; only -lex, -par and -int are supported"),
        };
    }
}

// Todo: Clean these up. Also fix so --all is enough and we dont need to specify a file

fn test_all_lex() {
    let paths = std::fs::read_dir(".\\samples").unwrap();
    for path in paths {
        let filename = format!("{}", path.unwrap().path().display())
            .strip_prefix(".\\samples\\")
            .unwrap()
            .to_string();
        let temp_file = filename.clone();
        let tokens = Lexer::new(temp_file).unwrap().lex();
        test_lex(filename, tokens, false);
    }

    println!("All tests for lexing successful!")
}

fn test_all_parse() {
    let paths = std::fs::read_dir(".\\samples").unwrap();
    for path in paths {
        let filename = format!("{}", path.unwrap().path().display())
            .strip_prefix(".\\samples\\")
            .unwrap()
            .to_string();
        let temp_file = filename.clone();
        let result = Parser::new(Lexer::new(temp_file).unwrap().lex()).parse_program();
        test_par(filename, result, false)
    }

    println!("All tests for parsing successful!");
}

fn test_all_int() {
    let paths = std::fs::read_dir(".\\samples").unwrap();
    for path in paths {
        let filename = format!("{}", path.unwrap().path().display())
            .strip_prefix(".\\samples\\")
            .unwrap()
            .to_string();
        let temp_file = filename.clone();

        let mut init_venv: HashMap<Id, Val> = HashMap::new();
        let mut init_fenv: HashMap<Id, Val> = HashMap::new();

        let val = Interpreter::new().eval(
            Parser::new(Lexer::new(temp_file).unwrap().lex())
                .parse_program()
                .unwrap(),
            &mut init_venv,
            &mut init_fenv,
        );
        test_int(filename, val, false)
    }
    println!("All tests for interpreter successful!");
}

// ToDo: Support run_all files in directory + clean up and reduce code duplication
fn test_lex(filename: String, tokens: Vec<Token>, do_print: bool) {
    let (expected, test_name) = generate_expected(".lex".to_string(), filename);

    let mut result = String::from("");
    for token in tokens {
        result.push_str(format!("{}", token).as_str());
    }
    result.retain(|c| !c.is_whitespace()); // remove whitespace

    assert!(expected.eq(&result));

    if (do_print) {
        println!("Lexing test successful for {}", test_name);
    }
}

fn test_par(filename: String, program: Result<Exp, &str>, do_print: bool) {
    let (expected, test_name) = generate_expected(".par".to_string(), filename);

    let mut result = format!("{}", program.unwrap());
    result.retain(|c| !c.is_whitespace()); // remove whitespace

    assert!(expected.eq(&result));
    if (do_print) {
        println!("Parsing test successful for {}", test_name);
    }
}

use crate::val::Val;

fn test_int(filename: String, value: Val, do_print: bool) {
    let (expected, test_name) = generate_expected(".int".to_string(), filename);

    let mut result = format!("{}", value);
    result.retain(|c| !c.is_whitespace()); // remove whitespace

    assert!(expected.eq(&result));
    if (do_print) {
        println!("Interpreter test successful for {}", test_name);
    }
}

fn generate_expected(test_type: String, filename: String) -> (String, String) {
    let mut test_name = filename.strip_suffix(".tsl").unwrap().to_owned();
    test_name.push_str(test_type.as_str());

    let path = match test_type.as_str() {
        ".lex" => format!(".\\expected\\lexing\\{}", test_name),
        ".par" => format!(".\\expected\\parsing\\{}", test_name),
        ".int" => format!(".\\expected\\runtime\\{}", test_name),
        _ => panic!("Unexpected test type {}", test_type),
    };

    let mut expected = std::fs::read_to_string(format!("{}", path)).unwrap();
    expected.retain(|c| !c.is_whitespace());
    (expected, test_name)
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
