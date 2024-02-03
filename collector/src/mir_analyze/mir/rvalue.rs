use super::operand::Operand;

#[derive(Debug)]

pub enum Rvalue {
    Use(Operand),
    BinaryOp(BinaryOp),
    Aggregate(Aggregate),
}

#[derive(Debug)]
pub enum BinaryOp {
    CheckedAdd(CheckedAdd),
}

#[derive(Debug)]
pub struct CheckedAdd {
    pub lhs: Operand,
    pub rhs: Operand,
}

#[derive(Debug)]
pub struct Aggregate {
    
}
