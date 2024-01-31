use std::str::FromStr;

use super::{basic_block::BasicBlockID, mir::ModuledIdentifier, operand::Operand, place::Place};

#[derive(Debug)]
pub enum Terminator {
    Return,
    Assert(Assert),
    Call(Call),
}

#[derive(Debug)]
pub struct Assert {
    pub operand: Operand,
    pub expected: bool,
    pub msg: String,
    pub success: BasicBlockID,
    pub unwind: UnwindAction,
}

#[derive(Debug)]
pub struct Call {
    pub callee: ModuledIdentifier,
    pub params: Vec<Operand>,
    pub recv: Place,
    pub success: Option<BasicBlockID>,
    pub unwind: UnwindAction,
}

#[derive(Debug)]
pub enum UnwindAction {
    CleanUp(u32),
    Continue,
    UnReachable,
}

impl FromStr for UnwindAction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(id) = u32::from_str(&s[2..]) {
            Ok(Self::CleanUp(id))
        } else if s == "continue" {
            Ok(Self::Continue)
        } else if s == "unreachable" {
            Ok(Self::UnReachable)
        } else {
            Err(format!("Fail to convert {} into a BasicBlockID.", s))
        }
    }
}
