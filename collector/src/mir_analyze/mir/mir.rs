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
pub struct BinaryOp();
