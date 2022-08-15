
/*
   Require a TypedAst also which is AST and type
   Semantic goes through untyped and types it
   So for instance a IntLit gets inttype
   and binop recursively checks left, right and ensures
   both are ints


   TODO: Maybe find a cleaner way to create these

*/

type Id = String;

struct Context {
    venv: HashMap<Id, Type>,
    //tenv:
    //err:
}

use crate::ast::Exp::*;
use crate::ast::{BinOp, Exp};
use crate::tabsyn::{TypedExp, TypedExpBase};
use crate::types::Type;
use crate::types::Type::{IntType, UnitType};
use std::collections::HashMap;
use std::iter::Map;

pub struct SemanticAnalyzer {
    ctxt: Context,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {
            ctxt: Context {
                venv: HashMap::new(),
            },
        }
    }

    //TODO: Composite types need to have typedexp as arguments
    // BinOp should be op two TypedExps
    pub fn analyze(&mut self, exp: &Exp) -> TypedExp {
        match exp {
            IntExp { value } => TypedExp {
                exp: TypedExpBase::IntExp { value: *value },
                ty: IntType,
            },
            BinOpExp { left, op, right } => match op {
                BinOp::PlusBinOp => {
                    let left_typed = self.analyze(&left);
                    let right_typed = self.analyze(&right);
                    match (left_typed.ty, right_typed.ty) {
                        (IntType, IntType) => TypedExp {
                            exp: TypedExpBase::BinOpExp {
                                left: Box::from(left_typed),
                                op: BinOp::PlusBinOp,
                                right: Box::from(right_typed),
                            },
                            ty: IntType,
                        },
                        _ => panic!("Panic"),
                    }
                }
                _ => panic!("Not implemented"), //TODO: Other BinOps
            },
            /*LetExp { id, value } => {
                let body = self.analyze(value);
                let id_clone = id.clone();
                //expand environment here
                self.ctxt.venv.insert(id.parse().unwrap(), body.ty);
                TypedExp {
                    exp: LetExp {
                        id: id_clone,
                        value: Box::from(body.exp),
                    },
                    ty: UnitType,
                }
            }*/
            _ => TypedExp {
                // TODO Implement
                exp: TypedExpBase::UnitExp,
                ty: UnitType,
            },
        }
    }

    pub fn print_typed(&self, ty_exp: &TypedExp) {
        println!("Typed AST: \n{}", ty_exp);
    }
}
