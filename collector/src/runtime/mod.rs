use std::{
    fs::{create_dir_all, read_dir},
    mem::ManuallyDrop,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context};

use crate::{
    benchmark::benchmark::Benchamrk,
    benchmark::benchmark::BenchmarkSuit,
    statistics::runtime_stat::RuntimeResultVec,
    toolchain::{Compiler, LocalToolchain, PerfTool},
};

use crate::statistics::runtime_stat::RuntimeResult;

pub mod cargo_bench_process;
pub mod cargo_example_process;
pub mod cargo_test_process;
pub mod measure;

pub trait Runtime {
    fn measure(
        &self,
        perf_tool: &PerfTool,
        event_filter_file: &PathBuf,
    ) -> anyhow::Result<Option<RuntimeResult>>;
    fn draw_flame_graph(&self, out_path: &Path) -> anyhow::Result<()>;
}

pub fn bench_runtime(
    ltc: &LocalToolchain,
    benchmark_dir: PathBuf,
    iterations: u32,
    perf_tool: &PerfTool,
    event_filter_file: &PathBuf,
    flame_graph_out_path: &Path,
    flamegraph_flag: i32,
    out_dir: &PathBuf,
) -> anyhow::Result<RuntimeResultVec> {
    let benchmark_suit = BenchmarkSuit {
        benchmarks: discover_benchmark_suit(&benchmark_dir)?,
    };
    println!("{}", benchmark_suit.display_benchmarks());

    let mut results = RuntimeResultVec(vec![]);

    for benchmark in benchmark_suit.benchmarks {
        let timing_dir = ManuallyDrop::new(benchmark.make_temp_dir(&benchmark.path)?);
        let cwd = timing_dir.path();

        let process =
            benchmark.make_runtime_process(Compiler::from_toolchain(ltc), cwd, iterations);

        match &process {
            Ok(process) => {
                let result = process.measure(perf_tool, event_filter_file);

                match result {
                    Ok(result) => {
                        match perf_tool.get_bencher() {
                            crate::toolchain::Bencher::PerfStat => {
                                if result.is_some() {
                                    results.0.push(result.unwrap())
                                }
                            }
                            crate::toolchain::Bencher::PerfRecord => {
                                // 将PerfRecord产生的数据文件拷贝至目标文件夹
                                let cwd = timing_dir.path();
                                let dst_dir = out_dir.join(benchmark.name.clone());

                                let _ = create_dir_all(dst_dir.clone());

                                // Find file whoose name contains "perf.analyze" in directory cwd,
                                // and copy it into directory dst_dir.
                                for entry in read_dir(cwd)? {
                                    let entry = entry?;
                                    if entry.file_name().to_str().unwrap().contains("perf.data") {
                                        let src_path = PathBuf::from(entry.file_name());
                                        let dst_path =
                                            dst_dir.join(src_path.as_os_str().to_str().unwrap());
                                        if let Err(err) =
                                            std::fs::copy(cwd.join(src_path), &dst_path)
                                        {
                                            eprintln!("Failed to copy 'perf.data' file: {}", err);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(_) => {
                        eprintln!("Fail to bench {}. Skip.", benchmark.name);
                        continue;
                    }
                }
                //draw flamegraph
                if flamegraph_flag > 0 {
                    process.draw_flame_graph(flame_graph_out_path)?;
                }
            }
            Err(msg) => {
                eprintln!("{}", msg);
                eprintln!(
                    "Benchmark {} skipped due to previous error.",
                    benchmark.name
                );
            }
        }

        // process need to be dropped before we drop timing_dir
        // but why?
        drop(process);
        drop(ManuallyDrop::into_inner(timing_dir));
    }

    Ok(results)
}

fn discover_benchmark_suit(dir: &PathBuf) -> anyhow::Result<Vec<Benchamrk>> {
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

    Ok(benchmarks)
}

lazy_static::lazy_static! {
    static ref FAKE_RUNTIME: PathBuf = {
        let mut fake_runtime = std::env::current_exe().unwrap();
        fake_runtime.pop();
        fake_runtime.push("runtime-fake");
        fake_runtime
    };

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

#[test]
fn test_runtime() {
    use std::io::BufRead;
    use std::process::Command;

    let mut test_cmd = Command::new(
        "/home/fxl191220029/study/Rust_Performance_Benchmark/collector/target/release/runtime-fake",
    );
    test_cmd.env("RUNTIME_ELF"
    , "/home/fxl191220029/study/Rust_Performance_Benchmark/benchmarks/runtime/target/release/helloworld");
    test_cmd.arg("-t").arg("--help").arg("testarg");
    let output = test_cmd.output();
    match output {
        core::result::Result::Ok(_) => {
            let stdout = output.unwrap();
            let stdout = stdout.stdout.lines();
            for line in stdout {
                println!("{}", line.unwrap());
            }
        }
        Err(_) => println!("{:?}", output),
    }
}
