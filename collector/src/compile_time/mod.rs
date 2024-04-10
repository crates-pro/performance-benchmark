use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Ok};

use crate::{
    benchmark::{
        benchmark::{Benchamrk, BenchmarkSuit},
        profile::Profile,
        scenario::Scenario,
    },
    execute::Stats,
    toolchain::{Compiler, LocalToolchain, PerfTool},
};

use self::result::CompileTimeBenchResult;

pub mod binary_size;
pub mod cargo_package_process;
pub mod cargo_single_process;
pub mod result;

pub fn bench_compile_time(
    ltc: &LocalToolchain,
    perf_tool: &PerfTool,
    event_filter_file: &PathBuf,
    profiles: &[Profile],
    scenarios: &[Scenario],
    benchmark_dir: PathBuf,
    iterations: usize,
    flame_graph_path: &Path,
    flamegraph_flag: i32,
    out_dir: &Path,
) -> anyhow::Result<Vec<CompileTimeBenchResult>> {
    let benchmark_suit = BenchmarkSuit {
        benchmarks: discover_benchmark_suit(&benchmark_dir)?,
    };
    println!("{}", benchmark_suit.display_benchmarks());

    let result = bench(
        perf_tool,
        event_filter_file,
        &profiles,
        &scenarios,
        Compiler::from_toolchain(ltc),
        &benchmark_suit.benchmarks,
        Some(iterations),
        flame_graph_path,
        flamegraph_flag,
        out_dir,
    )?;
    Ok(result)
}

pub fn bench<'a>(
    perf_tool: &PerfTool,
    event_filter_file: &PathBuf,
    profiles: &[Profile],
    scenarios: &[Scenario],
    compiler: Compiler<'_>,
    benchmarks: &[Benchamrk],
    iterations: Option<usize>,
    flame_graph_path: &Path,
    flamegraph_flag: i32,
    out_dir: &Path,
) -> anyhow::Result<Vec<CompileTimeBenchResult>> {
    let mut result_vec = vec![];
    let mut num_benchmark_left = benchmarks.len();
    benchmarks.iter().for_each(|b| {
        println!("{} benchmarks waiting.", num_benchmark_left);
        println!("Compile-time benchmarking for '{}'", b.name.as_str());
        num_benchmark_left -= 1;

        let result = b
            .measure_compile_time(
                perf_tool,
                event_filter_file,
                profiles,
                scenarios,
                compiler,
                iterations,
                flame_graph_path,
                flamegraph_flag,
                out_dir,
            )
            .with_context(|| format!("Faile to bench '{}'!", b.name));
        match result {
            core::result::Result::Ok(r) => result_vec.push(r),
            Err(s) => {
                eprintln!("{}", s);
                eprintln!("Please check your perf tool, rust compiler or the benchmark program.\n");
            }
        }
    });
    Ok(result_vec)
}

pub trait CompileTimeProcessor {
    fn run_rustc(
        &mut self,
        perf_tool: &PerfTool,
        event_filter_file: &PathBuf,
        needs_final: bool,
    ) -> anyhow::Result<Option<Stats>>;
    fn draw_flame_graph(&self, dir: &Path) -> anyhow::Result<()>;
    fn gen_pkg(&self) -> anyhow::Result<()>;
    fn increment(&mut self, incr: bool);
}

lazy_static::lazy_static! {
    static ref FAKE_RUSTC: PathBuf = {
        let mut fake_rustc = std::env::current_exe().unwrap();
        fake_rustc.pop();
        fake_rustc.push("rustc-fake");
        fake_rustc
    };

    static ref FAKE_FLAMEGRAPH: PathBuf = {
        let mut fake_runtime = std::env::current_exe().unwrap();
        fake_runtime.pop();
        fake_runtime.push("flamegraph-fake");
        fake_runtime
    };
}

pub fn discover_benchmark_suit(dir: &Path) -> anyhow::Result<Vec<Benchamrk>> {
    let mut benchmarks = vec![];

    for entry in read_dir(dir)
        .with_context(|| format!("failed to list benchmark dir '{}'", dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        let name = match entry.file_name().into_string() {
            core::result::Result::Ok(s) => s,
            Err(e) => bail!("non-utf8 benchmark name: {:?}", e),
        };
        if !entry.file_type()?.is_dir() {
            println!("benchmark '{}' ignored", name);
            continue;
        }
        benchmarks.push(Benchamrk::new(name, path)?);
    }
    if benchmarks.is_empty() {
        eprintln!("Error: no benchmark found in '{}'", dir.display());
    }
    Ok(benchmarks)
}
