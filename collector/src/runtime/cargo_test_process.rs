use std::{
    path::{Path, PathBuf},
    process::Command,
};

use crate::{
    execute::process_benchmark_output,
    toolchain::{Compiler, PerfTool},
    utils::command::{command_discard_output, command_output},
};

use super::{result::RuntimeResult, Runtime, FAKE_FLAMEGRAPH, FAKE_RUNTIME};

pub struct CargoTestProcess<'a> {
    pub compiler: Compiler<'a>,
    pub processor_name: String,
    pub cwd: &'a Path,
    pub manifest_path: String,
    pub iterations: u32,
    pub args: Vec<String>,
}

impl<'a> CargoTestProcess<'a> {
    fn base_command(&self) -> Command {
        let mut cmd = Command::new(&*FAKE_RUNTIME);
        cmd.env("RUNTIME_ELF", self.compiler.cargo)
            .env("RUSTC", self.compiler.rustc)
            .env("CARGO_INCREMENTAL", "0")
            .env("RUSTC_BOOTSTRAP", "1")
            .current_dir(self.cwd)
            .arg("test")
            .args(self.args.clone())
            .arg("--manifest-path")
            .arg(&self.manifest_path)
            .arg("release")
            .arg("--no-fail-fast");
        cmd
    }

    fn base_flame_graph_command(&self, out: &Path) -> Command {
        let mut cmd = Command::new(&*FAKE_FLAMEGRAPH);

        let mut flame_graph_file_name = self.processor_name.clone();
        flame_graph_file_name += "_runtime.svg";
        cmd.current_dir(self.cwd)
            .env("RUSTC", self.compiler.rustc)
            .env("CARGO_INCREMENTAL", "1")
            .env("RUSTC_BOOTSTRAP", "1")
            .arg("--output")
            .arg(out.join(flame_graph_file_name))
            .arg("--")
            .arg(self.compiler.cargo)
            .arg("test")
            .args(self.args.clone())
            .arg("--manifest-path")
            .arg(&self.manifest_path)
            .arg("release")
            .arg("--no-fail-fast");
        cmd
    }

    fn compile_test(&self) -> anyhow::Result<()> {
        let mut cmd = self.base_command();
        cmd.arg("--no-run").env("CARGO_INCREMENTAL", "0");

        eprintln!("{:?}", cmd.get_program());
        eprintln!("args: {:?}", cmd.get_args());
        eprintln!("env: {:?}", cmd.get_envs());

        command_discard_output(&mut cmd)
    }
}

impl<'a> Runtime for CargoTestProcess<'a> {
    fn measure(
        &self,
        perf_tool: &PerfTool,
        _event_filter_file: &PathBuf,
    ) -> anyhow::Result<Option<RuntimeResult>> {
        eprintln!("compiling test for {}...", self.processor_name);
        match self.compile_test() {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Fail to compile test for {}.", self.processor_name);
                eprintln!("{:?}", err);
                return Err(err);
            }
        }

        let mut result = RuntimeResult::new(self.processor_name.clone());

        for iteration in 0..self.iterations {
            eprintln!(
                "running '{}' Runtime iteration {}/{}...",
                self.processor_name.clone(),
                iteration + 1,
                self.iterations
            );

            let mut cmd = self.base_command();

            let perf_tool_name = perf_tool.name();
            cmd.arg("--wrap-rustc-with");
            cmd.arg(perf_tool_name);

            match perf_tool.get_bencher() {
                crate::toolchain::Bencher::PerfStat => (),
                crate::toolchain::Bencher::PerfRecord => {
                    cmd.arg("--output")
                        .arg(self.cwd.join(format!("{:02}_perf.data", iteration)));
                }
            }

            let output = command_output(&mut cmd)?;

            match perf_tool.get_bencher() {
                crate::toolchain::Bencher::PerfStat => match process_benchmark_output(output) {
                    Ok(output) => {
                        result.append(output);
                    }
                    Err(err) => {
                        eprintln!("Fail to test {}. Err msg:", self.processor_name);
                        eprintln!("{:?}", err);
                        return Err(err.into());
                    }
                },
                crate::toolchain::Bencher::PerfRecord => (),
            }
        }

        Ok(Some(result))
    }

    fn draw_flame_graph(&self, out_path: &Path) -> anyhow::Result<()> {
        eprintln!("compiling test for {}...", self.processor_name);
        match self.compile_test() {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Fail to compile test for {}.", self.processor_name);
                eprintln!("{:?}", err);
                return Err(err);
            }
        }
        eprintln!("drawing flamegraph for '{}'...", self.processor_name);
        let mut cmd = self.base_flame_graph_command(out_path);
        command_discard_output(&mut cmd)?;
        Ok(())
    }
}

#[test]
fn test_cargo_test_process() {
    let bench_dir = "/media/workstation/cc36671e-05f5-48bd-9b40-1b1c1f396fae/home/fxl/benchmarks_real/runtime/test/sled-0.34";
    let cargo = "/home/workstation/.rustup/toolchains/1.67-x86_64-unknown-linux-gnu/bin/cargo";
    let rustc = "/media/workstation/cc36671e-05f5-48bd-9b40-1b1c1f396fae/home/fxl/rustc/targets/Add_new_MIR_constant_propagation_based_on_dataflow_analysis/780952f922a_old/bin/rustc";

    let mut cmd = Command::new(cargo);
    cmd.env("RUSTC", rustc)
        .env("CARGO_INCREMENTAL", "0")
        .arg("test")
        .arg("--no-fail-fast")
        .arg("--no-run")
        .arg("--manifest-path")
        .arg("Cargo.toml")
        .arg("--features")
        .arg("testing")
        .arg("--profile")
        .arg("release");

    let _ = command_output(&mut cmd);

    let mut cmd = Command::new("/media/workstation/cc36671e-05f5-48bd-9b40-1b1c1f396fae/home/fxl/Rust_Performance_Benchmark/collector/target/release/runtime-fake");
    cmd.env("RUSTC", rustc)
        .env("RUNTIME_ELF", cargo)
        .env("CARGO_INCREMENTAL", "0")
        .env("RUSTC_BOOTSTRAP", "1")
        .current_dir(bench_dir)
        .arg("test")
        .arg("--features")
        .arg("testing")
        .arg("--no-fail-fast")
        .arg("--manifest-path")
        .arg("Cargo.toml")
        .arg("--profile")
        .arg("release");

    let output = command_output(&mut cmd);
    match output {
        Ok(output) => {
            eprintln!("{:?}", output);
            eprintln!("{:?}", process_benchmark_output(output));
        }
        _ => eprintln!("{:?}", output),
    }
}

#[test]
fn test_cargo_test_process_flamegraph() {
    let mut cmd = Command::new("/home/fxl191220029/study/Rust_Performance_Benchmark/collector/target/release/flamegraph-fake");
    let out = Path::new(
        "/home/fxl191220029/study/Rust_Performance_Benchmark/collector/results/flamegraphs",
    );
    let mut flame_graph_file_name = String::from("plus");
    flame_graph_file_name += "_runtime.svg";
    cmd.current_dir("/tmp/plus")
        .arg("--output")
        .arg(out.join(flame_graph_file_name))
        .arg("--")
        .arg("/home/fxl191220029/.cargo/bin/cargo")
        .arg("test");

    println!("{:?}", cmd);
    let _output = command_discard_output(&mut cmd);
}
