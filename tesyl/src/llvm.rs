use crate::llvm::Bop::Add;
use crate::llvm::Ty::I64;
use crate::tabsyn::TypedExp;

// LLVM Types - For now just simple stuff
#[derive(Debug)]
pub enum Ty {
    Void,
    I1,
    I8,
    I64,
}

#[derive(Clone, Copy, Debug)]
pub enum Operand {
    Const(i32),
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

    pub fn add_instruction(&mut self, instr: Instr) -> &Self {
        self.instructions.push(instr);
        self
    }
}

pub struct CFG {
    initial: BasicBlock,
    blocks: Vec<BasicBlock>,
}

impl CFG {
    // This is just some testing stuff
    pub fn cfg_test(texp: TypedExp) {
        println!("Testing from here");
        let mut bb = BasicBlock::new(String::from("01"));
        let v1 = Operand::Const(42);
        bb.add_instruction(Instr::BinOp(Add, I64, v1, v1));
        println!("The basic block is: {:?}", bb);
    }
}
