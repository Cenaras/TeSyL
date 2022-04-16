mod tokens; // Using the module tokens
use tokens::TOKENS; // Shorthanding tokens::TOKENS to just TOKENS

mod lexer;
use lexer::Lexer; // Must be possible to avoid doing this double stuff


fn main() {
    // Mutable, since the iterator updates the state after each .next call
    let mut args = std::env::args().skip(1);
    let filename = args.next().expect("No file was specified");
    let lexer = Lexer::new(filename);
    //let stream of tokens = lexer.run();


    test_stuff();

}




fn test_stuff() {

    let intlit = TOKENS::IntLit(2);
    let result = match intlit {
        TOKENS::IntLit(value) => Some(value),
        _ => None
    };

    println!("The value is: {}", result.unwrap());

    let lexer = Lexer::new("test".to_string());
    println!("Lexer value: {}", lexer.value)


    // Test read file, stream/vec of tokens, ...

}





