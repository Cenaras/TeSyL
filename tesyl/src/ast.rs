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
