use super::ty::Ty;

/// MIRs represent the structured MIR code of a project.
#[derive(Debug)]
pub struct MIRs {
    pub functions: Vec<Function>,
    pub unrecognized_lines: Vec<String>,
}

#[derive(Debug)]
pub struct Function {
    pub label: ModuledIdentifier,
    pub bbs: BasicBlocks,
    pub params: Vec<Local>,
    pub ret_ty: Ty,
    pub unrecognized_lines: Vec<String>,
}

pub type ModuledIdentifier = Vec<String>;

pub type BasicBlocks = Vec<BasicBlock>;

#[derive(Debug)]
pub struct BasicBlock {
    pub bbid: u32,
    pub statements: Vec<Statement>,
    pub terminator: Option<Terminator>,
}

#[derive(Debug)]
pub enum Statement {
    BinaryOp(BinaryOp),
}

#[derive(Debug)]
pub enum Terminator {}

#[derive(Debug)]
pub enum BinaryOp {
    CheckedAdd(CheckedAdd),
}

#[derive(Debug)]
pub struct CheckedAdd {
    pub res: Operand,
    pub lhs: Operand,
    pub rhs: Operand,
}

#[derive(Debug)]
pub enum Operand {
    LOCAL(Local),
}

#[derive(Debug)]
pub struct Local {
    pub local_id: u32,
    pub ty: Ty,
}
