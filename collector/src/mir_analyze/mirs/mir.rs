use regex::Regex;

#[derive(Debug)]
pub enum MIR {
    ASSIGN(Assign),
    REF(Reference),
    MOVE(Move),
    CALL(Call),
    FIELDACCESS(FIELDACCESS),
    MEMORY(Memory),
}

pub static ASSIGN_TYPE: &str = "assignment";
pub static REF_TYPE: &str = "reference";
pub static MOVE_TYPE: &str = "move";
pub static CALL_TYPE: &str = "call";
pub static FIELD_ACCESS_TYPE: &str = "field acess";
pub static MEMORY_TYPE: &str = "memory management";

impl MIR {
    pub fn get_all_types() -> Vec<String> {
        vec![
            ASSIGN_TYPE.to_string(),
            REF_TYPE.to_string(),
            MOVE_TYPE.to_string(),
            CALL_TYPE.to_string(),
            FIELD_ACCESS_TYPE.to_string(),
            MEMORY_TYPE.to_string(),
        ]
    }

    pub fn get_type(&self) -> String {
        match self {
            MIR::ASSIGN(_) => ASSIGN_TYPE.to_string(),
            MIR::REF(_) => REF_TYPE.to_string(),
            MIR::MOVE(_) => MOVE_TYPE.to_string(),
            MIR::CALL(_) => CALL_TYPE.to_string(),
            MIR::FIELDACCESS(_) => FIELD_ACCESS_TYPE.to_string(),
            MIR::MEMORY(_) => MEMORY_TYPE.to_string(),
        }
    }
}

pub type VarId = u32;

#[derive(Debug)]
pub struct Assign {
    pub left: Value,
    pub right: Value,
}

#[derive(Debug)]
pub enum FIELDACCESS {
    MOVE(FieldAccess),
    REF(FieldAccess),
    REFMUT(FieldAccess),
}

#[derive(Debug)]
pub struct FieldAccess {
    pub left: Var,
    pub struct_var: Var,
    pub field_num: u32,
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
    FUNCPTR(String),
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

#[derive(Debug)]
pub enum Memory {
    StorageLive(Var),
    SotrageDead(Var),
}

impl MIR {
    /// Match a mir from a string.
    /// * type bounding `let _i = some_type`
    /// * assignment `_i = _j` or `_i = cosnt xxx`
    /// * move `_i = move _j`
    /// * ref `_i = &_j` or `_i = &mut _j`
    /// * type-cast `_i = _j as xxx`
    /// * function/method call `_i = domain/type_ascription::func(args) -> [return: bb_x, unwind: bb_y] | return bb_x`
    /// * switch `_i = discriminant(_j); switchInt(move _i) -> [blocks]`
    /// * field access `_i = move (_j.x type) | &mut (_j.x type)| &(_j.x type)`
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
            MIR::field_access_capture,
            MIR::storage_capture,
        ]
    }

    /// `_i = _j` or `_i = cosnt xxx`
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

    /// `_i = move _j`
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

    /// `_i = &_j` or `_i = &mut _j`
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

    /// `_i = domain/type_ascription::func(args) -> [return: bb_x, unwind: bb_y] | return bb_x`
    fn call_capture(line: &String) -> Option<MIR> {
        let call_pattern = Regex::new(
            r"_(\d+) = (.*)\((.*)\) -> ((\[return(.*), unwind(.*)\];)|(([a-zA-Z]+[^;]);))",
        )
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
                                Param::FUNCPTR(param.to_string())
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

    /// `_i = move (_j.x type) | &mut (_j.x type)| &(_j.x type)`
    fn field_access_capture(line: &String) -> Option<MIR> {
        let field_access_pattern =
            Regex::new(r"_(\d+) = ((move )|(&mut )|(&))\(_(\d+)\.(\d+): .*\);.*").unwrap();

        if let Some(captures) = field_access_pattern.captures(line) {
            let field_access = FieldAccess {
                left: Var {
                    id: captures.get(1).unwrap().as_str().parse().unwrap(),
                },
                struct_var: Var {
                    id: captures.get(6).unwrap().as_str().parse().unwrap(),
                },
                field_num: captures.get(7).unwrap().as_str().parse().unwrap(),
            };

            if captures.get(3).is_some() {
                Some(MIR::FIELDACCESS(FIELDACCESS::MOVE(field_access)))
            } else if captures.get(4).is_some() {
                Some(MIR::FIELDACCESS(FIELDACCESS::REFMUT(field_access)))
            } else if captures.get(5).is_some() {
                Some(MIR::FIELDACCESS(FIELDACCESS::REF(field_access)))
            } else {
                panic!("Fail to capture {} as field access statement.", line);
            }
        } else {
            None
        }
    }

    fn storage_capture(line: &String) -> Option<MIR> {
        let storage_pattern =
            Regex::new(r"(StorageLive\(_(\d+)\))|(StorageDead\(_(\d+)\))").unwrap();
        if let Some(captures) = storage_pattern.captures(line) {
            if let Some(live) = captures.get(2).map(|m| m.as_str()) {
                Some(MIR::MEMORY(Memory::StorageLive(Var {
                    id: live.parse().unwrap(),
                })))
            } else if let Some(dead) = captures.get(4).map(|m| m.as_str()) {
                Some(MIR::MEMORY(Memory::SotrageDead(Var {
                    id: dead.parse().unwrap(),
                })))
            } else {
                None
            }
        } else {
            None
        }
    }
}
