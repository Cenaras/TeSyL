fn main() {
    println!("Hello, world!");
    let intlit = TOKENS::IntLit(2);

    let result = match intlit {
        TOKENS::IntLit(value) => Some(value),
        _ => None
    };
    println!("The value is: {}", result.unwrap());




}



enum TOKENS {
    PLUS,
    MINUS,
    TIMES,
    DIVIDE,
    GE,
    LE,
    EQUAL,
    IntLit(u32),
}
