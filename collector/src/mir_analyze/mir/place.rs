use super::mir::LocalID;

#[derive(Debug)]
pub enum Place {
    Local(LocalID),
}
