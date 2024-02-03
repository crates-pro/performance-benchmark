use super::{mir::LocalID, ty::Ty};

#[derive(Debug)]
pub enum Place {
    Local(LocalID),
    Field(Field),
    Deref(Deref),
}

#[derive(Debug)]
pub struct Field {
    pub place: Box<Place>,
    pub field_idx: FieldIdx,
    pub field_type: Ty,
}

pub type FieldIdx = u32;

#[derive(Debug)]
pub struct Deref {
    pub place: Box<Place>
}
