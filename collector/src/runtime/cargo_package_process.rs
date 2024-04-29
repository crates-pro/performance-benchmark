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

pub struct CargoPackageTestProcess<'a> {
    pub compiler: Compiler<'a>,
    pub processor_name: String,
    pub cwd: &'a Path,
    pub manifest_path: String,
    pub iterations: u32,
    pub args: Vec<String>,
    pub packages: Vec<String>,
}

impl<'a> CargoPackageTestProcess<'a> {
    fn base_command(&self) -> Command {
        let mut cmd = Command::new(&*FAKE_RUNTIME);
        cmd.env("RUNTIME_ELF", self.compiler.cargo)
            .env("RUSTC", self.compiler.rustc)
            .env("CARGO_INCREMENTAL", "1")
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

    fn add_packages(&self, cmd: &mut Command) {
        self.packages.iter().for_each(|p| {
            cmd.arg("--package");
            cmd.arg(p);
        });
    }

    fn compile_test(&self) -> anyhow::Result<()> {
        let mut cmd = self.base_command();
        cmd.arg("--no-run").env("CARGO_INCREMENTAL", "0");
        self.add_packages(&mut cmd);

        command_discard_output(&mut cmd)
    }
}

impl<'a> Runtime for CargoPackageTestProcess<'a> {
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
            self.add_packages(&mut cmd);

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
