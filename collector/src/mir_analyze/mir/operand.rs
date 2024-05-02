use super::{place::Place, ty::Ty};

#[derive(Debug)]
pub enum Operand {
    COPY(Place),
    MOVE(Place),
    CONST(Const),
}

#[derive(Debug)]
pub struct Const {
    pub ty: Ty,
    pub val: String,
}
