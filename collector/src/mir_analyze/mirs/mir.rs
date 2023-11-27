use regex::Regex;

#[derive(Debug)]
pub enum MIR {
    ASSIGN(Assign),
    MOVE(Move),
    CALL(Call),
}

pub type VarId = u32;
pub type BlockId = u32;

#[derive(Debug)]
pub struct Assign {
    pub left: Value,
    pub right: Value,
}

#[derive(Debug)]
pub struct Move {
    pub left: Value,
    pub right: Value,
}

#[derive(Debug)]
pub struct Call {
    pub ret: VarId,
    pub label: String,
    pub params: Vec<Value>,
    pub next_block: BlockId,
    pub unwind_block: BlockId,
}

#[derive(Debug)]
pub enum Value {
    CONST(Const),
    VAR(Var),
}

#[derive(Debug)]
pub struct Const {
    pub val: String,
}

#[derive(Debug)]
pub struct Var {
    pub id: VarId,
}

impl MIR {
    /// Match a mir from a string.
    /// * type bounding `let _i = some_type`
    /// * assignment `_i = _j` or `_i = cosnt xxx`
    /// * move `_i = move _j`
    /// * ref `_i = &_j` or `_i = &mut _j`
    /// * function/method call `_i = domain/type_ascription::func(args) -> [return: bb_x, unwind: bb_y]`, e.g. _5 = <Arc<Mutex<i32>> as Deref>::deref(move _6)
    /// * switch `_i = discriminant(_j); switchInt(move _i) -> [blocks]`
    /// * field access `_i.x`
    pub fn new(line: &String) -> Option<Self> {
        for capture in MIR::get_captures() {
            if let Some(mir) = capture(line) {
                return Some(mir);
            }
        }

        None
    }

    fn get_captures() -> Vec<impl Fn(&String) -> Option<MIR>> {
        vec![MIR::assignment_capture, MIR::move_capture]
    }

    fn assignment_capture(line: &String) -> Option<MIR> {
        let assignment_pattern = Regex::new(r"(_(\d+) = (_(\d+)|(const) (.*));.*)").unwrap();
        if let Some(captures) = assignment_pattern.captures(line.as_str()) {
            // Extract the first and second variables or constant
            let first_var = captures.get(2).map(|m| m.as_str()).unwrap();
            let second_var_or_const = captures.get(3).map(|m| m.as_str()).unwrap();

            if second_var_or_const.starts_with("const") {
                Some(Self::ASSIGN(Assign {
                    left: Value::VAR(Var {
                        id: first_var.parse().unwrap(),
                    }),
                    right: Value::CONST(Const {
                        val: captures
                            .get(6)
                            .map(|m| m.as_str())
                            .unwrap()
                            .parse()
                            .unwrap(),
                    }),
                }))
            } else {
                Some(Self::ASSIGN(Assign {
                    left: Value::VAR(Var {
                        id: first_var.parse().unwrap(),
                    }),
                    right: Value::VAR(Var {
                        id: captures
                            .get(4)
                            .map(|m| m.as_str())
                            .unwrap()
                            .parse()
                            .unwrap(),
                    }),
                }))
            }
        } else {
            None
        }
    }

    fn move_capture(line: &String) -> Option<MIR> {
        let move_pattern = Regex::new(r"_(\d+) = move _(\d+)").unwrap();
        if let Some(captures) = move_pattern.captures(line.as_str()) {
            let first_var = captures.get(1).map(|m| m.as_str()).unwrap();
            let second_var_or_const = captures.get(2).map(|m| m.as_str()).unwrap();

            Some(Self::MOVE(Move {
                left: Value::VAR(Var {
                    id: first_var.parse().unwrap(),
                }),
                right: Value::VAR(Var {
                    id: second_var_or_const.parse().unwrap(),
                }),
            }))
        } else {
            None
        }
    }
}
