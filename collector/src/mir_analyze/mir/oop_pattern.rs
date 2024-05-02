use std::collections::{HashMap, HashSet};
use std::fmt;

use super::{mir::MIRs, terminator::Terminator, terminator::*};

//LOF
pub fn lof(mir_file: &MIRs) -> i32 {
    let mut lof = 0;
    let mut flag;
    let functions = &mir_file.promoted_functions;
    for function in functions {
        let bbs = &function.body.bbs;
        for basic_block in bbs {
            let statements = &basic_block.statements;
            for _statement in statements {
                lof += 1;
            }
            let terminator = &basic_block.terminator;
            match terminator {
                Some(_terminator) => {
                    lof += 1;
                }
                None => {}
            }
        }
    }
    let functions = &mir_file.functions;
    for function in functions {
        flag = 0;
        let label = &function.label;
        for sub_label in label {
            if sub_label.contains("closure") {
                flag = 1;
                break;
            } else {
            }
        }
        if flag == 0 {
            continue;
        }
        let bbs = &function.bbs;
        for basic_block in bbs {
            let statements = &basic_block.statements;
            for _statement in statements {
                lof += 1;
            }
            let terminator = &basic_block.terminator;
            match terminator {
                Some(_terminator) => {
                    lof += 1;
                }
                None => {}
            }
        }
    }
    // println!("{:?}", lof);
    lof
}
//DFC
pub fn count_function_call(mir_file: &MIRs) -> HashMap<Vec<String>, Vec<Vec<String>>> {
    let functions = &mir_file.functions;
    let mut funcname_call = HashMap::new();
    for function in functions {
        let labels = &function.label;
        let params = &function.params;
        let param_ty = if let Some(param) = params.get(0) {
            param.ty.to_string()
        } else {
            "Unknown".to_string()
        };

        let mut result = Vec::new();
        let mut found_impl = false;
        for label in labels.clone() {
            if label.contains("<impl at") {
                if found_impl {
                    result.push(label.clone());
                } else {
                    if param_ty != "Unknown" {
                        result.push(param_ty.clone());
                    }
                    found_impl = true;
                }
            } else if found_impl {
                result.push(label.clone());
            }
        }
        if result.len() == 0 {
            for label in labels.clone() {
                result.push(label.clone());
            }
        }
        let mut call = Vec::new(); // 正确的定义应该是 HashMap<Vec<String>, Vec<Vec<String>>>
        let bbs = &function.bbs;
        for basic_block in bbs {
            let terminator = &basic_block.terminator;
            match terminator {
                Some(terminator) => {
                    match terminator {
                        Terminator::Call(call_data) => {
                            let callee = &call_data.callee;
                            call.push(callee.clone()); // 这里应该将 callee 插入到 call[key] 中
                        }
                        _ => {}
                    }
                }
                None => {}
            }
        }
        funcname_call.insert(result, call); // 应该是 funcname_call.insert(param_ty, call);
    }
    funcname_call
}
struct CallGraph {
    edges: HashMap<Vec<String>, HashSet<Vec<String>>>,
}
impl CallGraph {
    fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }

    fn add_edge(&mut self, caller: Vec<String>, callees: Vec<String>) {
        if let Some(existing_callees) = self.edges.get_mut(&caller) {
            existing_callees.insert(callees);
        } else {
            let mut callees_set = HashSet::new();
            callees_set.insert(callees);
            self.edges.insert(caller, callees_set);
        }
    }

    fn depth(&self) -> HashMap<Vec<String>, usize> {
        let mut depths: HashMap<Vec<String>, usize> = HashMap::new();

        for node in self
            .edges
            .keys()
            .filter(|node| node.iter().any(|s| s == "main"))
        {
            self.dfs(node.clone(), 1, &mut depths, &HashSet::new());
        }

        depths
    }

    fn dfs(
        &self,
        node: Vec<String>,
        depth: usize,
        depths: &mut HashMap<Vec<String>, usize>,
        visited: &HashSet<Vec<String>>,
    ) {
        if visited.contains(&node) {
            return;
        }

        depths.insert(node.clone(), depth);
        let visited = visited
            .iter()
            .cloned()
            .chain(std::iter::once(node.clone()))
            .collect::<HashSet<_>>();

        if let Some(neighbors) = self.edges.get(&node) {
            for neighbor in neighbors {
                self.dfs(neighbor.clone(), depth + 1, depths, &visited);
            }
        }
    }
}
impl fmt::Debug for CallGraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.edges)
    }
}

pub fn dfc(mir_file: &MIRs) -> i32 {
    let funcname_call = count_function_call(mir_file);
    let cg = build_call_graph(&funcname_call);
    // println!("{:?}", cg);
    let depths = cg.depth();
    // println!("{:?}", depths);

    let funcname_call_count = funcname_call.len();
    let depths_count = depths.len();
    let depths_sum: usize = depths.values().sum();

    let result = funcname_call_count - depths_count + depths_sum / funcname_call_count;
    // println!("Result: {}", result);
    result as i32
}

// 将切片中包含 "main" 的键放入工作列表中
fn add_main_to_wl(
    funcname_call: &HashMap<Vec<String>, Vec<Vec<String>>>,
    wl: &mut Vec<Vec<String>>,
) {
    for key in funcname_call.keys() {
        if key.iter().any(|s| s == "main") {
            wl.push(key.clone());
        }
    }
}

fn build_call_graph(funcname_call: &HashMap<Vec<String>, Vec<Vec<String>>>) -> CallGraph {
    let mut wl = Vec::new();
    add_main_to_wl(funcname_call, &mut wl);
    let mut cg = CallGraph::new();
    let mut rm = HashSet::new();

    while !wl.is_empty() {
        let mm = wl.pop().unwrap(); // 从工作列表中移除一个方法

        if !rm.contains(&mm) {
            rm.insert(mm.clone());

            // 获取方法的调用点集合
            let mut call_sites = Vec::new();
            if let Some(sites) = funcname_call.get(&mm) {
                call_sites = sites.clone();
            }
            for call_site in call_sites {
                let targets = resolve(funcname_call, &call_site);
                if targets.len() != 0 {
                    cg.add_edge(mm.clone(), targets.clone());
                    wl.push(targets.clone());
                }
            }
        }
    }
    cg // 返回 CallGraph 结果
}

fn resolve(
    function_call: &HashMap<Vec<String>, Vec<Vec<String>>>,
    call_site: &Vec<String>,
) -> Vec<String> {
    for key in function_call.keys() {
        let first_elem = &key[0];
        if first_elem.contains("closure") && call_site.iter().any(|s| s.contains(first_elem)) {
            // 如果第一个元素包含 "closure"，并且 call_site 中的任何一个元素包含 first_elem，则直接返回该键值，并构建调用边
            return key.clone();
        } else if first_elem.starts_with("&") {
            let mut key_without_prefix = first_elem[1..].to_string(); // 去掉前缀 "&"
            key_without_prefix.push_str("::");
            key_without_prefix.push_str(&key[1..].join("::")); // 使用 "::" 连接剩余部分
            let call_site_str = call_site.join("::"); // 使用 "::" 连接调用点集合中的元素
            if call_site_str.contains(&key_without_prefix) {
                return key.clone();
            }
        } else {
            // 其他情况直接返回键值
            if call_site.iter().any(|s| s.contains(first_elem)) {
                return key.clone();
            }
        }
    }
    Vec::new() // 如果未找到匹配的键值，则返回一个空的 Vec
}

//PBF
pub fn pbf(mir_file: &MIRs) -> i32 {
    let mut pbf = 1;
    let mut func = 0;
    let functions = &mir_file.promoted_functions;
    for function in functions {
        func += 1;
        let bbs = &function.body.bbs;
        for basic_block in bbs {
            let terminator = &basic_block.terminator;
            match terminator {
                Some(terminator) => match terminator {
                    Terminator::Assert(assert) => match &assert.unwind {
                        Some(unwind) => match unwind {
                            UnwindAction::CleanUp(_) | UnwindAction::Continue => {
                                pbf += 1;
                            }
                            _ => {}
                        },
                        _ => {}
                    },
                    Terminator::Call(call) => match call.success {
                        None => {}
                        _ => match &call.unwind {
                            Some(unwind) => match unwind {
                                UnwindAction::CleanUp(_) | UnwindAction::Continue => {
                                    pbf += 1;
                                }
                                _ => {}
                            },
                            _ => {}
                        },
                    },
                    Terminator::SwitchInt(switch) => {
                        let count = switch.success.len();
                        pbf += count - 1;
                    }
                    _ => {}
                },
                None => {}
            }
        }
    }
    let functions = &mir_file.functions;
    for function in functions {
        func += 1;
        let bbs = &function.bbs;
        for basic_block in bbs {
            let terminator = &basic_block.terminator;
            match terminator {
                Some(terminator) => match terminator {
                    Terminator::Assert(assert) => match &assert.unwind {
                        Some(unwind) => match unwind {
                            UnwindAction::CleanUp(_) | UnwindAction::Continue => {
                                pbf += 1;
                            }
                            _ => {}
                        },
                        _ => {}
                    },
                    Terminator::Call(call) => match call.success {
                        None => {}
                        _ => match &call.unwind {
                            Some(unwind) => match unwind {
                                UnwindAction::CleanUp(_) | UnwindAction::Continue => {
                                    pbf += 1;
                                }
                                _ => {}
                            },
                            _ => {}
                        },
                    },
                    Terminator::SwitchInt(switch) => {
                        let count = switch.success.len();
                        pbf += count - 1;
                    }
                    _ => {}
                },
                None => {}
            }
        }
    }
    let result = pbf as f64 / func as f64;
    // println!("{}", result);
    result as i32
}
//WMS
pub fn wms_noc_rfs(mir_file: &MIRs) -> Vec<i32> {
    let mut struct_method = HashMap::new();
    let mut rfs = HashMap::new();
    let mut flag;
    let mut closure_flag;

    for function in &mir_file.functions {
        flag = false;
        closure_flag = false;

        let params = &function.params;
        let labels = &function.label;
        let method_name = labels.last().expect("Empty labels").clone();

        for label in labels {
            if label.contains("<impl at") {
                flag = true;
            }
            if label.contains("closure") {
                closure_flag = true;
            }
        }

        if flag && !closure_flag {
            if params.len() != 0 {
                let name = params[0].ty.to_string()[1..].to_string();
                let count = struct_method.entry(name.clone()).or_insert(0);
                *count += 1;
                rfs.insert(name.clone() + "::" + &method_name, 0);
            }
        }
    }

    for function in &mir_file.functions {
        flag = false;
        closure_flag = false;

        let labels = &function.label;

        for label in labels {
            if label.contains("<impl at") {
                flag = true;
            }
            if label.contains("closure") {
                closure_flag = true;
            }
        }

        if flag && !closure_flag {
            let bbs = &function.bbs;

            for basic_block in bbs {
                if let Some(Terminator::Call(call_data)) = &basic_block.terminator {
                    let callee = &call_data.callee;
                    let module = callee.join("::");

                    for (key, value) in &mut rfs {
                        if module.contains(key) {
                            *value += 1;
                        }
                    }
                }
            }
        }
    }

    for (name, score) in &struct_method {
        // println!("{}: {}", name, score);
    }
    //wms 几何均值
    let wms = arithmetic_mean(&struct_method);
    // println!("{:?}", wms);
    // println!("{}", struct_method.len());

    for (name, score) in &rfs {
        // println!("{}: {}", name, score);
    }
    let rfs = arithmetic_mean(&rfs);
    // println!("{:?}", rfs);
    //rfs 几何均值

    let mut final_vec = Vec::new();
    final_vec.push(wms as i32);
    final_vec.push(rfs as i32);
    final_vec.push(struct_method.len() as i32);
    final_vec
}

//RFS
//same as vms: rfs
//NOC
//same as vms: struct_method.len()
fn arithmetic_mean(scores: &HashMap<String, i32>) -> f64 {
    let mut sum: i64 = 0;
    let mut count = 0;

    // 计算总和和计数
    for score in scores.values() {
        sum += *score as i64; // 将 score 转换为 i64 类型，并累加到 sum 中
        count += 1; // 统计迭代次数
    }

    // 计算算数平均值
    let arithmetic_mean = if count > 0 {
        sum as f64 / count as f64
    } else {
        0.0 // 防止除以零
    };

    arithmetic_mean
}
