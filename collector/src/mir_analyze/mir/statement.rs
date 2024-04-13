use super::{mir::LocalID, place::Place, rvalue::Rvalue,operand::Operand};

#[derive(Debug)]
pub enum Statement {
    Assign(Assign),
    StorageLive(LocalID),
    StorageDead(LocalID),
    ConstEvalCounter,
    Intrinsic(Box<NonDivergingIntrinsic>),
    SetDiscriminant(SetDiscriminant),
    Nop,
}

#[derive(Debug)]
pub struct Assign {
    pub place: Place,
    pub rvalue: Rvalue,
}

#[derive(Debug)]
pub enum NonDivergingIntrinsic {
    Assume(Operand),
    //CopyNonOverlapping(CopyNonOverlapping<'tcx>),
}

#[derive(Debug)]
pub struct SetDiscriminant {
    pub place: Place,
    pub variant_index: String,
}
