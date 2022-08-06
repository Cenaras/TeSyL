/*
    For now: Hoisting essentially just adds a main function.
    In future - we need to hoist fun decls out and
    other stuff here...

    There are some issues with types here, ecerything is expected
    int for now

*/

use crate::habsyn::{FunDeclData, HoistedExp, HoistedExpBase, HoistedProgram};
use crate::llvm::Program;
use crate::tabsyn::{TypedExp, TypedExpBase};
use crate::types::Type;

fn hoist_exp(exp: TypedExp) -> HoistedExp {
    match exp.exp {
        TypedExpBase::IntExp { value } => HoistedExp {
            exp: HoistedExpBase::IntExp { value },
            ty: Type::IntType,
        },
        TypedExpBase::BinOpExp { left, op, right } => {
            let l = hoist_exp(*left);
            let r = hoist_exp(*right);
            HoistedExp {
                exp: HoistedExpBase::BinOpExp {
                    left: Box::from(l),
                    op,
                    right: Box::from(r),
                },
                ty: Type::IntType,
            }
        }
        _ => panic!("Unimplemented"),
    }
}

pub fn hoister(exp: TypedExp) -> HoistedProgram {
    let hoisted_exp = hoist_exp(exp);

    let main = FunDeclData {
        name: "main",
        result: Type::IntType,
        body: hoisted_exp,
    };
    // For now, all hoister does is return program with main
    // func, with given expression
    HoistedProgram {
        fun_decls: vec![main],
    }
}
