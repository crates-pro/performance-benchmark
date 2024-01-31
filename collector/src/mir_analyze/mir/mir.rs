use super::{
    basic_block::BasicBlocks, place::Place, scope::{LocalDefs, Scopes, VarDebugInfos}, ty::Ty
};

/// MIRs represent the structured MIR code of a project.
#[derive(Debug)]
pub struct MIRs {
    pub functions: Vec<Function>,
}

#[derive(Debug)]
pub struct Function {
    pub label: ModuledIdentifier,
    pub bbs: BasicBlocks,
    pub params: Params,
    pub var_debug_infos: VarDebugInfos,
    pub local_defs: LocalDefs,
    pub scopes: Scopes,
    pub ret_ty: Ty,
}

pub type Params = Vec<Param>;

#[derive(Debug)]
pub struct Param {
    pub local_id: LocalID,
    pub ty: Ty,
}

pub type ModuledIdentifier = Vec<String>;

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

#[derive(Debug)]
pub struct Local {
    pub local_id: LocalID,
    pub ty: Ty,
    pub mutability: bool,
}

pub type LocalID = u32;
