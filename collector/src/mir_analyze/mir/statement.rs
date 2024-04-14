use super::{mir::LocalID, place::Place, rvalue::Rvalue};

#[derive(Debug)]
pub enum Statement {
    Assign(Assign),
    StorageLive(LocalID),
    StorageDead(LocalID),
}

#[derive(Debug)]
pub struct Assign {
    pub place: Place,
    pub rvalue: Rvalue,
}
