use crate::ast::Exp;
use crate::llvm::Bop::Add;
use crate::llvm::Operand::Const;
use crate::llvm::Ty::I64;
use crate::tabsyn::{TypedExp, TypedExpBase};

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
    Const(i64), //Constants
                Id(i32), //Variables TODO MAKE STRING
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
    instructions: Vec<Instr>, // (Option(uid), instruc)
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

pub struct CFG {
    initial: BasicBlock,
    blocks: Vec<BasicBlock>,
}

// Potential label to store result and the instruction
type Instruction = (Option<String>, Instr);

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
        self.rev_instr.push(instr)
    }

    // Playground for now - maybe return non-mut builder
    // with updated bindings?
    pub fn construct_cfg(&mut self, typed_prog: TypedExp) -> Operand {
        match typed_prog.exp {
            TypedExpBase::IntExp { value } => Const(value),

            TypedExpBase::BinOpExp { left, op, right } => {
                // TODO: More stuff than just this - id's and stuff...
                let left_op = self.construct_cfg(*left);
                let right_op = self.construct_cfg(*right);

                let lbl = String::from("label");

                // Test
                self.add_instr((
                    Some(lbl),
                    Instr::BinOp(Bop::Add, Ty::I64, left_op, right_op),
                ));

                Operand::Id(0)
            }
            _ => panic!("Unimpl"),
        }
    }
}

impl CFG {
    // This is just some testing stuff
    pub fn cfg_test(texp: TypedExp) {
        //println!("Testing from here");
        //let mut bb = BasicBlock::new(String::from("01"));
        //let v1 = Operand::Const(42);
        //bb.add_instruction(Instr::BinOp(Add, I64, v1, v1));
        //println!("The basic block is: {:?}", bb);
    }
}
