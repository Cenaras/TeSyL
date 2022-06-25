use crate::ast::BinOp;
use crate::types::Type;

type Id = &'static str;

pub struct HoistedExp {
    pub exp: HoistedExpBase,
    pub ty: Type,
}

pub enum HoistedExpBase {
    IntExp {
        value: i64,
    },
    BinOpExp {
        left: Box<HoistedExp>,
        op: BinOp,
        right: Box<HoistedExp>,
    },
}

pub struct FunDeclData {
    pub(crate) name: Id,
    // args,
    pub(crate) result: Type,
    pub(crate) body: HoistedExp,
    //locals, parent, ...
}

pub struct Program {
    fun_decls: Vec<FunDeclData>,
}
