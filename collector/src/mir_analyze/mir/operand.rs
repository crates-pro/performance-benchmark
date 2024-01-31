use super::place::Place;

#[derive(Debug)]
pub enum Operand {
    COPY(Place),
    MOVE(Place),
    CONST(Const),
}

#[derive(Debug)]
pub struct Const {
    pub val: String,
}
