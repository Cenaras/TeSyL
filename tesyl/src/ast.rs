use std::fmt::{Display, Formatter};

type Id = String;

#[derive(Debug, Clone, PartialEq)]
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
    LetExp {
        id: Id,
        value: Box<Exp>,
    },
    SeqExp {
        expr: Vec<Exp>,
    },
    UnitExp,
}

// Decls produce no value, but may change the state
// TODO Determine if we use these - and update structure
// so we have ASTNode with Decl and Exp as subtypes
// FOR NOW: Let is an expression with UnitType that updates the environment

pub enum Decl {
    //LetDecl { id: Id, value: Box<Exp> },
}

#[derive(Clone, PartialEq, Debug)]
pub enum BinOp {
    PlusBinOp,
    MinusBinOp,
    TimesBinOp,
    DivideBinOp,
}

impl Display for Exp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Exp::BinOpExp { left, op, right } => write!(f, "BinOpExp({} {} {})", left, op, right),
            Exp::IntExp { value } => write!(f, "IntExp({})", value),
            Exp::LetExp { id, value } => write!(f, "LetExp({}, {})", id, value),
            Exp::SeqExp { expr } => write!(f, "SeqExp({:?})", expr),
            _ => write!(f, "AST Print Not Implemented"),
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
