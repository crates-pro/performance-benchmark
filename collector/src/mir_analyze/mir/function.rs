use super::{
    basic_block::BasicBlocks,
    mir::{LocalID, ModuledIdentifier},
    scope::{LocalDefs, Scopes, VarDebugInfos},
    ty::Ty,
    operand::Operand,
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

#[derive(Debug)]
pub struct NofnFunction {
    pub body: Function,
}

pub type PromotedID = u32;

#[derive(Debug)]
pub struct ConstBlock {
    pub const_var: ModuledIdentifier,
    pub ty: Ty,
    pub body: Function,
}

#[derive(Debug)]
pub struct FunctionName {
    pub name: String,
    pub params: Vec<Operand>,
}

#[derive(Debug)]
pub struct StaticStruct {
    pub body: Function,
}

pub type AllocParams = Vec<AllocParam>;
#[derive(Debug)]
pub struct AllocParam {
    pub name: String,
    pub val: ModuledIdentifier,
}

/*#[derive(Debug)]
pub struct AllocStruct {
    pub label: String,
    pub align: String,
    pub size: String,
}*/
#[derive(Debug)]
pub struct AllocStruct{
    pub label: String,
}