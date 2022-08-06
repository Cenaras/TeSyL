#![allow(dead_code)]
#![allow(unused_variables)]
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
    // Format: samples/[name].tsl
    let filename = args.next().unwrap();
    let mut lexer = Lexer::new(&filename).unwrap();
    let tokens = lexer.lex();
    //lexer.print_tokens(&tokens);

    let mut parser = Parser::new(tokens);
    let program_exp = parser.parse_program().unwrap();
    //parser.print_result(&program_exp);

    let mut sem = SemanticAnalyzer::new();
    let typed_program = sem.analyze(&program_exp);
    //sem.print_typed(&typed_program);

    let typed_prog_test = sem.analyze(&program_exp);
    let hoisted = hoister(typed_prog_test);

    let mut cfg_builder = CFGBuilder::new();

    let llvm_prog = cfg_builder.codegen_prog(&hoisted);

    cfg_builder.print_llvm_prog_debug(&llvm_prog);


    // TODO: First basic block is never terminated,
    // deal with this, and create basic blocks correctly.
    // Reason: We need hoisting to create functions, so simple programs
    // have block termination. Otherwise, an addition never terminates the block.

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
