use super::{mir::MIRs, terminator::Terminator,scope::Scope};

fn contains_parallelism_keywords(input: &str) -> bool {
    // 定义与 I/O 操作相关的关键字数组
    let parallelism_keywords = [
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
    for keyword in &parallelism_keywords {
        if input.contains(keyword) {
            return true;
        }
    }
    false
}

pub fn count_parallelism_metrics(mir_file: MIRs) -> i32{
    let mut parallelism_count = 0;
    let functions = mir_file.functions;
    for function in functions {
        let bbs = function.bbs;
        for basic_block in bbs {
            let terminator = basic_block.terminator;
            match terminator {
                Some(terminator) => {
                    match terminator {
                        Terminator::Call(call_data) => {
                            let callee = call_data.callee;
                            for moduled_name in callee {
                                if contains_parallelism_keywords(&moduled_name) {
                                    parallelism_count += 1;
                                    break;  
                                }                                 
                            }
                        }
                        _ => {
                        }
                    }
                }
                None => {
                }
            }
        }
    }
    println!("{:?}", parallelism_count);
    parallelism_count
}

pub fn count_parallelism_strcut(mir_file: MIRs) -> i32{
    let mut parallelism_struct_count = 0;
    let functions = mir_file.functions;
    for function in functions {
        let local_defs = function.local_defs;
        for local_def in local_defs{
            let ty = local_def.ty;
            if contains_parallelism_keywords(&ty.to_string()){
                parallelism_struct_count += 1;
            }
        }
        let scopes = function.scopes;
        parallelism_struct_count = count_scopes(scopes, parallelism_struct_count);
    }
    println!("{:?}", parallelism_struct_count);
    parallelism_struct_count
    
}

pub fn count_scopes(scopes: Vec<Scope>, mut parallelism_struct_count:i32) -> i32 {
    for scope in scopes {
        let local_defs = scope.local_defs;
        for local_def in local_defs{
            let ty = local_def.ty;
            if contains_parallelism_keywords(&ty.to_string()){
                parallelism_struct_count += 1;
            }
        }
        let sub_scopes = scope.sub_scopes;
        parallelism_struct_count = count_scopes(sub_scopes, parallelism_struct_count);
    }
    parallelism_struct_count
}