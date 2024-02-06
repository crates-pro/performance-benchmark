use super::{operand::Operand, place::Place, ty::Ty};

#[derive(Debug)]
pub enum Rvalue {
    Use(Operand),
    BinaryOp(BinaryOp),
    Aggregate(Aggregate),
    Cast(Cast),
    Ref(Ref),
    UnaryOp(UnaryOp),
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
pub enum BinaryOp {
    CheckedAdd(CheckedAdd),
    CheckedSub(CheckedSub),
    CheckedMul(CheckedMul),
    Eq(Eq),
    BitAnd(BitAnd),
    Div(Div),
    Ge(Ge),
    Gt(Gt),
    Le(Le),
    BitXor(BitXor),
    BitOr(BitOr),
    Rem(Rem),
    Lt(Lt),
    Shl(Shl),
    Shr(Shr),
}
#[derive(Debug)]
pub struct CheckedAdd {
    pub lhs: Operand,
    pub rhs: Operand,
}

#[derive(Debug)]
pub struct CheckedSub {
    pub lhs: Operand,
    pub rhs: Operand,
}
#[derive(Debug)]
pub struct CheckedMul {
    pub lhs: Operand,
    pub rhs: Operand,
}
#[derive(Debug)]
pub struct Eq {
    pub lhs: Operand,
    pub rhs: Operand,
}
#[derive(Debug)]
pub struct BitAnd {
    pub lhs: Operand,
    pub rhs: Operand,
}
#[derive(Debug)]
pub struct Div {
    pub lhs: Operand,
    pub rhs: Operand,
}
#[derive(Debug)]
pub struct Rem {
    pub lhs: Operand,
    pub rhs: Operand,
}

#[derive(Debug)]
pub struct Lt {
    pub lhs: Operand,
    pub rhs: Operand,
}

#[derive(Debug)]
pub struct Shl {
    pub lhs: Operand,
    pub rhs: Operand,
}

#[derive(Debug)]
pub struct Shr {
    pub lhs: Operand,
    pub rhs: Operand,
}

#[derive(Debug)]
pub struct BitXor {
    pub lhs: Operand,
    pub rhs: Operand,
}

#[derive(Debug)]
pub struct BitOr {
    pub lhs: Operand,
    pub rhs: Operand,
}

#[derive(Debug)]
pub struct Gt {
    pub lhs: Operand,
    pub rhs: Operand,
}

#[derive(Debug)]
pub struct Le {
    pub lhs: Operand,
    pub rhs: Operand,
}

#[derive(Debug)]
pub struct Ge {
    pub lhs: Operand,
    pub rhs: Operand,
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
    // ClosureFnPointer(Unsafety),
    // MutToConstPointer,
    // ArrayToPointer,
    Unsize,
}

// #[derive(Debug)]
// pub enum Unsafety {
//     Unsafe,
//     Normal,
// }

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
