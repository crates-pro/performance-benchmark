use std::str::FromStr;

use super::{basic_block::BasicBlockID, mir::ModuledIdentifier, operand::Operand, place::Place};

#[derive(Debug)]
pub enum Terminator {
    Return,
    Assert(Assert),
    Call(Call),
    SwitchInt(SwitchInt),
    Goto(BasicBlockID),
    Drop(Drop),
    UnReachable,
    UnwindResume,
}

#[derive(Debug)]
pub struct SwitchInt {
    pub operand: Operand,
    pub success: Vec<Target>,
}

#[derive(Debug)]
pub struct Target {
    pub line: u32,
}

#[derive(Debug)]
pub struct Assert {
    pub operand: Operand,
    pub expected: bool,
    pub msg: FormatStr,
    pub success: BasicBlockID,
    pub unwind: Option<UnwindAction>,
}

#[derive(Debug)]
pub struct Drop {
    pub place: Place,
    pub replace: bool,
    pub success: BasicBlockID,
    pub unwind: Option<UnwindAction>,
}

#[derive(Debug)]
pub struct Call {
    pub callee: ModuledIdentifier,
    pub params: Vec<Operand>,
    pub recv: Option<Place>,
    pub success: Option<BasicBlockID>,
    pub unwind: Option<UnwindAction>,
}

#[derive(Debug)]
pub enum UnwindAction {
    CleanUp(u32),
    Continue,
    UnReachable,
    Terminate(UnwindTerminateReason),
}

#[derive(Debug)]
pub enum UnwindTerminateReason {
    //Abi,
    Incleanup,
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
        } else if s == "cleanup" {
            Ok(Self::Terminate(UnwindTerminateReason::Incleanup))
        } else if s == "terminate" {
            Ok(Self::Terminate(UnwindTerminateReason::Incleanup))
        } else {
            Err(format!("Fail to convert {} into a BasicBlockID.", s))
        }
    }
}

#[derive(Debug)]
pub struct FormatStr {
    pub msg: String,
    pub args: Vec<Operand>,
}
