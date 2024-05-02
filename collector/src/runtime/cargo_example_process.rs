use std::{
    io::BufReader,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use cargo_metadata::Message;

use crate::{
    execute::{process_benchmark_output, Stats},
    statistics::runtime_stat::RuntimeResult,
    toolchain::{Compiler, PerfTool},
    utils::command::{command_discard_output, command_output},
};

use super::{Runtime, FAKE_FLAMEGRAPH, FAKE_RUNTIME};

pub struct CargoExampleProcess<'a> {
    pub compiler: Compiler<'a>,
    pub processor_name: String,
    pub cwd: &'a Path,
    pub manifest_path: String,
    pub iterations: u32,
    pub examples: Vec<String>,
    pub example_elf: Vec<(String, PathBuf)>,
    pub args: Vec<String>,
}

impl<'a> CargoExampleProcess<'a> {
    pub fn new(
        compiler: Compiler<'a>,
        processor_name: String,
        cwd: &'a Path,
        manifest_path: String,
        iterations: u32,
        examples: Vec<String>,
        args: Vec<String>,
    ) -> Self {
        let mut process = Self {
            compiler,
            processor_name,
            cwd,
            manifest_path,
            iterations,
            examples,
            example_elf: vec![],
            args,
        };
        match process.build() {
            Ok(_) => (),
            Err(_) => eprintln!("Fail to compile examples for {}.", process.processor_name),
        };

        process
    }

    fn build_command(&self, example_name: &String) -> Command {
        let mut cmd = Command::new(self.compiler.cargo);
        cmd.env("RUSTC", self.compiler.rustc)
            .current_dir(self.cwd)
            .arg("rustc")
            .args(self.args.clone())
            .arg("--profile")
            .arg("release")
            .arg("--example")
            .arg(example_name)
            .arg("--manifest-path")
            .arg(&self.manifest_path)
            .arg("--message-format")
            .arg("json-diagnostic-rendered-ansi");
        cmd
    }

    fn base_command(&self, elf: &Path) -> Command {
        let mut cmd = Command::new(Path::new(&*FAKE_RUNTIME));
        cmd.env("RUNTIME_ELF", elf);
        cmd
    }

    fn base_flame_graph_command(&self, example_name: &String, elf: &Path, out: &Path) -> Command {
        let mut cmd = Command::new(&*FAKE_FLAMEGRAPH);

        let mut flame_graph_file_name = self.processor_name.clone() + "_example_" + example_name;
        flame_graph_file_name += "_runtime.svg";
        cmd.current_dir(self.cwd)
            .arg("--output")
            .arg(out.join(flame_graph_file_name))
            .arg("--")
            .arg(elf);
        cmd
    }

    fn build(&mut self) -> anyhow::Result<()> {
        for example in &self.examples {
            eprintln!(
                "Building example {} for {}...",
                example, self.processor_name
            );

            let mut cmd = self.build_command(example);

            let mut child = cmd
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()
                .map_err(|error| anyhow::anyhow!("Failed to start cargo: {:?}", error))?;

            let stream = BufReader::new(child.stdout.take().unwrap());
            let mut beacon = false;
            for message in Message::parse_stream(stream) {
                let message = message?;
                match message {
                    Message::CompilerArtifact(artifact) => {
                        if let Some(ref executable) = artifact.executable {
                            if artifact.target.kind.iter().any(|k| k == "example") {
                                let path = executable.as_std_path().to_path_buf();
                                log::info!("Compiled {}", path.display());
                                self.example_elf.push((example.clone(), path));
                                beacon = true;
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

            if beacon == false {
                eprintln!("Fail to compile example {}, will be skipped.", example);
            }
        }
        log::info!("{:?}", self.example_elf);

        if self.example_elf.len() != self.examples.len() {
            eprintln!(
                "{} example(s) skipped due to compile failure.",
                self.examples.len() - self.example_elf.len()
            );
        }
        Ok(())
    }
}

impl<'a> Runtime for CargoExampleProcess<'a> {
    fn measure(
        &self,
        perf_tool: &PerfTool,
        _event_filter_file: &PathBuf,
    ) -> anyhow::Result<Option<RuntimeResult>> {
        log::info!("{:?}", self.examples);

        let mut result = RuntimeResult::new(self.processor_name.clone());

        for iteration in 0..self.iterations {
            let mut output_sum = Stats::new();
            for (example, elf) in &self.example_elf {
                eprintln!(
                    "running '{}-example-{}' Runtime iteration {}/{}",
                    self.processor_name.clone(),
                    example,
                    iteration + 1,
                    self.iterations
                );

                let mut cmd = self.base_command(elf);

                match perf_tool.get_bencher() {
                    crate::toolchain::Bencher::PerfStat => (),
                    crate::toolchain::Bencher::PerfRecord => {
                        cmd.arg("--output").arg(
                            self.cwd
                                .join(format!("{}_{:02}_perf.data", example, iteration)),
                        );
                    }
                }

                let output = command_output(&mut cmd)?;

                let output = process_benchmark_output(output)?;

                match perf_tool.get_bencher() {
                    crate::toolchain::Bencher::PerfStat => output_sum += output,
                    crate::toolchain::Bencher::PerfRecord => (),
                }
            }

            result.append(output_sum);
        }
        Ok(Some(result))
    }

    fn draw_flame_graph(&self, out_path: &Path) -> anyhow::Result<()> {
        for (example, elf) in &self.example_elf {
            eprintln!(
                "drawing flamegraph for '{}-example-{}'",
                self.processor_name, example
            );
            let mut cmd = self.base_flame_graph_command(example, elf, out_path);
            command_discard_output(&mut cmd)?;
        }
        Ok(())
    }
}

#[test]
fn test_cargo_example() {
    let mut cmd = Command::new(
        "/home/fxl191220029/study/Rust_Performance_Benchmark/collector/target/release/runtime-fake",
    );
    cmd.current_dir("/home/fxl191220029/study/test/bat/")
        .env("RUNTIME_ELF", "./target/release/examples/yaml");

    let output = command_output(&mut cmd);
    match output {
        Ok(output) => {
            println!("{:?}", process_benchmark_output(output));
        }
        _ => eprintln!("{:?}", output),
    }
}
