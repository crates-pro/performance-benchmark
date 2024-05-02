use std::{
    io::BufReader,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use anyhow::Context;
use cargo_metadata::Message;

use crate::{
    execute::process_benchmark_output,
    toolchain::{Compiler, PerfTool},
    utils::command::{command_discard_output, command_output},
};

use super::{Runtime, FAKE_FLAMEGRAPH, FAKE_RUNTIME};
use crate::statistics::runtime_stat::RuntimeResult;

pub struct RuntimeProcess<'a> {
    compiler: Compiler<'a>,
    cwd: &'a Path,
    elf: PathBuf,
    name: String,
    args: Vec<String>,
    manifest_path: String,
    iterations: u32,
}

impl<'a> RuntimeProcess<'a> {
    pub fn new(
        compiler: Compiler<'a>,
        cwd: &'a Path,
        name: String,
        args: Vec<String>,
        manifest_path: String,
        iterations: u32,
    ) -> Self {
        let mut process = RuntimeProcess {
            compiler,
            cwd,
            elf: PathBuf::new(),
            name,
            args,
            manifest_path,
            iterations,
        };

        match process.build() {
            Ok(elf_path) => {
                eprintln!("Successfully compiled {}.", process.name);
                process.elf = elf_path;
            }
            Err(_) => {
                eprintln!("Fail to compile {}.", process.name);
            }
        };
        process
    }

    fn base_command(&self) -> Command {
        let mut cmd = Command::new(Path::new(&*FAKE_RUNTIME));
        cmd.env("RUNTIME_ELF", self.elf.clone())
            .args(self.args.clone());
        cmd
    }

    fn base_flame_graph_command(&self, out: &Path) -> Command {
        let mut cmd = Command::new(&*FAKE_FLAMEGRAPH);

        let mut flame_graph_file_name = self.name.clone();
        flame_graph_file_name += "_runtime.svg";
        cmd.arg("--output")
            .arg(out.join(flame_graph_file_name))
            .arg("--")
            .arg(self.elf.clone())
            .args(self.args.clone());
        cmd
    }

    fn build(&self) -> anyhow::Result<PathBuf> {
        let mut path = PathBuf::new();
        let mut cmd = Command::new(Path::new(self.compiler.cargo));
        cmd.current_dir(self.cwd)
            .env("RUSTC", self.compiler.rustc)
            .arg("build")
            .arg("--release")
            .arg("--message-format")
            .arg("--manifest-path")
            .arg(&self.manifest_path)
            .arg("json-diagnostic-rendered-ansi");

        let mut child = cmd
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|error| anyhow::anyhow!("Failed to start cargo: {:?}", error))?;

        let stream = BufReader::new(child.stdout.take().unwrap());
        for message in Message::parse_stream(stream) {
            let message = message?;
            match message {
                Message::CompilerArtifact(artifact) => {
                    if let Some(ref executable) = artifact.executable {
                        if artifact.target.kind.iter().any(|k| k == "bin") {
                            path = executable.as_std_path().to_path_buf();
                            log::info!("Compiled {}", path.display());
                        }
                    }
                }
                Message::TextLine(_line) => (),
                Message::CompilerMessage(_msg) => (),
                _ => {
                    log::debug!("Cargo metadata output: {:?}", message);
                }
            }
        }
        Ok(path)
    }
}

impl<'a> Runtime for RuntimeProcess<'a> {
    fn measure(
        &self,

        perf_tool: &PerfTool,
        _event_filter_file: &PathBuf,
    ) -> anyhow::Result<Option<RuntimeResult>> {
        let mut result = RuntimeResult::new(self.name.clone());

        for iteration in 0..self.iterations {
            eprintln!(
                "running '{}' Runtime iteration {}/{}",
                self.name,
                iteration + 1,
                self.iterations
            );

            let mut cmd = self.base_command();
            log::info!("{:?}", cmd);
            log::info!("{:?}", cmd.get_args());

            let perf_tool_name = perf_tool.name();
            cmd.arg("--wrap-rustc-with");
            cmd.arg(perf_tool_name);

            match perf_tool.get_bencher() {
                crate::toolchain::Bencher::PerfStat => (),
                crate::toolchain::Bencher::PerfRecord => {
                    cmd.arg("--output")
                        .arg(self.cwd.join(format!("{}_perf.data", iteration)));
                }
            }

            match perf_tool.get_bencher() {
                crate::toolchain::Bencher::PerfStat => {
                    let output = command_output(&mut cmd)
                        .with_context(|| format!("fail to start benchmark process."))?;
                    let stats = process_benchmark_output(output)?;
                    log::info!("stats:{:?}", stats);
                    result.append(stats);
                }
                crate::toolchain::Bencher::PerfRecord => (),
            }
        }

        Ok(Some(result))
    }

    fn draw_flame_graph(&self, out_path: &Path) -> anyhow::Result<()> {
        println!("drawing flamegraph for '{}'", self.name);
        let mut cmd = self.base_flame_graph_command(out_path);
        command_discard_output(&mut cmd)?;
        Ok(())
    }
}
