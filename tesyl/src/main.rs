extern crate core;

use crate::hoisting::hoister;
use crate::lexer::Lexer;
use crate::llvm::{CFGBuilder, CFG};
use crate::parser::Parser;
use crate::semantic::SemanticAnalyzer;
use std::env;

mod ast;
mod habsyn;
mod hoisting;
mod ir;
mod lexer;
mod llvm;
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

    let mut builder = CFGBuilder::new();
    let t = builder.construct_cfg(typed_program);
    println!("{:?}", t);

    let typed_prog_test = sem.analyze(&program_exp);
    hoister(typed_prog_test);

    // TODO: First basic block is never terminated,
    // deal with this, and create basic blocks correctly
    //let cfg = builder.get_cfg();
    //println!("CFG: {:?}", cfg);

    /*
        I believe LLVM programs are just:
        list of type decls, list of globals, list of func
        So init with a main function?

        Hoisted AST has a "Program" with type decls
        and fun decls also - maybe do like this?

        Look hoisting.ml line 180



    */

    //CFG::cfg_test(typed_program);
}
