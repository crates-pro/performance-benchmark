use std::{
    path::{Path, PathBuf},
    process::Command,
};

use crate::{
    execute::process_benchmark_output,
    statistics::runtime_stat::RuntimeResult,
    toolchain::{Compiler, PerfTool},
    utils::command::{command_discard_output, command_output},
};

use super::{Runtime, FAKE_FLAMEGRAPH, FAKE_RUNTIME};

pub struct CargoBenchProcess<'a> {
    pub compiler: Compiler<'a>,
    pub processor_name: String,
    pub cwd: &'a Path,
    pub manifest_path: String,
    pub iterations: u32,
    args: Vec<String>,
}

impl<'a> CargoBenchProcess<'a> {
    pub fn new(
        compiler: Compiler<'a>,
        processor_name: String,
        cwd: &'a Path,
        manifest_path: String,
        iterations: u32,
        args: Vec<String>,
    ) -> Self {
        let process = Self {
            compiler,
            processor_name,
            cwd,
            manifest_path,
            iterations,
            args,
        };
        match process.build() {
            Ok(_) => (),
            Err(_) => eprintln!("Fail to compile examples for {}.", process.processor_name),
        };

        process
    }

    fn base_command(&self) -> Command {
        let mut cmd = Command::new(&*FAKE_RUNTIME);
        cmd.env("RUNTIME_ELF", self.compiler.cargo)
            .env("RUSTC", self.compiler.rustc)
            // .env("CARGO_INCREMENTAL", "0")
            .env("RUSTC_BOOTSTRAP", "1")
            .current_dir(self.cwd)
            .arg("bench")
            .args(self.args.clone())
            .arg("--manifest-path")
            .arg(&self.manifest_path)
            .arg("--profile")
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
            .arg("bench")
            .args(self.args.clone())
            .arg("--manifest-path")
            .arg(&self.manifest_path)
            .arg("--profile")
            .arg("release")
            .arg("--no-fail-fast");
        cmd
    }

    fn build(&self) -> anyhow::Result<()> {
        eprintln!("Building bench for {}...", self.processor_name);
        let mut cmd = self.base_command();
        cmd.arg("--no-run").env("CARGO_INCREMENTAL", "0");

        match command_discard_output(&mut cmd) {
            Ok(_) => Ok(()),
            Err(err) => {
                eprintln!(
                    "Fail to compile bench {}, will be skipped.",
                    self.processor_name
                );
                eprintln!("{:?}", err);
                return Err(err);
            }
        }
    }
}

impl<'a> Runtime for CargoBenchProcess<'a> {
    fn measure(
        &self,
        perf_tool: &PerfTool,
        _event_filter_file: &PathBuf,
    ) -> anyhow::Result<Option<crate::statistics::runtime_stat::RuntimeResult>> {
        let mut result = RuntimeResult::new(self.processor_name.clone());

        for iteration in 0..self.iterations {
            eprintln!(
                "running '{}' Runtime Bench iteration {}/{}",
                self.processor_name.clone(),
                iteration + 1,
                self.iterations
            );
            let mut cmd = self.base_command();

            match perf_tool.get_bencher() {
                crate::toolchain::Bencher::PerfStat => (),
                crate::toolchain::Bencher::PerfRecord => {
                    cmd.arg("--output")
                        .arg(self.cwd.join(format!("{:02}_perf.data", iteration)));
                }
            }

            let output = command_output(&mut cmd)?;

            match perf_tool.get_bencher() {
                crate::toolchain::Bencher::PerfStat => {
                    let output = process_benchmark_output(output)?;
                    result.append(output);
                }
                crate::toolchain::Bencher::PerfRecord => (),
            }
        }
        Ok(Some(result))
    }

    fn draw_flame_graph(&self, out_path: &Path) -> anyhow::Result<()> {
        match self.build() {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Fail to compile test for {}.", self.processor_name);
                eprintln!("{:?}", err);
                return Err(err);
            }
        }
        eprintln!("drawing flamegraph for '{}' Bench", self.processor_name);
        let mut cmd = self.base_flame_graph_command(out_path);
        command_discard_output(&mut cmd)?;
        Ok(())
    }
}

#[test]
fn test_cargo_example() {
    let mut cmd = Command::new(
        "/home/fxl191220029/study/Rust_Performance_Benchmark/collector/target/release/runtime-fake",
    );
    cmd.current_dir("/home/fxl191220029/study/benchmark_exp/bat/")
        .env("RUNTIME_ELF", "./target/release/examples/advanced");

    let output = command_output(&mut cmd);
    match output {
        Ok(output) => {
            eprintln!("{:?}", output);
            println!("{:?}", process_benchmark_output(output));
        }
        _ => eprintln!("{:?}", output),
    }
}
