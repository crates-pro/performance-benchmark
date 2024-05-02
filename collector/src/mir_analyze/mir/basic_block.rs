use super::{statement::Statement, terminator::Terminator};

pub type BasicBlocks = Vec<BasicBlock>;

#[derive(Debug)]
pub struct BasicBlock {
    pub bbid: BasicBlockID,
    pub statements: Vec<Statement>,
    pub terminator: Option<Terminator>,
}

pub type BasicBlockID = u32;
