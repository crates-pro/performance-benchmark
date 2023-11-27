use std::{fs::read_dir, path::PathBuf, vec};

use anyhow::{bail, Context};

use crate::{
    benchmark::profile::Profiles,
    csv_transfer::{
        compile_time::create_compile_time_csv,
        runtime::{create_runtime_csv, read_runtime_json},
    },
};

use self::{compare::compare::do_compare, compile_time::read_compile_time_json};

mod compile_time;
mod runtime;

pub mod compare;

pub(self) fn get_sub_dir(in_dir: &PathBuf) -> Vec<PathBuf> {
    let mut v = vec![];
    let entries = match read_dir(in_dir) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Fail to open directory {}\n{}", in_dir.display(), e);
            panic!();
        }
    };
    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Fail to read directory {}\n{}", in_dir.display(), e);
                panic!();
            }
        };

        let path = entry.path();
        if match entry.file_type() {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Fail to read file {}\n{}", path.display(), e);
                panic!();
            }
        }
        .is_dir()
        {
            v.push(path)
        }
    }
    v
}

pub fn sheduler(in_dir: &PathBuf, ty: &String, profiles: &Profiles) {
    get_sub_dir(in_dir).iter().for_each(|in_dir| {
        get_sub_dir(in_dir)
            .iter()
            .for_each(|in_dir| match ty.as_str() {
                "compile_time" => generate_compile_time_csv(&in_dir, &in_dir, profiles),
                "runtime" => generate_runtime_csv(&in_dir, &in_dir),
                _ => panic!(),
            });

        do_compare(in_dir.clone()).unwrap();
    })
}

fn generate_compile_time_csv(in_dir: &PathBuf, out_dir: &PathBuf, profiles: &Profiles) {
    let f = match read_input_dir(in_dir) {
        Ok(f) => f,
        Err(e) => {
            eprintln!(
                "output json data not found in target directory '{}'.\n{}",
                in_dir.display(),
                e
            );
            return;
        }
    };

    let rustc_id = if in_dir.to_str().unwrap().split('/').last().unwrap().len() != 0 {
        in_dir.to_str().unwrap().split('/').last().unwrap()
    } else {
        in_dir.to_str().unwrap()[0..in_dir.to_str().unwrap().len() - 1]
            .split('/')
            .last()
            .unwrap()
    };

    match create_compile_time_csv(
        out_dir,
        rustc_id,
        &match read_compile_time_json(&f) {
            Ok(data) => data,
            Err(_) => {
                eprintln!("Fail to parse {}", f.display());
                return;
            }
        },
        profiles,
    ) {
        Ok(_) => eprintln!("Create csv for {}", rustc_id), //(),
        Err(e) => eprintln!("Fail to create csv for {}\n{}", rustc_id, e),
    };
}

fn generate_runtime_csv(in_dir: &PathBuf, out_dir: &PathBuf) {
    let f = match read_input_dir(in_dir) {
        Ok(f) => f,
        Err(e) => {
            eprintln!(
                "output json data not found in target directory '{}'.\n{}",
                in_dir.display(),
                e
            );
            return;
        }
    };

    let rustc_id = in_dir.to_str().unwrap().split('/').last().unwrap();

    match create_runtime_csv(
        out_dir,
        rustc_id,
        &match read_runtime_json(&f) {
            Ok(data) => data,
            Err(_) => {
                eprintln!("Fail to parse {}", f.display());
                return;
            }
        },
    ) {
        Ok(_) => (),
        Err(e) => eprintln!("Fail to create csv for {}\n{}", rustc_id, e),
    };
}

pub fn read_input_dir(path: &PathBuf) -> anyhow::Result<PathBuf> {
    for entry in read_dir(path)
        .with_context(|| format!("failed to read input directory '{}'", path.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        let name = match entry.file_name().into_string() {
            core::result::Result::Ok(s) => s,
            Err(e) => bail!("non-utf8 file name: {:?}", e),
        };

        if entry.file_type()?.is_file() {
            if name.contains("results.json") {
                return Ok(path);
            }
        }
    }
    bail!(format!(
        "result json file not found in '{}'",
        path.display()
    ))
}
