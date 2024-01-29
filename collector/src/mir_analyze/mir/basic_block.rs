use std::str::FromStr;

use super::{statement::Statement, terminator::Terminator};

pub type BasicBlocks = Vec<BasicBlock>;

#[derive(Debug)]
pub struct BasicBlock {
    pub bbid: BasicBlockID,
    pub statements: Vec<Statement>,
    pub terminator: Option<Terminator>,
}

#[derive(Debug)]
pub enum BasicBlockID {
    Number(u32),
    Continue,
}

impl FromStr for BasicBlockID {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(id) = u32::from_str(&s[2..]) {
            Ok(Self::Number(id))
        } else if s == "continue" {
            Ok(Self::Continue)
        } else {
            Err(format!("Fail to convert {} into a BasicBlockID.", s))
        }
    }
}
