use crate::ast::{BinOp, Decl, Exp};
use crate::types::Type;
use std::fmt::{Display, Formatter};

// Maybe we need entire new AST instead of this quick fix?
type Id = String;
// TODO: Need TypedExpBase for TypedExp for IR.

pub struct TypedExp {
    pub exp: Exp,
    pub ty: Type,
}

pub enum TypedExpBase {
    IntExp {
        value: i64
    },
    BinOpExp {
        left: Box<TypedExp>,
        op: BinOp,
        right: Box<TypedExp>,
    },
    LetExp {
        id: Id,
        value: Box<TypedExp>,
    },
    UnitExp,
}


pub struct TypedDecl {
    decl: Decl,
    ty: Type,
}

impl Display for TypedExp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.exp, self.ty)
    }
}
