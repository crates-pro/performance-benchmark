use super::{basic_block::BasicBlockID, mir::Operand};

#[derive(Debug)]
pub enum Terminator {
    Return,
    Assert(Assert),
}

#[derive(Debug)]
pub struct Assert {
    pub operand: Operand,
    pub expected: bool,
    pub msg: String,
    pub success: BasicBlockID,
    pub unwind: BasicBlockID,
}
