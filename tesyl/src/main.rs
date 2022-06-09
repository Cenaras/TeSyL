use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::semantic::SemanticAnalyzer;
use std::env;

mod ast;
mod lexer;
mod parser;
mod position;
mod semantic;
mod tabsyn;
mod token;
mod types;

// TODO: Minimize ownership where not needed in general

fn main() {
    let mut args = env::args().skip(1);
    let filename = args.next().unwrap();
    let mut lexer = Lexer::new(&filename).unwrap();
    let tokens = lexer.lex();
    lexer.print_tokens(&tokens);

    let mut parser = Parser::new(tokens);
    let program_exp = parser.parse_program().unwrap();
    parser.print_result(&program_exp);

    let mut sem = SemanticAnalyzer::new();
    let typed_program = sem.analyze(&program_exp);
    sem.print_typed(&typed_program);
}
