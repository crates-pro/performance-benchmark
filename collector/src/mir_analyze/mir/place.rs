use super::{mir::LocalID, ty::Ty};

#[derive(Debug)]
pub enum Place {
    Local(LocalID),
    Field(Field),
}

#[derive(Debug)]
pub struct Field {
    pub local_id: LocalID,
    pub field_idx: FieldIdx,
    pub field_type: Ty,
}

pub type FieldIdx = u32;
