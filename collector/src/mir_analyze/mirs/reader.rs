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
/// * function/method call `_i = domain/type_ascription::func(args) -> [return: bb_x, unwind: bb_y]`, e.g. _5 = <Arc<Mutex<i32>> as Deref>::deref(move _6)
/// * switch `_i = discriminant(_j); switchInt(move _i) -> [blocks]`
/// * field access `_i.x`
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
        Assign, Call, Const, Move, Reference, Value, Value::VAR, Var,
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
            }
        }
    }

    impl PartialEq for Assign {
        fn eq(&self, other: &Self) -> bool {
            self.left == other.left
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
    fn test_read_mir() {
        use super::MIR::{ASSIGN, MOVE, REF};
        use crate::mir_analyze::mirs::mir::{Assign, Const, Move, Value, Value::VAR, Var};

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
        ];

        results
            .iter()
            .for_each(|r| assert!(std_results.contains(r)));
        std_results
            .iter()
            .for_each(|r| assert!(results.contains(r)));
    }
}
