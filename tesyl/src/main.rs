#![allow(unused_parens)]
#![allow(unused_variables)]

mod tokens;

// Using the module tokens
use tokens::Tokens; // Shorthanding tokens::TOKENS to just TOKENS

mod lexer;
use lexer::Lexer; // Must be possible to avoid doing this double stuff

// Path is from current terminal path. Call from root of project

fn main() {
    // Mutable, since the iterator updates the state after each .next call
    let mut args = std::env::args().skip(1);
    let filename = args.next().expect("No file was specified");

    let lexer = Lexer::new(filename).unwrap();
    println!("Raw data is: {}", lexer.raw);
    let tokens = lexer.lex();
    printTokens(tokens);


    //let stream of tokens = lexer.run();

    test_stuff();
}

fn test_stuff() {
    let intlit = Tokens::IntLit(2);
    let result = match intlit {
        Tokens::IntLit(value) => Some(value),
        _ => None,
    };

    println!("The value is: {}", result.unwrap());

    //let lexer = Lexer::new("test".to_string());
    //println!("Lexer raw: {}", lexer.raw)

    // Test read file, stream/vec of tokens, ...
}

fn printTokens(tokens: Vec<Tokens>) {
    println!("Tokens are: ");
    for token in tokens {
        print!("{}", token);
    }
    println!();
}

