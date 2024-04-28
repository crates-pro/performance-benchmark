use super::{
    mir::MIRs, operand::Operand, rvalue::Rvalue, statement::Statement, terminator::Terminator,
    ty::Ty,
};

fn contains_outlet_keywords(input: &str) -> bool {
    // 定义与 I/O 操作相关的关键字数组
    let outlet_keywords = [
        // 网络 I/O 操作
        "UdpSocket",
        "TcpStream",
        "TcpListener",
        // 标准输入/输出操作
        "stdin",
        "stdout",
        "stderr",
        // 读写操作
        "read",
        "write",
        "BufRead",
        "seek",
        "copy",
        "flush",
        "read_to_string",
        "read_to_end",
        // 文件操作
        "open",
        "create",
        //"std::fs::read",
        //"std::fs::write",

        //随机数
        "rand",
        // 线程调用
        "spawn",
        //异步函数
        "async",
        //锁
        "Mutex",
        "RwLock",
        "tokio",
        "crossbeam",
        "Condvar",
        "mpsc",
        "Atomic",
        "Arc",
        "sync",
    ];
    for keyword in &outlet_keywords {
        if input.contains(keyword) {
            return true;
        }
    }
    false
}

pub fn count_pure_function(mir_file: MIRs) -> i32 {
    let mut func_all = 0;
    let mut flag;
    let mut func_part = 0;
    let functions = mir_file.functions;
    for function in functions {
        func_all += 1;
        flag = 0;
        let params = function.params;
        for param in params {
            if let Ty::Mut(_) = param.ty {
                flag = 1;
                func_part += 1;
                break;
            } else {
                flag = 0;
            }
        }
        if flag == 1 {
            continue;
        }
        let bbs = function.bbs;
        for basic_block in bbs {
            if flag == 1 {
                break;
            }
            let terminator = basic_block.terminator;
            match terminator {
                Some(terminator) => match terminator {
                    Terminator::Call(call_data) => {
                        let callee = call_data.callee;
                        for moduled_name in callee {
                            if contains_outlet_keywords(&moduled_name) {
                                flag = 1;
                                func_part += 1;
                                break;
                            }
                        }
                    }
                    _ => {}
                },
                None => {}
            }
            if flag == 1 {
                break;
            }
            let statements = basic_block.statements;
            for statement in statements {
                match statement {
                    Statement::Assign(assign) => {
                        let rvalue = assign.rvalue;
                        match rvalue {
                            Rvalue::Use(operand) => match operand {
                                Operand::CONST(const_val) => {
                                    if let Ty::AllocTy(_, _) = const_val.ty {
                                        flag = 1;
                                        func_part += 1;
                                        break;
                                    } else {
                                        flag = 0;
                                    }
                                }

                                _ => {}
                            },
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    println!("{:?}", func_all - func_part);
    func_all - func_part
}

pub fn count_closure(mir_file: MIRs) -> i32 {
    let mut closure_count = 0;
    let mut closure_use_count = 0;
    let functions = mir_file.functions;
    for function in functions {
        let label = function.label;
        for sub_label in label {
            if sub_label.contains("closure") {
                closure_count += 1;
                break;
            } else {
            }
        }
        let bbs = function.bbs;
        for basic_block in bbs {
            let terminator = basic_block.terminator;
            match terminator {
                Some(terminator) => match terminator {
                    Terminator::Call(call_data) => {
                        let callee = call_data.callee;
                        for moduled_name in callee {
                            if moduled_name.contains("closure") {
                                closure_use_count += 1;
                                break;
                            }
                        }
                    }
                    _ => {}
                },
                None => {}
            }
        }
    }
    println!("{:?}", closure_count);
    println!("{:?}", closure_use_count);
    closure_count
}

pub fn higher_function(mir_file: MIRs) -> i32 {
    let mut high_count = 0;
    let mut _high_use = 0;
    let mut flag;
    let mut use_flag = 0;
    let mut need_check: Vec<String> = Vec::new();
    let mut func_param: Vec<String> = Vec::new();
    let functions = mir_file.functions;
    for function in functions {
        flag = 0;
        let params = function.params;
        for param in params {
            let param_ty = param.ty;
            let ids = param.local_id;
            match param_ty {
                Ty::ClosureDefault | Ty::Trait | Ty::Closure(_) | Ty::CoroutineClosure(_) => {
                    func_param.push(ids.to_string());
                    if flag == 0 {
                        high_count += 1;
                    }
                    flag = 1;
                }
                Ty::SelfDef(vec) => {
                    if vec.len() == 1 {
                        need_check.push(vec[0].clone());
                    }
                }
                _ => {
                    // 其他情况的处理逻辑（如果需要）
                }
            }
        }
        let recv = function.ret_ty;
        match recv {
            Ty::ClosureDefault | Ty::Trait | Ty::Closure(_) | Ty::CoroutineClosure(_) => {
                if flag == 0 {
                    high_count += 1;
                }
                flag = 1;
            }
            Ty::SelfDef(vec) => {
                if vec.len() == 1 {
                    need_check.push(vec[0].clone());
                }
                // 可以添加特定于 Ty::SelfDef(_) 的操作
            }
            _ => {
                // 其他情况的处理逻辑（如果需要）
            }
        }
        let bbs = function.bbs;
        for basic_block in bbs {
            let terminator = basic_block.terminator;
            match terminator {
                Some(terminator) => match terminator {
                    Terminator::Call(call_data) => {
                        let callee = call_data.callee;
                        for call_name in callee {
                            if contains_with_prefix_or_suffix(&call_name, &need_check) {
                                if flag == 0 {
                                    high_count += 1;
                                }
                                flag = 1;
                                if use_flag == 0 {
                                    _high_use += 1;
                                }
                                use_flag = 1;
                            }
                            if contains_any_substring(&call_name, &func_param) {
                                if use_flag == 0 {
                                    _high_use += 1;
                                }
                                use_flag = 1;
                            }
                        }
                        if use_flag == 0 {
                            let call_params = call_data.params;
                            for call_param in call_params {
                                match call_param {
                                    Operand::CONST(const_val) => {
                                        if let Ty::ClosureDefault = const_val.ty {
                                            if use_flag == 0 {
                                                _high_use += 1;
                                            }
                                            use_flag = 1;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    _ => {}
                },
                None => {}
            }
            use_flag = 0;
        }
    }
    high_count
}

fn contains_with_prefix_or_suffix(input: &str, another_array: &[String]) -> bool {
    for element in another_array {
        // 添加前缀 impl
        let with_prefix = format!("impl{}", element);

        // 添加后缀 as
        let with_suffix = format!("{} as", element);

        // 检查 input 是否包含前缀或后缀形式的字符串
        if input.contains(&with_prefix) || input.contains(&with_suffix) {
            return true;
        }
    }

    // 如果没有匹配项，返回 false
    false
}

fn contains_any_substring(text: &str, substrings: &[String]) -> bool {
    // 遍历 `substrings` 中的每一个子字符串
    for substring in substrings {
        // 使用 `text.contains()` 判断 `text` 是否包含 `substring`
        if text.contains(substring) {
            // 如果 `text` 包含 `substring`，返回 `true`
            return true;
        }
    }
    // 如果 `text` 没有包含 `substrings` 中的任何一个元素，返回 `false`
    false
}
