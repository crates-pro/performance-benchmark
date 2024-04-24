use std::{
    fs::File,
    io::{BufReader, Read},
};
use std::collections::HashMap;

use libc::PIPE_BUF;

use crate::mir_analyze::mir::statement;

use super::{basic_block, function, mir::{MIRs, ModuledIdentifier}, operand::Operand, rvalue::Rvalue, scope::Scope, statement::Statement, terminator::Terminator, ty::Ty, terminator::*};

//LOF
pub fn lof(mir_file: MIRs) {
    let mut lof = 0;
    let mut flag = 0;
    let functions = mir_file.promoted_functions;
    for function in functions {
        let bbs = function.body.bbs;
        for basic_block in bbs {
            let statements = basic_block.statements;
            for statement in statements {
                lof += 1;
            }
            let terminator = basic_block.terminator;
            match terminator {
                Some(terminator) => {
                    lof += 1;
                }
                None => {
                }
            }
        }
    }
    let functions = mir_file.functions;
    for function in functions{
        flag = 0;
        let label = function.label;
        for sub_label in label{
            if sub_label.contains("closure") {
                flag = 1;
                break;
            } else {
                
            } 
        }
        if flag ==0 {continue;}
        let bbs = function.bbs;
        for basic_block in bbs {
            let statements = basic_block.statements;
            for statement in statements {
                lof += 1;
            }
            let terminator = basic_block.terminator;
            match terminator {
                Some(terminator) => {
                    lof += 1;
                }
                None => {
                }
            }
        }
    }
    println!("{:?}", lof);
}
//DFC
//PBF
pub fn pbf(mir_file: MIRs) {
    let mut pbf = 1;
    let mut func = 0;
    let functions = mir_file.promoted_functions;
    for function in functions {
        func += 1;
        let bbs = function.body.bbs;
        for basic_block in bbs {
            let terminator = basic_block.terminator;
            match terminator {
                Some(terminator) => {
                    match terminator {
                        Terminator::Assert(assert) => {
                            match assert.unwind{
                                Some(unwind) => {
                                    match unwind{
                                        UnwindAction::CleanUp(_)| UnwindAction::Continue => {
                                            pbf += 1;
                                        }
                                        _ => {
        
                                        }
                                    }
                                }
                                _ => {

                                }
                            }
                        }
                        Terminator::Call(call) => {
                            match call.success{
                                None => {

                                }
                                _ => {
                                    match call.unwind{
                                        Some(unwind) => {
                                            match unwind{
                                                UnwindAction::CleanUp(_)| UnwindAction::Continue => {
                                                    pbf += 1;
                                                }
                                                _ => {
                
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                        Terminator::SwitchInt(switch) => {
                            let count = switch.success.len();
                            pbf += count -1; 
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
    let functions = mir_file.functions;
    for function in functions {
        func += 1;
        let bbs = function.bbs;
        for basic_block in bbs {
            let terminator = basic_block.terminator;
            match terminator {
                Some(terminator) => {
                    match terminator {
                        Terminator::Assert(assert) => {
                            match assert.unwind{
                                Some(unwind) => {
                                    match unwind{
                                        UnwindAction::CleanUp(_)| UnwindAction::Continue => {
                                            pbf += 1;
                                        }
                                        _ => {
        
                                        }
                                    }
                                }
                                _ => {

                                }
                            }
                        }
                        Terminator::Call(call) => {
                            match call.success{
                                None => {

                                }
                                _ => {
                                    match call.unwind{
                                        Some(unwind) => {
                                            match unwind{
                                                UnwindAction::CleanUp(_)| UnwindAction::Continue => {
                                                    pbf += 1;
                                                }
                                                _ => {
                
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                        Terminator::SwitchInt(switch) => {
                            let count = switch.success.len();
                            pbf += count -1; 
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
    println!("{:?}", pbf/func);
}
//WMS
pub fn wms_noc_rfs(mir_file: MIRs) {
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
            let name = params[0].ty.to_string()[1..].to_string();
            let count = struct_method.entry(name.clone()).or_insert(0);
            *count += 1;

            rfs.insert(name.clone() + "::" + &method_name, 0);
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
        println!("{}: {}", name, score);
    }
    println!("{}", struct_method.len());

    for (name, score) in &rfs {
        println!("{}: {}", name, score);
    }
}


//RFS
//same as vms: rfs
//NOC
//same as vms: struct_method.len()

