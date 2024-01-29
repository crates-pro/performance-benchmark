use super::mir::Operand;

#[derive(Debug)]

pub enum Rvalue {
    Use(Operand),
    BinaryOp(BinaryOp),
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
