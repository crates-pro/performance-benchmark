use super::{place::Place, rvalue::Rvalue};

#[derive(Debug)]
pub enum Statement {
    Assign(Assign),
}

#[derive(Debug)]
pub struct Assign {
    pub place: Place,
    pub rvalue: Rvalue,
}
