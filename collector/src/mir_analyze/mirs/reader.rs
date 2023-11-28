use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use super::mir::MIR;

/// `read_mir` will take a mir file as input,
/// count the mir lines and classify the mirs into:
///
/// * type bounding `let _i = some_type`
/// * assignment `_i = _j` or `_i = cosnt xxx`
/// * move `_i = move _j`
/// * ref `_i = &_j` or `_i = &mut _j`
/// * type-cast `_i = _j as xxx`
/// * function/method call `_i = domain/type_ascription::func(args) -> [return: bb_x, unwind: bb_y] | return bb_x`, e.g. _5 = <Arc<Mutex<i32>> as Deref>::deref(move _6)
/// * switch `_i = discriminant(_j); switchInt(move _i) -> [blocks]`
/// * field access `_i = move (_j.x type) | &mut (_j.x type)| &(_j.x type)`
pub fn read_mir(path: PathBuf) -> anyhow::Result<Vec<MIR>> {
    let mut mirs = vec![];
    let file = File::open(path)?;

    let mut reader = BufReader::new(file);

    let mut buf = String::new();
    while reader.read_line(&mut buf)? > 0 {
        if let Some(mir) = MIR::new(&buf) {
            mirs.push(mir);
        }

        buf.clear();
    }

    Ok(mirs)
}

mod test {

    use super::MIR;
    use crate::mir_analyze::mirs::mir::{
        Assign, Call, Const, FieldAccess, Move, Param, Reference, Value, Value::VAR, Var,
        FIELDACCESS,
    };

    impl PartialEq for MIR {
        fn eq(&self, other: &Self) -> bool {
            match self {
                MIR::ASSIGN(assign) => match other {
                    MIR::ASSIGN(o_assign) => assign == o_assign,
                    _ => false,
                },
                MIR::MOVE(mv) => match other {
                    MIR::MOVE(o_move) => mv == o_move,
                    _ => false,
                },
                MIR::CALL(call) => match other {
                    MIR::CALL(o_call) => call == o_call,
                    _ => false,
                },
                MIR::REF(reference) => match other {
                    MIR::REF(o_ref) => reference == o_ref,
                    _ => false,
                },
                MIR::FIELDACCESS(field_access) => match other {
                    MIR::FIELDACCESS(o_field_access) => field_access == o_field_access,
                    _ => false,
                },
            }
        }
    }

    impl PartialEq for Assign {
        fn eq(&self, other: &Self) -> bool {
            self.left == other.left
        }
    }

    impl PartialEq for FIELDACCESS {
        fn eq(&self, other: &Self) -> bool {
            match self {
                FIELDACCESS::MOVE(m) => match other {
                    FIELDACCESS::MOVE(o_m) => m == o_m,
                    _ => false,
                },
                FIELDACCESS::REF(r) => match other {
                    FIELDACCESS::REF(o_r) => r == o_r,
                    _ => false,
                },
                FIELDACCESS::REFMUT(r) => match other {
                    FIELDACCESS::REFMUT(o_r) => r == o_r,
                    _ => false,
                },
            }
        }
    }

    impl PartialEq for FieldAccess {
        fn eq(&self, other: &Self) -> bool {
            self.left == other.left
                && self.struct_var == other.struct_var
                && self.field_num == other.field_num
        }
    }

    impl PartialEq for Reference {
        fn eq(&self, other: &Self) -> bool {
            self.src == other.src && self.dst == other.dst
        }
    }

    impl PartialEq for Call {
        fn eq(&self, other: &Self) -> bool {
            self.ret == other.ret
                && self.label == other.label
                && self.params == other.params
                && self.next_block == other.next_block
                && self.unwind_block == other.unwind_block
        }
    }

    impl PartialEq for Move {
        fn eq(&self, other: &Self) -> bool {
            self.left == other.left && self.right == other.right
        }
    }

    impl PartialEq for Param {
        fn eq(&self, other: &Self) -> bool {
            match self {
                Param::MOVE(m) => match other {
                    Param::MOVE(o) => m == o,
                    _ => false,
                },
                Param::VAR(v) => match other {
                    Param::VAR(o) => v == o,
                    _ => false,
                },
                Param::COSNT(c) => match other {
                    Param::COSNT(o) => c == o,
                    _ => false,
                },
            }
        }
    }

    impl PartialEq for Value {
        fn eq(&self, other: &Self) -> bool {
            match self {
                Value::CONST(c) => match other {
                    Value::CONST(o_c) => c == o_c,
                    VAR(_) => false,
                },
                VAR(v) => match other {
                    Value::CONST(_) => false,
                    VAR(o_v) => v == o_v,
                },
            }
        }
    }

    impl PartialEq for Const {
        fn eq(&self, other: &Self) -> bool {
            self.val == other.val
        }
    }

    impl PartialEq for Var {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    #[test]
    fn test_read_mir_condvar() {
        use super::MIR::{ASSIGN, CALL, MOVE, REF};
        use crate::mir_analyze::mirs::mir::{
            Assign, Call, Const, Move, Param, Value, Value::VAR, Var,
        };

        use crate::mir_analyze::mirs::reader::read_mir;
        use std::path::PathBuf;

        let mir_file = PathBuf::from("test/mir_analyze/reader/condvar-9b2e97b194975c57.mir");

        let results = read_mir(mir_file).unwrap();

        let std_results = vec![
            ASSIGN(Assign {
                left: VAR(Var { id: 49 }),
                right: Value::CONST(Const {
                    val: "false".to_string(),
                }),
            }),
            ASSIGN(Assign {
                left: VAR(Var { id: 50 }),
                right: Value::CONST(Const {
                    val: "false".to_string(),
                }),
            }),
            ASSIGN(Assign {
                left: VAR(Var { id: 50 }),
                right: Value::CONST(Const {
                    val: "true".to_string(),
                }),
            }),
            ASSIGN(Assign {
                left: VAR(Var { id: 50 }),
                right: Value::CONST(Const {
                    val: "false".to_string(),
                }),
            }),
            ASSIGN(Assign {
                left: VAR(Var { id: 49 }),
                right: Value::CONST(Const {
                    val: "true".to_string(),
                }),
            }),
            ASSIGN(Assign {
                left: VAR(Var { id: 11 }),
                right: Value::CONST(Const {
                    val: "0_i32".to_string(),
                }),
            }),
            ASSIGN(Assign {
                left: VAR(Var { id: 18 }),
                right: VAR(Var { id: 19 }),
            }),
            ASSIGN(Assign {
                left: VAR(Var { id: 23 }),
                right: VAR(Var { id: 11 }),
            }),
            ASSIGN(Assign {
                left: VAR(Var { id: 49 }),
                right: Value::CONST(Const {
                    val: "false".to_string(),
                }),
            }),
            ASSIGN(Assign {
                left: VAR(Var { id: 47 }),
                right: Value::CONST(Const {
                    val: "_".to_string(),
                }),
            }),
            ASSIGN(Assign {
                left: VAR(Var { id: 33 }),
                right: VAR(Var { id: 34 }),
            }),
            ASSIGN(Assign {
                left: VAR(Var { id: 48 }),
                right: Value::CONST(Const {
                    val: "_".to_string(),
                }),
            }),
            ASSIGN(Assign {
                left: VAR(Var { id: 49 }),
                right: Value::CONST(Const {
                    val: "false".to_string(),
                }),
            }),
            ASSIGN(Assign {
                left: VAR(Var { id: 50 }),
                right: Value::CONST(Const {
                    val: "false".to_string(),
                }),
            }),
            ASSIGN(Assign {
                left: VAR(Var { id: 32 }),
                right: Value::CONST(Const {
                    val: "false".to_string(),
                }),
            }),
            ASSIGN(Assign {
                left: VAR(Var { id: 4 }),
                right: VAR(Var { id: 5 }),
            }),
            ASSIGN(Assign {
                left: VAR(Var { id: 32 }),
                right: Value::CONST(Const {
                    val: "true".to_string(),
                }),
            }),
            ASSIGN(Assign {
                left: VAR(Var { id: 7 }),
                right: Value::CONST(Const {
                    val: "0_i32".to_string(),
                }),
            }),
            ASSIGN(Assign {
                left: VAR(Var { id: 9 }),
                right: VAR(Var { id: 7 }),
            }),
            ASSIGN(Assign {
                left: VAR(Var { id: 16 }),
                right: VAR(Var { id: 17 }),
            }),
            ASSIGN(Assign {
                left: VAR(Var { id: 32 }),
                right: Value::CONST(Const {
                    val: "false".to_string(),
                }),
            }),
            ASSIGN(Assign {
                left: VAR(Var { id: 32 }),
                right: Value::CONST(Const {
                    val: "true".to_string(),
                }),
            }),
            ASSIGN(Assign {
                left: VAR(Var { id: 31 }),
                right: Value::CONST(Const {
                    val: "_".to_string(),
                }),
            }),
            ASSIGN(Assign {
                left: VAR(Var { id: 32 }),
                right: Value::CONST(Const {
                    val: "false".to_string(),
                }),
            }),
            MOVE(Move {
                left: VAR(Var { id: 19 }),
                right: VAR(Var { id: 2 }),
            }),
            MOVE(Move {
                left: VAR(Var { id: 38 }),
                right: VAR(Var { id: 9 }),
            }),
            MOVE(Move {
                left: VAR(Var { id: 2 }),
                right: VAR(Var { id: 14 }),
            }),
            REF(Reference {
                src: VAR(Var { id: 6 }),
                dst: VAR(Var { id: 1 }),
            }),
            REF(Reference {
                src: VAR(Var { id: 8 }),
                dst: VAR(Var { id: 3 }),
            }),
            REF(Reference {
                src: VAR(Var { id: 20 }),
                dst: VAR(Var { id: 1 }),
            }),
            REF(Reference {
                src: VAR(Var { id: 15 }),
                dst: VAR(Var { id: 16 }),
            }),
            REF(Reference {
                src: VAR(Var { id: 31 }),
                dst: VAR(Var { id: 11 }),
            }),
            REF(Reference {
                src: VAR(Var { id: 28 }),
                dst: VAR(Var { id: 29 }),
            }),
            REF(Reference {
                src: VAR(Var { id: 35 }),
                dst: VAR(Var { id: 3 }),
            }),
            REF(Reference {
                src: VAR(Var { id: 46 }),
                dst: VAR(Var { id: 1 }),
            }),
            REF(Reference {
                src: VAR(Var { id: 43 }),
                dst: VAR(Var { id: 44 }),
            }),
            REF(Reference {
                src: VAR(Var { id: 0 }),
                dst: VAR(Var { id: 1 }),
            }),
            REF(Reference {
                src: VAR(Var { id: 13 }),
                dst: VAR(Var { id: 2 }),
            }),
            REF(Reference {
                src: VAR(Var { id: 21 }),
                dst: VAR(Var { id: 2 }),
            }),
            REF(Reference {
                src: VAR(Var { id: 30 }),
                dst: VAR(Var { id: 7 }),
            }),
            REF(Reference {
                src: VAR(Var { id: 27 }),
                dst: VAR(Var { id: 28 }),
            }),
            REF(Reference {
                src: VAR(Var { id: 0 }),
                dst: VAR(Var { id: 1 }),
            }),
            CALL(Call {
                ret: Var { id: 2 },
                label: "Mutex::<bool>::new".to_string(),
                params: vec![Param::COSNT(Const {
                    val: "const false".to_string(),
                })],
                next_block: "bb1;".to_string(),
                unwind_block: None,
            }),
            CALL(Call {
                ret: Var { id: 1 },
                label: "Arc::<Mutex<bool>>::new".to_string(),
                params: vec![Param::MOVE(Var { id: 2 })],
                next_block: "bb2;".to_string(),
                unwind_block: None,
            }),
            CALL(Call {
                ret: Var { id: 4 },
                label: "Condvar::new".to_string(),
                params: vec![],
                next_block: "bb3".to_string(),
                unwind_block: Some("bb32".to_string()),
            }),
            CALL(Call {
                ret: Var { id: 3 },
                label: "Arc::<Condvar>::new".to_string(),
                params: vec![Param::MOVE(Var { id: 4 })],
                next_block: "bb4".to_string(),
                unwind_block: Some("bb32".to_string()),
            }),
            CALL(Call {
                ret: Var { id: 5 },
                label: "<Arc<Mutex<bool>> as Clone>::clone".to_string(),
                params: vec![Param::MOVE(Var { id: 6 })],
                next_block: "bb5".to_string(),
                unwind_block: Some("bb31".to_string()),
            }),
            CALL(Call {
                ret: Var { id: 7 },
                label: "<Arc<Condvar> as Clone>::clone".to_string(),
                params: vec![Param::MOVE(Var { id: 8 })],
                next_block: "bb6".to_string(),
                unwind_block: Some("bb37".to_string()),
            }),
            CALL(Call {
                ret: Var { id: 9 },
                label: "spawn::<[closure@src/main.rs:11:21: 11:28], ()>".to_string(),
                params: vec![Param::MOVE(Var { id: 10 })],
                next_block: "bb7".to_string(),
                unwind_block: Some("bb37".to_string()),
            }),
            CALL(Call {
                ret: Var { id: 13 },
                label: "Duration::from_millis".to_string(),
                params: vec![Param::COSNT(Const {
                    val: "const 1000_u64".to_string(),
                })],
                next_block: "bb9".to_string(),
                unwind_block: Some("bb35".to_string()),
            }),
            CALL(Call {
                ret: Var { id: 12 },
                label: "sleep".to_string(),
                params: vec![Param::MOVE(Var { id: 13 })],
                next_block: "bb10".to_string(),
                unwind_block: Some("bb35".to_string()),
            }),
            CALL(Call {
                ret: Var { id: 19 },
                label: "<Arc<Mutex<bool>> as Deref>::deref".to_string(),
                params: vec![Param::MOVE(Var { id: 20 })],
                next_block: "bb11".to_string(),
                unwind_block: Some("bb35".to_string()),
            }),
            CALL(Call {
                ret: Var { id: 17 },
                label: "Mutex::<bool>::lock".to_string(),
                params: vec![Param::MOVE(Var { id: 18 })],
                next_block: "bb12".to_string(),
                unwind_block: Some("bb35".to_string()),
            }),
            CALL(Call {
                ret: Var { id: 16 },
                label: "Result::<MutexGuard<'_, bool>, PoisonError<MutexGuard<'_, bool>>>::unwrap"
                    .to_string(),
                params: vec![Param::MOVE(Var { id: 17 })],
                next_block: "bb13".to_string(),
                unwind_block: Some("bb35".to_string()),
            }),
            CALL(Call {
                ret: Var { id: 14 },
                label: "<MutexGuard<'_, bool> as DerefMut>::deref_mut".to_string(),
                params: vec![Param::MOVE(Var { id: 15 })],
                next_block: "bb14".to_string(),
                unwind_block: Some("bb30".to_string()),
            }),
            CALL(Call {
                ret: Var { id: 37 },
                label: "JoinHandle::<()>::join".to_string(),
                params: vec![Param::MOVE(Var { id: 38 })],
                next_block: "bb23".to_string(),
                unwind_block: Some("bb35".to_string()),
            }),
            CALL(Call {
                ret: Var { id: 30 },
                label: "core::fmt::ArgumentV1::<'_>::new_display::<i32>".to_string(),
                params: vec![Param::VAR(Var { id: 31 })],
                next_block: "bb19".to_string(),
                unwind_block: Some("bb35".to_string()),
            }),
            CALL(Call {
                ret: Var { id: 25 },
                label: "Arguments::<'_>::new_v1".to_string(),
                params: vec![Param::MOVE(Var { id: 26 }), Param::MOVE(Var { id: 27 })],
                next_block: "bb20".to_string(),
                unwind_block: Some("bb35".to_string()),
            }),
            CALL(Call {
                ret: Var { id: 24 },
                label: "_print".to_string(),
                params: vec![Param::MOVE(Var { id: 25 })],
                next_block: "bb21".to_string(),
                unwind_block: Some("bb35".to_string()),
            }),
            CALL(Call {
                ret: Var { id: 34 },
                label: "<Arc<Condvar> as Deref>::deref".to_string(),
                params: vec![Param::MOVE(Var { id: 35 })],
                next_block: "bb22".to_string(),
                unwind_block: Some("bb35".to_string()),
            }),
            CALL(Call {
                ret: Var { id: 32 },
                label: "Condvar::notify_one".to_string(),
                params: vec![Param::MOVE(Var { id: 33 })],
                next_block: "bb38".to_string(),
                unwind_block: Some("bb35".to_string()),
            }),
            CALL(Call {
                ret: Var { id: 36 },
                label: "Result::<(), Box<dyn Any + Send>>::unwrap".to_string(),
                params: vec![Param::MOVE(Var { id: 37 })],
                next_block: "bb24".to_string(),
                unwind_block: Some("bb35".to_string()),
            }),
            CALL(Call {
                ret: Var { id: 45 },
                label: "core::fmt::ArgumentV1::<'_>::new_debug::<Arc<Mutex<bool>>>".to_string(),
                params: vec![Param::VAR(Var { id: 46 })],
                next_block: "bb25".to_string(),
                unwind_block: Some("bb35".to_string()),
            }),
            CALL(Call {
                ret: Var { id: 40 },
                label: "Arguments::<'_>::new_v1".to_string(),
                params: vec![Param::MOVE(Var { id: 41 }), Param::MOVE(Var { id: 42 })],
                next_block: "bb26".to_string(),
                unwind_block: Some("bb35".to_string()),
            }),
            CALL(Call {
                ret: Var { id: 39 },
                label: "_print".to_string(),
                params: vec![Param::MOVE(Var { id: 40 })],
                next_block: "bb27".to_string(),
                unwind_block: Some("bb35".to_string()),
            }),
            CALL(Call {
                ret: Var { id: 5 },
                label: "<Arc<Mutex<bool>> as Deref>::deref".to_string(),
                params: vec![Param::MOVE(Var { id: 6 })],
                next_block: "bb1".to_string(),
                unwind_block: Some("bb19".to_string()),
            }),
            CALL(Call {
                ret: Var { id: 3 },
                label: "Mutex::<bool>::lock".to_string(),
                params: vec![Param::MOVE(Var { id: 4 })],
                next_block: "bb2".to_string(),
                unwind_block: Some("bb19".to_string()),
            }),
            CALL(Call {
                ret: Var { id: 2 },
                label: "Result::<MutexGuard<'_, bool>, PoisonError<MutexGuard<'_, bool>>>::unwrap"
                    .to_string(),
                params: vec![Param::MOVE(Var { id: 3 })],
                next_block: "bb3".to_string(),
                unwind_block: Some("bb19".to_string()),
            }),
            CALL(Call {
                ret: Var { id: 12 },
                label: "<MutexGuard<'_, bool> as Deref>::deref".to_string(),
                params: vec![Param::MOVE(Var { id: 13 })],
                next_block: "bb6".to_string(),
                unwind_block: Some("bb22".to_string()),
            }),
            CALL(Call {
                ret: Var { id: 17 },
                label: "<Arc<Condvar> as Deref>::deref".to_string(),
                params: vec![Param::MOVE(Var { id: 18 })],
                next_block: "bb8".to_string(),
                unwind_block: Some("bb22".to_string()),
            }),
            CALL(Call {
                ret: Var { id: 15 },
                label: "Condvar::wait::<bool>".to_string(),
                params: vec![Param::MOVE(Var { id: 16 }), Param::MOVE(Var { id: 19 })],
                next_block: "bb9".to_string(),
                unwind_block: Some("bb22".to_string()),
            }),
            CALL(Call {
                ret: Var { id: 14 },
                label: "Result::<MutexGuard<'_, bool>, PoisonError<MutexGuard<'_, bool>>>::unwrap"
                    .to_string(),
                params: vec![Param::MOVE(Var { id: 15 })],
                next_block: "bb10".to_string(),
                unwind_block: Some("bb22".to_string()),
            }),
            CALL(Call {
                ret: Var { id: 20 },
                label: "<MutexGuard<'_, bool> as DerefMut>::deref_mut".to_string(),
                params: vec![Param::MOVE(Var { id: 21 })],
                next_block: "bb12".to_string(),
                unwind_block: Some("bb22".to_string()),
            }),
            CALL(Call {
                ret: Var { id: 29 },
                label: "core::fmt::ArgumentV1::<'_>::new_display::<i32>".to_string(),
                params: vec![Param::VAR(Var { id: 30 })],
                next_block: "bb14".to_string(),
                unwind_block: Some("bb22".to_string()),
            }),
            CALL(Call {
                ret: Var { id: 24 },
                label: "Arguments::<'_>::new_v1".to_string(),
                params: vec![Param::MOVE(Var { id: 25 }), Param::MOVE(Var { id: 26 })],
                next_block: "bb15".to_string(),
                unwind_block: Some("bb22".to_string()),
            }),
            CALL(Call {
                ret: Var { id: 23 },
                label: "_print".to_string(),
                params: vec![Param::MOVE(Var { id: 24 })],
                next_block: "bb23".to_string(),
                unwind_block: Some("bb22".to_string()),
            }),
            MIR::FIELDACCESS(FIELDACCESS::MOVE(FieldAccess {
                left: Var { id: 11 },
                struct_var: Var { id: 21 },
                field_num: 0,
            })),
            MIR::FIELDACCESS(FIELDACCESS::REF(FieldAccess {
                left: Var { id: 6 },
                struct_var: Var { id: 1 },
                field_num: 0,
            })),
            MIR::FIELDACCESS(FIELDACCESS::REF(FieldAccess {
                left: Var { id: 18 },
                struct_var: Var { id: 1 },
                field_num: 1,
            })),
            MIR::FIELDACCESS(FIELDACCESS::MOVE(FieldAccess {
                left: Var { id: 7 },
                struct_var: Var { id: 22 },
                field_num: 0,
            })),
        ];

        results
            .iter()
            .for_each(|r| assert!(std_results.contains(r)));
        std_results
            .iter()
            .for_each(|r| assert!(results.contains(r)));
    }

    #[test]
    fn test_read_mir_field_access() {
        use super::MIR::{ASSIGN, CALL, REF};
        use crate::mir_analyze::mirs::mir::{Assign, Call, Const, Param, Value, Value::VAR, Var};

        use crate::mir_analyze::mirs::reader::read_mir;
        use std::path::PathBuf;

        let mir_file = PathBuf::from("test/mir_analyze/reader/field-access.mir");

        let results = read_mir(mir_file).unwrap();

        let std_results = vec![
            ASSIGN(Assign {
                left: VAR(Var { id: 16 }),
                right: Value::CONST(Const {
                    val: "false".to_string(),
                }),
            }),
            CALL(Call {
                ret: Var { id: 2 },
                label: "<String as From<&str>>::from".to_string(),
                params: vec![Param::COSNT(Const {
                    val: "const \"string\"".to_string(),
                })],
                next_block: "bb1".to_string(),
                unwind_block: Some("continue".to_string()),
            }),
            ASSIGN(Assign {
                left: VAR(Var { id: 16 }),
                right: Value::CONST(Const {
                    val: "true".to_string(),
                }),
            }),
            ASSIGN(Assign {
                left: VAR(Var { id: 15 }),
                right: Value::CONST(Const {
                    val: "_".to_string(),
                }),
            }),
            MIR::FIELDACCESS(FIELDACCESS::REF(FieldAccess {
                left: Var { id: 10 },
                struct_var: Var { id: 1 },
                field_num: 0,
            })),
            CALL(Call {
                ret: Var { id: 9 },
                label: "core::fmt::rt::Argument::<'_>::new_display::<String>".to_string(),
                params: vec![Param::VAR(Var { id: 10 })],
                next_block: "bb2".to_string(),
                unwind_block: Some("bb10".to_string()),
            }),
            REF(Reference {
                src: VAR(Var { id: 7 }),
                dst: VAR(Var { id: 8 }),
            }),
            CALL(Call {
                ret: Var { id: 4 },
                label: "Arguments::<'_>::new_v1".to_string(),
                params: vec![Param::MOVE(Var { id: 5 }), Param::MOVE(Var { id: 6 })],
                next_block: "bb3".to_string(),
                unwind_block: Some("bb10".to_string()),
            }),
            CALL(Call {
                ret: Var { id: 3 },
                label: "_print".to_string(),
                params: vec![Param::MOVE(Var { id: 4 })],
                next_block: "bb4".to_string(),
                unwind_block: Some("bb10".to_string()),
            }),
            ASSIGN(Assign {
                left: VAR(Var { id: 16 }),
                right: Value::CONST(Const {
                    val: "false".to_string(),
                }),
            }),
            MIR::FIELDACCESS(FIELDACCESS::MOVE(FieldAccess {
                left: Var { id: 12 },
                struct_var: Var { id: 1 },
                field_num: 0,
            })),
            MIR::FIELDACCESS(FIELDACCESS::REFMUT(FieldAccess {
                left: Var { id: 14 },
                struct_var: Var { id: 11 },
                field_num: 0,
            })),
            CALL(Call {
                ret: Var { id: 13 },
                label: "String::clear".to_string(),
                params: vec![Param::MOVE(Var { id: 14 })],
                next_block: "bb5".to_string(),
                unwind_block: Some("bb7".to_string()),
            }),
            ASSIGN(Assign {
                left: VAR(Var { id: 16 }),
                right: Value::CONST(Const {
                    val: "false".to_string(),
                }),
            }),
            REF(Reference {
                src: VAR(Var { id: 0 }),
                dst: VAR(Var { id: 1 }),
            }),
        ];

        results
            .iter()
            .for_each(|r| assert!(std_results.contains(r)));
        std_results
            .iter()
            .for_each(|r| assert!(results.contains(r)));
    }
}
