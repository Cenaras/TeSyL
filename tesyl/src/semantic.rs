/*
   Require a TypedAst also which is AST and type
   Semantic goes through untyped and types it
   So for instance a IntLit gets inttype
   and binop recursively checks left, right and ensures
   both are ints


   TODO: Maybe find a cleaner way to create these

*/

/*
   struct Context {
       venv:
       tenv:
       err:
   }
*/

use crate::ast::Exp::*;
use crate::ast::{BinOp, Exp};
use crate::tabsyn::TypedExp;
use crate::types::Type;
use crate::types::Type::{IntType, UnitType};

pub struct SemanticAnalyzer {}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {}
    }

    pub fn analyze(&self, exp: &Exp) -> TypedExp {
        match exp {
            IntExp { value } => TypedExp {
                exp: IntExp { value: *value },
                ty: IntType,
            },
            BinOpExp { left, op, right } => match op {
                BinOp::PlusBinOp => {
                    let left_typed = self.analyze(&left);
                    let right_typed = self.analyze(&right);
                    match (left_typed.ty, right_typed.ty) {
                        (IntType, IntType) => TypedExp {
                            exp: BinOpExp {
                                left: Box::from(left_typed.exp),
                                op: BinOp::PlusBinOp,
                                right: Box::from(right_typed.exp),
                            },
                            ty: IntType,
                        },
                        _ => panic!("Panic"),
                    }
                }
                BinOp::MinusBinOp => {
                    let left_typed = self.analyze(&left);
                    let right_typed = self.analyze(&right);
                    match (left_typed.ty, right_typed.ty) {
                        (IntType, IntType) => TypedExp {
                            exp: BinOpExp {
                                left: Box::from(left_typed.exp),
                                op: BinOp::MinusBinOp,
                                right: Box::from(right_typed.exp),
                            },
                            ty: IntType,
                        },
                        _ => panic!("Error here"),
                    }
                }
                BinOp::TimesBinOp => {
                    let left_typed = self.analyze(&left);
                    let right_typed = self.analyze(&right);
                    match (left_typed.ty, right_typed.ty) {
                        (IntType, IntType) => TypedExp {
                            exp: BinOpExp {
                                left: Box::from(left_typed.exp),
                                op: BinOp::TimesBinOp,
                                right: Box::from(right_typed.exp),
                            },
                            ty: IntType,
                        },
                        _ => panic!("Error here"),
                    }
                }
                BinOp::DivideBinOp => {
                    let left_typed = self.analyze(&left);
                    let right_typed = self.analyze(&right);
                    match (left_typed.ty, right_typed.ty) {
                        (IntType, IntType) => TypedExp {
                            exp: BinOpExp {
                                left: Box::from(left_typed.exp),
                                op: BinOp::DivideBinOp,
                                right: Box::from(right_typed.exp),
                            },
                            ty: IntType,
                        },
                        _ => panic!("Error here"),
                    }
                }
                _ => panic!("Not implemented"), //TODO: Other BinOps
            },

            _ => TypedExp {
                // TODO Implement
                exp: UnitExp,
                ty: UnitType,
            },
        }
    }

    pub fn print_typed(&self, ty_exp: &TypedExp) {
        println!("Typed AST: \n{}", ty_exp);
    }
}
