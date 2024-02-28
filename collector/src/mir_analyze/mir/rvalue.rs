use super::{mir::ModuledIdentifier, operand::Operand, place::Place, ty::Ty};

#[derive(Debug)]
pub enum Rvalue {
    Use(Operand),
    BinaryOp(BinaryOp),
    Aggregate(Aggregate),
    Cast(Cast),
    Ref(Ref),
    UnaryOp(UnaryOp),
    //CopyForDeref(Place),
}
#[derive(Debug)]
pub enum UnaryOp {
    Neg(Neg),
    Not(Not),
}
#[derive(Debug)]
pub struct Neg {
    pub operand: Operand,
}
#[derive(Debug)]
pub struct Not {
    pub operand: Operand,
}

#[derive(Debug)]
pub struct BinaryOp {
    pub op_kind: BinaryOpKind,
    pub lhs: Operand,
    pub rhs: Operand,
}

#[derive(Debug)]
pub enum BinaryOpKind {
    CheckedAdd,
    CheckedSub,
    CheckedMul,
    Eq,
    BitAnd,
    Div,
    Ge,
    Gt,
    Le,
    BitXor,
    BitOr,
    Rem,
    Lt,
    Shl,
    Shr,
}

#[derive(Debug)]
pub struct Aggregate {
    pub aggregate_kind: AggregateKind,
    pub elements: Vec<Operand>,
}

#[derive(Debug)]
pub enum AggregateKind {
    Array,
    Tuple,
    Struct(ModuledIdentifier),
}

#[derive(Debug)]
pub struct Cast {
    pub cast_kind: CastKind,
    pub operand: Operand,
    pub ty: Ty,
}

#[derive(Debug)]
pub enum CastKind {
    PointerCoercion(PointerCoercion),
    IntToInt,
    // PointerExposeAddress,
    // PointerFromExposedAddress,
    // DynStar,
    // FloatToInt,
    // FloatToFloat,
    // IntToFloat,
    // PtrToPtr,
    // FnPtrToPtr,
    // Transmute,
}

#[derive(Debug)]
pub enum PointerCoercion {
    // ReifyFnPointer,
    // UnsafeFnPointer,
    //ClosureFnPointer(Unsafety),
    // MutToConstPointer,
    // ArrayToPointer,
    Unsize,
}

// #[derive(Debug)]
// pub enum Unsafety {
//    Unsafe,
//    Normal,
//}

#[derive(Debug)]
pub struct Ref {
    pub place: Place,
    pub borrow_kind: BorrowKind,
}

#[derive(Debug)]
pub enum BorrowKind {
    Shared,
    Mut,
}
