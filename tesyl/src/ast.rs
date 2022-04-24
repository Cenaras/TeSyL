// Data in structs/enums is stored inline inside memory. Since recursive type, we cannot allocate
// size on stack, since recursion gives infinite size. Therefore wrap in Box which then stores on heap.
// Box<T> is a pointer (fixed size) to the heap, where we can have dynamic size.
pub enum Exp {
    BinOpExp(Box<Exp>, BinOp, Box<Exp>),
    IntExp(i32),
}

pub enum BinOp {
    PlusBinOp,
    MinusBinOp,
    TimesBinOp, 
    DivideBinOp,
}
