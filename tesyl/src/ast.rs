// Data in structs/enums is stored inline inside memory. Since recursive type, we cannot allocate
// size on stack, since recursion gives infinite size. Therefore wrap in Box which then stores on heap.
// Box<T> is a pointer (fixed size) to the heap, where we can have dynamic size.
use std::fmt::{self};
type Id = String;

// Do we require ; after all exps? Or just for seqexps?
#[derive(Debug, Clone)]
pub enum Exp {
    BinOpExp(Box<Exp>, BinOp, Box<Exp>),
    IntExp(i32),
    BoolExp(bool),
    VarExp(Id),
    LetExp(Id, Box<Exp>),
    AssignmentExp(Id, Box<Exp>),
    SeqExp(Vec<Exp>),
    IfExp(Box<Exp>, Box<Exp>, Box<Exp>),
    WhileExp(Box<Exp>, Box<Exp>),
    UnitExp,
    TupleExp(), // Vec of elements? In parser, comsume untill no more ','
    //Undefined,
}

#[derive(Debug, Clone)]
pub enum BinOp {
    PlusBinOp,
    MinusBinOp,
    TimesBinOp,
    DivideBinOp,
    EqualBinOp,
    NotEqualBinOp,
    GreaterThanBinOp,
    GreaterThanEqualBinOp,
    LessThanBinOp,
    LessThenEqualBinOp,
}

/*pub fn bin_op_exp_from_token(token: &Token) -> BinOp {
    match token {
        Token::PLUS => BinOp::PlusBinOp,
        Token::MINUS => BinOp::MinusBinOp,
        Token::TIMES => BinOp::TimesBinOp,
        Token::DIVIDE => BinOp::DivideBinOp,
        _ => BinOp::Undefined,
    }
}*/

// Potentially fix SeqExp to write more like we expect.
impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Exp::BinOpExp(left, op, right) => write!(f, "BinOpExp({} {} {})", left, op, right),
            Exp::IntExp(v) => write!(f, "IntExp({})", v),
            Exp::BoolExp(b) => write!(f, "BoolExp({})", b),
            Exp::VarExp(var) => write!(f, "VarExp({})", var),
            Exp::LetExp(x, v) => write!(f, "LetExp({}, {})", x, v),
            Exp::AssignmentExp(id, exp) => write!(f, "AssignmentExp({}, {})", id, exp),
            Exp::SeqExp(l) => write!(f, "SeqExp({:?})", l),
            Exp::IfExp(guard, then, els) => write!(f, "IfExp({}, {}, {})", guard, then, els),
            Exp::WhileExp(guard, body) => write!(f, "WhileExp({}, {})", guard, body),
            Exp::UnitExp => write!(f, "UnitExp"),
            //Exp::Undefined => write!(f, "Undefined"),
            //_ => write!(f, "Not Implemented "),
        }
    }
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            BinOp::PlusBinOp => write!(f, "+"),
            BinOp::MinusBinOp => write!(f, "-"),
            BinOp::TimesBinOp => write!(f, "*"),
            BinOp::DivideBinOp => write!(f, "/"),
            BinOp::EqualBinOp => write!(f, "="),
            BinOp::NotEqualBinOp => write!(f, "!="),
            BinOp::GreaterThanBinOp => write!(f, ">"),
            BinOp::GreaterThanEqualBinOp => write!(f, ">="),
            BinOp::LessThanBinOp => write!(f, "<"),
            BinOp::LessThenEqualBinOp => write!(f, "<="),
        }
    }
}
