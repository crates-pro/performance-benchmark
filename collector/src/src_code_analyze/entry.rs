use std::fs::{create_dir, create_dir_all, File};
use std::io::BufWriter;
use std::path::PathBuf;

use crate::src_code_analyze::ops::analyze_ops;
use crate::statistics::compile_time_stat::{CompileTimeBenchResult, CompileTimeResultSet};

use crate::{compile_time::discover_benchmark_suit, src_code_analyze::analyzer::analyze_benchmark};

pub fn src_code_analyze(
    bench_dir: PathBuf,
    dependency_dir: PathBuf,
    out_path: PathBuf,
) -> anyhow::Result<PathBuf> {
    assert!(dependency_dir.exists());
    let benchmarks = discover_benchmark_suit(&bench_dir)?;

    let ops = analyze_ops();

    let mut results = vec![];

    for b in benchmarks {
        results.push(CompileTimeBenchResult {
            benchmark: b.name.clone(),
            iterations: 0,
            result_vec: vec![analyze_benchmark(&b, &dependency_dir, &ops)],
        });
    }

    let results = CompileTimeResultSet::new(0.to_string(), results);
    create_dir_all(&out_path)?;
    serde_json::to_writer(
        BufWriter::new(File::create(
            &out_path.join("src-code-analyze-results.json"),
        )?),
        &results,
    )?;

    Ok(out_path)
}
