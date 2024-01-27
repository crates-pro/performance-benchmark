use std::str::FromStr;

#[derive(Debug)]
pub enum Ty {
    Unit,
    UserDef(String),
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
        match s {
            "()" => Ok(Self::Unit),
            _ => Ok(Self::UserDef(s.to_string())),
        }
    }
}
