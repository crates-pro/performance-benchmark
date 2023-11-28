use regex::Regex;

#[derive(Debug)]
pub enum MIR {
    ASSIGN(Assign),
    REF(Reference),
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
pub struct Reference {
    pub src: Value,
    pub dst: Value,
}

#[derive(Debug)]
pub struct Move {
    pub left: Value,
    pub right: Value,
}

#[derive(Debug)]
pub struct Call {
    pub ret: Var,
    pub label: String,
    pub params: Vec<Param>,
    pub next_block: String,
    pub unwind_block: Option<String>,
}

#[derive(Debug)]
pub enum Param {
    MOVE(Var),
    VAR(Var),
    COSNT(Const),
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
    /// * type-cast `_i = _j as xxx`
    /// * function/method call `_i = domain/type_ascription::func(args) -> [return: bb_x, unwind: bb_y] | return bb_x`, e.g. _5 = <Arc<Mutex<i32>> as Deref>::deref(move _6)
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
        vec![
            MIR::assignment_capture,
            MIR::move_capture,
            MIR::ref_capture,
            MIR::call_capture,
        ]
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
            let second_var = captures.get(2).map(|m| m.as_str()).unwrap();

            Some(Self::MOVE(Move {
                left: Value::VAR(Var {
                    id: first_var.parse().unwrap(),
                }),
                right: Value::VAR(Var {
                    id: second_var.parse().unwrap(),
                }),
            }))
        } else {
            None
        }
    }

    fn ref_capture(line: &String) -> Option<MIR> {
        let ref_capture = Regex::new(r"_(\d+) = ((&_(\d+))|(&mut _(\d+)))").unwrap();
        if let Some(captures) = ref_capture.captures(line.as_str()) {
            let first_var = captures.get(1).map(|m| m.as_str()).unwrap();

            let second_var = if let Some(second_var) = captures.get(6).map(|m| m.as_str()) {
                second_var
            } else {
                captures.get(4).map(|m| m.as_str()).unwrap()
            };

            Some(Self::REF(Reference {
                src: (Value::VAR(Var {
                    id: first_var.parse().unwrap(),
                })),
                dst: (Value::VAR(Var {
                    id: second_var.parse().unwrap(),
                })),
            }))
        } else {
            None
        }
    }

    fn call_capture(line: &String) -> Option<MIR> {
        let call_pattern =
            Regex::new(r"_(\d+) = (.*)\((.*)\) -> ((\[return(.*), unwind(.*)\];)|((.*[^;]);))")
                .unwrap();
        if let Some(captures) = call_pattern.captures(line.as_str()) {
            let params = captures.get(3).unwrap().as_str();

            let get_block = |s| {
                let bb_pattern = Regex::new(r"(: (.*))|( (.*))|(.*)").unwrap();
                let captures = bb_pattern.captures(s).unwrap();

                if let Some(bb) = captures.get(2).map(|m| m.as_str()) {
                    Some(bb.to_string())
                } else if let Some(bb) = captures.get(4).map(|m| m.as_str()) {
                    Some(bb.to_string())
                } else if let Some(bb) = captures.get(5).map(|m| m.as_str()) {
                    Some(bb.to_string())
                } else {
                    None
                }
            };

            Some(MIR::CALL(Call {
                ret: Var {
                    id: captures.get(1).unwrap().as_str().parse().unwrap(),
                },
                label: captures.get(2).unwrap().as_str().to_string(),
                params: if params.is_empty() {
                    vec![]
                } else {
                    params
                        .split(", ")
                        .into_iter()
                        .map(|param| {
                            let param_pattern =
                                Regex::new(r"(move _(\d+))|(_(\d+))|(const .*)").unwrap();
                            if let Some(captures) = param_pattern.captures(param) {
                                if let Some(c) = captures.get(5).map(|m| m.as_str()) {
                                    Param::COSNT(Const { val: c.to_string() })
                                } else if let Some(p) = captures.get(2).map(|m| m.as_str()) {
                                    Param::MOVE(Var {
                                        id: p.parse().unwrap(),
                                    })
                                } else {
                                    Param::VAR(Var {
                                        id: captures.get(4).unwrap().as_str().parse().unwrap(),
                                    })
                                }
                            } else {
                                panic!();
                            }
                        })
                        .collect()
                },
                next_block: if let Some(s) = captures.get(6).map(|m| m.as_str()) {
                    get_block(s).unwrap()
                } else {
                    get_block(captures.get(8).unwrap().as_str()).unwrap()
                },
                unwind_block: if let Some(s) = captures.get(7).map(|m| m.as_str()) {
                    get_block(s)
                } else {
                    None
                },
            }))
        } else {
            None
        }
    }
}
