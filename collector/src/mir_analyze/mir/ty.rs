use std::str::FromStr;

use super::mir::ModuledIdentifier;

#[derive(Debug)]
pub enum Ty {
    Unit,
    Bool,
    I32,
    SelfDef(ModuledIdentifier),
    Tuple(Vec<Ty>),
    UND,
    U32, 
}

impl Default for Ty {
    fn default() -> Self {
        Self::UND
    }
}

impl FromStr for Ty {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "()" => Ok(Self::Unit),
            "i32" => Ok(Self::I32),
            "u32" => Ok(Self::U32),
            "bool" => Ok(Self::Bool),
            _ => Ok(Self::SelfDef(
                s.split("::").map(|s| s.to_string()).collect(),
            )),
        }
    }
}

impl From<ModuledIdentifier> for Ty {
    fn from(value: ModuledIdentifier) -> Self {
        Self::SelfDef(value)
    }
}
