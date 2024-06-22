use super::{
    mir::{LocalID, ModuledIdentifier},
    operand::Operand,
    ty::Ty,
};
use std::str::FromStr;

#[derive(Debug)]
pub enum Place {
    Local(LocalID),
    Field(Field),
    Deref(Deref),
    Downcast(Downcast),
    Subslice(Subslice),
    Empty,
    Index(Index),
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
    pub place: Box<Place>,
}

#[derive(Debug)]
pub struct Index {
    pub place: Box<Place>,
    //pub id: String,
}

#[derive(Debug)]
pub struct Subslice {
    pub calu_type: String,
    pub slice_type: Ty,
}

pub type Downcast = Vec<String>;

impl From<ModuledIdentifier> for Place {
    fn from(value: ModuledIdentifier) -> Self {
        if value.len() == 1 {
            if let Some(s) = value.get(0) {
                if let Ok(t) = u32::from_str(&s[1..]) {
                    Self::Local(t)
                } else {
                    Self::Empty
                }
            } else {
                Self::Empty
            }
        } else {
            /*if let Some(slice) = value.get(0..2) {
                let set = slice[0].clone();
                let variant = slice[1].clone();
                let downcast = Downcast { set, variant};
                Self::Downcast(downcast)
            }
            else{
                Self::Empty
            }*/
            Self::Downcast(value)
        }
    }
}
