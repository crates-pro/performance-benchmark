use super::{
    function::{ConstBlock, Function, PromotedFunction,NofnFunction,StaticStruct},
    ty::Ty,
};

/// MIRs represent the structured MIR code of a project.
#[derive(Debug)]
pub struct MIRs {
    pub functions: Vec<Function>,
    pub promoted_functions: Vec<PromotedFunction>,
    pub const_blocks: Vec<ConstBlock>,
    pub nofn_functions: Vec<NofnFunction>,
    pub static_structs: Vec<StaticStruct>,
}

pub type ModuledIdentifier = Vec<String>;

#[derive(Debug)]
pub struct Local {
    pub local_id: LocalID,
    pub ty: Ty,
    pub mutability: bool,
}

pub type LocalID = u32;
