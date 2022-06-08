use std::fmt::{Display, Formatter};

type Id = String;

pub enum Exp {
    IntExp {
        value: u32,
    },
    BinOpExp {
        left: Box<Exp>,
        op: BinOp,
        right: Box<Exp>,
    },
    VarExp {
        id: Id,
    },
    SeqExp {
        expr: Vec<Exp>,
    },
    UnitExp,
}

// Decls produce no value, but may change the state
pub enum Decls {
    LetDecl { id: Id, value: Box<Exp> },
}

pub enum BinOp {
    PlusBinOp,
    MinusBinOp,
    TimesBinOp,
    DivideBinOp,
}

impl Display for Exp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Exp::BinOpExp {left, op, right} => write!(f, "BinOpExp({} {} {})", left, op, right),
            Exp::IntExp {value} => write!(f, "IntExp({})", value),
            _ => write!(f, "AST Print Not Implemented")
        }
    }
}

impl Display for BinOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &*self {
            BinOp::PlusBinOp => write!(f, "+"),
            BinOp::MinusBinOp => write!(f, "-"),
            BinOp::TimesBinOp => write!(f, "*"),
            BinOp::DivideBinOp => write!(f, "/"),
        }
    }
}
