use crate::ast::{Decl, Exp};
use crate::types::Type;
use std::fmt::{Display, Formatter};

// Maybe we need entire new AST instead of this quick fix?

pub struct TypedExp {
    pub exp: Exp,
    pub ty: Type,
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
