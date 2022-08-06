use crate::ast::{BinOp, Exp};
use crate::habsyn::{FunDeclData, HoistedExp, HoistedExpBase, HoistedProgram};
use crate::llvm::Bop::Add;
use crate::llvm::Operand::Const;
use crate::llvm::Ty::I64;
use crate::tabsyn::{TypedExp, TypedExpBase};
use crate::types::Type;

// TODO: Need a proper design choice in mutation vs non mutation, and need
// to test a lot of things such as basic block creation.

type FreshId = &'static str;

// Local identifiers for functions
type Uid = FreshId;

// Global identifiers
type Gid = FreshId;

// List of arguments and return type
type FunType = (Vec<Ty>, Ty);

// Lifetime says: The reference to the Gid, FunDecl must life as long as the Program Struct does.
pub struct Program<'a> {
    //tdecls, globals,
    pub fun_decls: Vec<&'a (Gid, FunDecl)>,
}

pub struct FunDecl {
    fun_type: FunType,
    params: Vec<Uid>,
    cfg: CFG,
}

// LLVM Types - For now just simple stuff
#[derive(Debug, Copy, Clone)]
pub enum Ty {
    Void,
    I1,
    I8,
    I64,
}

// Types of operands for expressions
// Copy prolly not, fix box and stuff later to get id
#[derive(Clone, Copy, Debug)]
pub enum Operand {
    Const(i64),
    //Constants
    Id(FreshId), //Variables - typically storing results
}

#[derive(Debug)]
pub enum Bop {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Instr {
    BinOp(Bop, Ty, Operand, Operand),
    Alloca(Ty),
}

#[derive(Debug)]
pub enum Terminator {
    Ret(Ty, Option<Operand>),
    Unreachable, //testing
    // Br lbl, Cbr op lb, lb
}

#[derive(Debug)]
pub struct BasicBlock {
    label: String,
    instructions: Vec<Instruction>,
    // (Option(uid), instruc)
    terminator: Terminator,
}

impl BasicBlock {
    pub fn new(label: String) -> Self {
        BasicBlock {
            label,
            instructions: Vec::new(),
            terminator: Terminator::Unreachable,
        }
    }

    /*    pub fn add_instruction(&mut self, instr: Instr) -> &Self {
        self.instructions.push(instr);
        self
    }*/
}

#[derive(Debug)]
pub struct CFG {
    initial: BasicBlock,
    blocks: Vec<BasicBlock>,
}

// Potential label to store result and the instruction
type Instruction = (Option<FreshId>, Instr);

pub struct CFGBuilder {
    rev_basic_blocks: Vec<BasicBlock>,
    rev_instr: Vec<Instruction>,
    first_basic_block: Option<BasicBlock>,
    cur_block_label: Option<String>,
}

impl CFGBuilder {
    pub fn new() -> Self {
        CFGBuilder {
            rev_basic_blocks: vec![],
            rev_instr: vec![],
            first_basic_block: None,
            cur_block_label: None,
        }
    }

    pub fn add_instr(&mut self, instr: Instruction) {
        println!("Adding instruction");
        self.rev_instr.push(instr)
    }

    // Maybe we need to split the naming of block out separate from the struct?
    // Either return cfg builder or mutate?
    pub fn term_block(mut self, term: Terminator) {
        let mut ins_list = self.rev_instr;
        ins_list.reverse();

        let bb = BasicBlock {
            label: "name".to_string(),
            instructions: ins_list,
            terminator: term,
        };

        match self.first_basic_block {
            None => self.first_basic_block = Some(bb),
            Some(block) => self.rev_basic_blocks.push(bb),
        };
    }

    // Playground for now - maybe return non-mut builder
    // with updated bindings?
    pub fn codegen_exp(&mut self, hoisted_prog: &HoistedExp) -> Operand {
        match &hoisted_prog.exp {
            HoistedExpBase::IntExp { value } => Const(*value),

            HoistedExpBase::BinOpExp { left, op, right } => {
                // TODO: More stuff than just this - id's and stuff...

                // Get types of left and right
                let ty_left = left.ty;
                let ty_right = right.ty;

                // Recursive compute left and right, and get operand storing result of each
                let left_op = self.codegen_exp(&left);
                let right_op = self.codegen_exp(&right);

                // Placeholder label for now
                let lbl = "lbl";

                match (ty_left, ty_right) {
                    // Case where binop between ints
                    (Type::IntType, Type::IntType) => {
                        // Pretty do this instead of each branch being duplicate
                        match op {
                            BinOp::PlusBinOp => {
                                self.add_instr((
                                    Some(lbl),
                                    Instr::BinOp(Add, I64, left_op, right_op),
                                ));
                            }
                            _ => {
                                panic!("Only add")
                            }
                        }
                    }

                    _ => {
                        panic!("No other types")
                    }
                }
                Operand::Id(lbl)
            }
            _ => panic!("Unimpl"),
        }
    }

    pub fn codegen_fun_decl(&mut self, fdecl: &FunDeclData) -> (FreshId, FunDecl) {
        // Note; This has to handle a lot more in the future

        // Get return type of function - for now this is only main
        let ret_ty = match fdecl.result {
            Type::IntType => {
                I64 // Default to I64
            }
            _ => panic!("Not implemented"),
        };

        // Code gen for the body
        let body = self.codegen_exp(&fdecl.body);
        println!("The body: {:?}", body);


        /*
        This is just placeholder. As of now: No args given, return type is I64.
        Instructions are the ones accumulated by the cfg builder.
        Terminator is a Return of I64 with the Operand from the body
        */

        let test = FunDecl {
            fun_type: (vec![], ret_ty),
            params: vec![],
            cfg: CFG {
                initial: BasicBlock {
                    label: "".to_string(),
                    instructions: self.rev_instr,
                    terminator: Terminator::Ret(I64, Some(body)),
                },
                blocks: vec![],
            },
        };

        //println!("Instructions in block: {:?}", self.rev_instr);

        ("main", test)
    }

    // TODO: Undone, trying some things out
    pub fn get_cfg(self) -> CFG {
        let first_block = match self.first_basic_block {
            Some(bb) => bb,
            None => panic!("Cannot construct CFG with no entrypoint"),
        };

        CFG {
            initial: first_block,
            blocks: vec![],
        }
    }

    // Has to call code_gen_fdecl to generate functions
    pub fn codegen_prog(&self, program: &HoistedProgram) -> Program {
        let fun_decl_data = program.fun_decls.get(0).expect("Main function required");
        if program.fun_decls.len() != 1 {
            panic!("Only single function programs supported")
        }
        let main = self.codegen_fun_decl(fun_decl_data);
        // Should be vec![main], but doesn't work cause of Rust.
        Program { fun_decls: vec![] }
    }

    pub fn print_llvm_prog_debug(&self, program: &Program) {
        let fun_decl = &program.fun_decls.get(0).unwrap().1;
        let ret_type = &fun_decl.fun_type;
        println!("Function type: {:?}", ret_type);

        let cfg = &fun_decl.cfg;
        let initial_block = &cfg.initial;
        println!("Initial Basic Block: {:?}", initial_block);
    }
}


