use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

use crate::execute::Stats;

use crate::{
    benchmark::{benchmark::Benchamrk, profile::Profile, scenario::Scenario},
    statistics::compile_time_stat::CompileTimeResult,
};

use super::dependancy::read_dependencies;

pub fn analyze_benchmark(
    benchmark: &Benchamrk,
    dependency_dir: &PathBuf,
    ops: &Vec<Box<dyn Fn(&String) -> (String, f64)>>,
) -> CompileTimeResult {
    eprintln!(
        "analyzing benchmark {} {}",
        benchmark.name,
        benchmark.path.to_str().unwrap()
    );
    let stats = analyze_dir(&benchmark.path, dependency_dir, ops, &mut HashSet::new()).unwrap();
    CompileTimeResult::new(
        benchmark.name.clone(),
        0,
        Profile::Check,
        Scenario::Full,
        stats,
    )
}

fn analyze_dir(
    p: &PathBuf,
    dependency_dir: &PathBuf,
    ops: &Vec<Box<dyn Fn(&String) -> (String, f64)>>,
    analyzed_dependency: &mut HashSet<PathBuf>,
) -> anyhow::Result<Stats> {
    let mut stats = Stats::new();
    for entry in p.read_dir()? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            stats += analyze_dir(&entry.path(), dependency_dir, ops, analyzed_dependency)?;
        } else if entry.file_type()?.is_file() {
            if entry.file_name().to_str().unwrap().ends_with(".rs") {
                let mut reader = BufReader::new(File::open(entry.path())?);
                let mut buf = vec![];
                reader.read_to_end(&mut buf)?;
                if let Ok(buf) = String::from_utf8(buf) {
                    ops.iter().for_each(|op| {
                        let t = op(&buf);
                        stats.add_or_insert(t.0, t.1)
                    });
                }
            } else if entry.file_name().to_str().unwrap().eq("Cargo.lock") {
                for d in read_dependencies(&entry.path())? {
                    let path = &d.path(dependency_dir);
                    if path.exists() && !analyzed_dependency.contains(path) {
                        println!("  |---analyzing {}", d);
                        analyzed_dependency.insert(path.clone());
                        stats += analyze_dir(path, dependency_dir, ops, analyzed_dependency)?;
                    }
                }
            }
        }
    }
    Ok(stats)
}
