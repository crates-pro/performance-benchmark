use std::str::FromStr;

use super::mir::ModuledIdentifier;

#[derive(Debug)]
pub enum Ty {
    Unit,
    Bool,
    I32,
    SelfDef(ModuledIdentifier),
    Tuple(Vec<Ty>),
    Ref(Box<Ty>),
    UND,
}

impl Default for Ty {
    fn default() -> Self {
        Self::UND
    }
}

impl FromStr for Ty {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(c) = s.chars().nth(0) {
            if c == '&' {
                return Ok(Self::Ref(Box::new(Self::from_str(&s[1..])?)));
            }
        }

        match s {
            "()" => Ok(Self::Unit),
            "i32" => Ok(Self::I32),
            "bool" => Ok(Self::Bool),
            "undef" => Ok(Self::UND),
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