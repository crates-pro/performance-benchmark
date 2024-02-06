use super::{
    basic_block::BasicBlocks,
    mir::{LocalID, ModuledIdentifier},
    scope::{LocalDefs, Scopes, VarDebugInfos},
    ty::Ty,
};

#[derive(Debug)]
pub struct Function {
    pub label: ModuledIdentifier,
    pub params: Params,
    pub ret_ty: Ty,
    pub var_debug_infos: VarDebugInfos,
    pub local_defs: LocalDefs,
    pub scopes: Scopes,
    pub bbs: BasicBlocks,
}

pub type Params = Vec<Param>;

#[derive(Debug)]
pub struct Param {
    pub local_id: LocalID,
    pub ty: Ty,
}

#[derive(Debug)]
pub struct PromotedFunction {
    pub promoted_id: PromotedID,
    pub body: Function,
}

pub type PromotedID = u32;

#[derive(Debug)]
pub struct ConstBlock {
    pub const_var: ModuledIdentifier,
    pub ty: Ty,
    pub body: Function,
}
