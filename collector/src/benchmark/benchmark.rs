use std::{
    collections::HashMap,
    fs::{create_dir_all, read_dir, File},
    mem::ManuallyDrop,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Ok};
use serde::Deserialize;
use tempfile::TempDir;

use crate::{
    compile_time::{
        cargo_package_process::CargoPackageProcess,
        cargo_single_process::CargoSingleProcess,
        result::{CompileTimeBenchResult, CompileTimeResult},
        CompileTimeProcessor,
    },
    runtime::{
        cargo_bench_process::CargoBenchProcess, cargo_example_process::CargoExampleProcess,
        cargo_test_process::CargoTestProcess, measure::RuntimeProcess, Runtime,
    },
    toolchain::{Compiler, PerfTool},
};

use super::{profile::Profile, scenario::Scenario};

fn default_runs() -> usize {
    3
}

pub struct Benchamrk {
    pub name: String,
    pub path: PathBuf,
    pub config: BenchmarkConfig,
}

impl Benchamrk {
    pub fn new(name: String, path: PathBuf) -> anyhow::Result<Self> {
        let config_path = path.join("perf-config.json");
        let config: BenchmarkConfig = if config_path.exists() {
            serde_json::from_reader(
                File::open(&config_path)
                    .with_context(|| format!("failed to open {:?}", config_path))?,
            )
            .with_context(|| format!("failed to parse {:?}", config_path))?
        } else {
            bail!("missing a perf-config.json file for `{}`", name);
        };
        Ok(Benchamrk { name, path, config })
    }

    pub fn measure_compile_time(
        &self,
        perf_tool: &PerfTool,
        event_filter_file: &PathBuf,
        profiles: &[Profile],
        scenarios: &[Scenario],
        compiler: Compiler<'_>,
        iterations: Option<usize>,
        flame_graph_result_path: &Path,
        flamegraph_flag: i32,
        out_dir: &Path,
    ) -> anyhow::Result<CompileTimeBenchResult> {
        if self.config.disabled {
            eprintln!("Skipping {}: disabled", self.name);
            bail!("disabled benchmark");
        }

        let iterations = iterations.unwrap_or(self.config.runs);

        let mut bench_result = CompileTimeBenchResult::new(self.name.clone(), iterations);

        if profiles.is_empty() {
            eprintln!("Skipping {}: no profiles selected", self.name);
            return Ok(bench_result);
        }

        eprintln!("Preparing {}...", self.name);

        let profile_dirs = profiles
            .iter()
            .map(|profile| Ok((*profile, self.make_temp_dir(&self.path)?)))
            .collect::<anyhow::Result<Vec<_>>>()?;

        for (profile, dir) in profile_dirs {
            let mut result_map = HashMap::new();
            scenarios.iter().for_each(|scenario| {
                result_map.insert(scenario.clone(), vec![]);
            });

            for i in 0..iterations {
                println!(
                    "running '{}' {:?} + {:?} iteration {}/{}",
                    self.name,
                    profile,
                    scenarios,
                    i + 1,
                    iterations
                );
                let timing_dir = ManuallyDrop::new(self.make_temp_dir(dir.path())?);

                let cwd = timing_dir.path();

                if scenarios.contains(&Scenario::Full) {
                    let mut cargo_process = self.make_cargo_process(compiler, cwd, profile);
                    cargo_process.gen_pkg()?;
                    let stats = cargo_process
                        .run_rustc(perf_tool, event_filter_file, true)
                        .with_context(|| format!("Fail to run rust_c for {}", self.name));

                    match stats {
                        Result::Ok(stats) => {
                            if let Some(stats) = stats {
                                let result_vec = result_map.get_mut(&Scenario::Full).unwrap();
                                result_vec.push(CompileTimeResult::new(
                                    self.name.clone(),
                                    i + 1,
                                    profile,
                                    Scenario::Full,
                                    stats,
                                ));
                            }
                        }
                        Err(s) => {
                            eprintln!(
                                "Fail to bench {} with Profile {:?}: {:?}. Skip.",
                                self.name, profile, s
                            );
                            drop(cargo_process);
                            drop(ManuallyDrop::into_inner(timing_dir));
                            continue;
                        }
                    }
                }
                // Rustdoc does not support incremental compilation
                if profile != Profile::Doc {
                    // An incremental  from scratch (slowest incremental case).
                    // This is required for any subsequent incremental builds.
                    if scenarios.iter().any(|s| s.is_increment()) {
                        let mut process = self.make_cargo_process(compiler, cwd, profile);
                        process.increment(true);
                        let stats = process
                            .run_rustc(perf_tool, event_filter_file, true)
                            .with_context(|| format!("Fail to run rust_c for {}", self.name));

                        match stats {
                            Result::Ok(stats) => {
                                if let Some(stats) = stats {
                                    let result_vec = result_map.get_mut(&Scenario::Full).unwrap();
                                    result_vec.push(CompileTimeResult::new(
                                        self.name.clone(),
                                        i + 1,
                                        profile,
                                        Scenario::Full,
                                        stats,
                                    ));
                                }
                            }
                            Err(s) => {
                                eprintln!(
                                    "Fail to bench {} with Profile {:?}: {:?}. Skip.",
                                    self.name, profile, s
                                );
                                drop(process);
                                drop(ManuallyDrop::into_inner(timing_dir));
                                continue;
                            }
                        }
                    }

                    // An incremental build with no changes (fastest incremental case).
                    if scenarios.contains(&Scenario::IncrUnchanged) {
                        let mut process = self.make_cargo_process(compiler, cwd, profile);
                        process.increment(true);
                        let stats = process.run_rustc(perf_tool, event_filter_file, true)?;
                        let result_vec = result_map.get_mut(&Scenario::Full).unwrap();

                        if let Some(stats) = stats {
                            result_vec.push(CompileTimeResult::new(
                                self.name.clone(),
                                i + 1,
                                profile,
                                Scenario::Full,
                                stats,
                            ));
                        }
                    }

                    // If BenchTool is PerfRecord, we need to move perf.data
                    // out of the temp dir.
                    match perf_tool.get_bencher() {
                        crate::toolchain::Bencher::PerfStat => (),
                        crate::toolchain::Bencher::PerfRecord => {
                            let cwd = timing_dir.path();
                            let dst_dir = out_dir.join(self.name.clone());

                            let _ = create_dir_all(dst_dir.clone());

                            // Find file whoose name contains "perf.analyze" in directory cwd,
                            // and copy it into directory dst_dir.
                            for entry in read_dir(cwd)? {
                                let entry = entry?;
                                if entry.file_name().to_str().unwrap().contains("perf.data") {
                                    let src_path = PathBuf::from(entry.file_name());
                                    let dst_path = dst_dir.join(format!(
                                        "{}_{:02}_{}",
                                        profile,
                                        i,
                                        src_path.as_os_str().to_str().unwrap()
                                    ));
                                    if let Err(err) = std::fs::copy(cwd.join(src_path), &dst_path) {
                                        eprintln!("Failed to copy 'perf.data' file: {}", err);
                                    }
                                }
                            }
                        }
                    }
                }
                drop(ManuallyDrop::into_inner(timing_dir));
            }

            result_map.iter().for_each(|(_scenario, results)| {
                results.iter().for_each(|result| {
                    bench_result.add_result(result.clone());
                });
            });

            //draw flamegraph with Full Scenario
            if flamegraph_flag > 0 {
                println!(
                    "drawing flamegraph for '{}' {:?} + {:?}",
                    self.name,
                    profile,
                    Scenario::Full
                );
                let timing_dir = ManuallyDrop::new(self.make_temp_dir(dir.path())?);
                let cwd = timing_dir.path();

                let cargo_process = self.make_cargo_process(compiler, cwd, profile);
                cargo_process.gen_pkg()?;
                cargo_process.draw_flame_graph(flame_graph_result_path)?;

                drop(cargo_process);
                drop(ManuallyDrop::into_inner(timing_dir));
            }
        }

        Ok(bench_result)
    }

    pub fn make_temp_dir(&self, base: &Path) -> anyhow::Result<TempDir> {
        let mut base_dot = base.to_path_buf();
        base_dot.push(".");
        let tmp_dir = TempDir::new()?;
        match Self::copy(&base_dot, tmp_dir.path()) {
            core::result::Result::Ok(_) => Ok(tmp_dir),
            Err(s) => {
                eprintln!("{:?}", s);
                Err(s)
            }
        }
    }

    #[cfg(windows)]
    fn copy(from: &Path, to: &Path) -> anyhow::Result<()> {
        crate::utils::fs::robocopy(from, to, &[])
    }

    #[cfg(unix)]
    fn copy(from: &Path, to: &Path) -> anyhow::Result<()> {
        use crate::utils::command::command_output;
        use std::process::Command;

        let mut cmd = Command::new("cp");
        cmd.arg("-pLR").arg(from).arg(to);
        command_output(&mut cmd)?;
        Ok(())
    }

    fn make_cargo_process<'a>(
        &'a self,
        compiler: Compiler<'a>,
        cwd: &'a Path,
        profile: Profile,
    ) -> Box<dyn CompileTimeProcessor + 'a> {
        let mut cargo_args = self
            .config
            .cargo_opts
            .clone()
            .unwrap_or_default()
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<_>>();
        if let Some(count) = std::env::var("CARGO_THREAD_COUNT")
            .ok()
            .and_then(|v| v.parse::<u32>().ok())
        {
            cargo_args.push(format!("-j{}", count));
        }

        match &self.config.compile_time_type {
            Some(ctt) => match ctt {
                CompileTimeType::Single => {
                    let process = CargoSingleProcess {
                        compiler,
                        processor_name: self.name.clone(),
                        cwd,
                        profile,
                        incremental: false,
                        manifest_path: self
                            .config
                            .cargo_toml
                            .clone()
                            .unwrap_or_else(|| String::from("Cargo.toml")),
                        cargo_args,
                        rustc_args: self
                            .config
                            .cargo_rustc_opts
                            .clone()
                            .unwrap_or_default()
                            .split_whitespace()
                            .map(String::from)
                            .collect(),
                        touch_file: self.config.touch_file.clone(),
                    };
                    Box::new(process)
                }
                CompileTimeType::Packages => match &self.config.packages {
                    Some(packages) => {
                        let process = CargoPackageProcess {
                            compiler,
                            processor_name: self.name.clone(),
                            cwd,
                            profile,
                            incremental: false,
                            manifest_path: self
                                .config
                                .cargo_toml
                                .clone()
                                .unwrap_or_else(|| String::from("Cargo.toml")),
                            cargo_args,
                            rustc_args: self
                                .config
                                .cargo_rustc_opts
                                .clone()
                                .unwrap_or_default()
                                .split_whitespace()
                                .map(String::from)
                                .collect(),
                            touch_file: self.config.touch_file.clone(),
                            packages: packages.clone(),
                        };
                        Box::new(process)
                    }
                    None => {
                        eprintln!("Missing package list in perf-config. Generating a Single Processor instead.");
                        let process = CargoSingleProcess {
                            compiler,
                            processor_name: self.name.clone(),
                            cwd,
                            profile,
                            incremental: false,
                            manifest_path: self
                                .config
                                .cargo_toml
                                .clone()
                                .unwrap_or_else(|| String::from("Cargo.toml")),
                            cargo_args,
                            rustc_args: self
                                .config
                                .cargo_rustc_opts
                                .clone()
                                .unwrap_or_default()
                                .split_whitespace()
                                .map(String::from)
                                .collect(),
                            touch_file: self.config.touch_file.clone(),
                        };
                        Box::new(process)
                    }
                },
            },
            None => {
                let process = CargoSingleProcess {
                    compiler,
                    processor_name: self.name.clone(),
                    cwd,
                    profile,
                    incremental: false,
                    manifest_path: self
                        .config
                        .cargo_toml
                        .clone()
                        .unwrap_or_else(|| String::from("Cargo.toml")),
                    cargo_args,
                    rustc_args: self
                        .config
                        .cargo_rustc_opts
                        .clone()
                        .unwrap_or_default()
                        .split_whitespace()
                        .map(String::from)
                        .collect(),
                    touch_file: self.config.touch_file.clone(),
                };
                Box::new(process)
            }
        }
    }

    pub fn make_runtime_process<'a>(
        &'a self,
        compiler: Compiler<'a>,
        cwd: &'a Path,
        iterations: u32,
    ) -> Result<Box<dyn Runtime + 'a>, String> {
        log::debug!(
            "make_runtime_process: get runtime args: {:?}",
            self.config.runtime_args
        );
        match &self.config.runtime_test_type {
            Some(test_type) => match test_type {
                RuntimeTestType::Test => {
                    let process = CargoTestProcess {
                        compiler,
                        processor_name: self.name.clone(),
                        cwd,
                        manifest_path: self
                            .config
                            .cargo_toml
                            .clone()
                            .unwrap_or_else(|| String::from("Cargo.toml")),
                        iterations,
                        args: self
                            .config
                            .runtime_args
                            .clone()
                            .unwrap_or_default()
                            .split_whitespace()
                            .map(String::from)
                            .collect(),
                    };
                    core::result::Result::Ok(Box::new(process))
                }
                RuntimeTestType::Example => {
                    if self.config.example_lst == None {
                        return core::result::Result::Err(format!(
                            "missing example list in json-profile for benchmark {}",
                            self.name
                        ));
                    }
                    let process = CargoExampleProcess::new(
                        compiler,
                        self.name.clone(),
                        cwd,
                        self.config
                            .cargo_toml
                            .clone()
                            .unwrap_or_else(|| String::from("Cargo.toml")),
                        iterations,
                        self.config.example_lst.clone().unwrap(),
                        self.config
                            .runtime_args
                            .clone()
                            .unwrap_or_default()
                            .split_whitespace()
                            .map(String::from)
                            .collect(),
                    );
                    core::result::Result::Ok(Box::new(process))
                }
                RuntimeTestType::Binary => {
                    let process = RuntimeProcess::new(
                        compiler,
                        cwd,
                        self.name.clone(),
                        self.config
                            .runtime_args
                            .clone()
                            .unwrap_or_default()
                            .split_whitespace()
                            .map(String::from)
                            .collect(),
                        self.config
                            .cargo_toml
                            .clone()
                            .unwrap_or_else(|| String::from("Cargo.toml")),
                        iterations,
                    );
                    core::result::Result::Ok(Box::new(process))
                }
                RuntimeTestType::Bench => {
                    let process = CargoBenchProcess::new(
                        compiler,
                        self.name.clone(),
                        cwd,
                        self.config
                            .cargo_toml
                            .clone()
                            .unwrap_or_else(|| String::from("Cargo.toml")),
                        iterations,
                        self.config
                            .runtime_args
                            .clone()
                            .unwrap_or_default()
                            .split_whitespace()
                            .map(String::from)
                            .collect(),
                    );
                    core::result::Result::Ok(Box::new(process))
                }
            },
            None => {
                let process = CargoTestProcess {
                    compiler,
                    processor_name: self.name.clone(),
                    cwd,
                    manifest_path: self
                        .config
                        .cargo_toml
                        .clone()
                        .unwrap_or_else(|| String::from("Cargo.toml")),
                    iterations,
                    args: self
                        .config
                        .runtime_args
                        .clone()
                        .unwrap_or_default()
                        .split_whitespace()
                        .map(String::from)
                        .collect(),
                };
                core::result::Result::Ok(Box::new(process))
            }
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize, Default)]
pub struct BenchmarkConfig {
    pub cargo_opts: Option<String>,
    pub cargo_rustc_opts: Option<String>,
    pub cargo_toml: Option<String>,
    pub compile_time_type: Option<CompileTimeType>,
    pub packages: Option<Vec<String>>,
    pub runtime_test_type: Option<RuntimeTestType>,
    pub example_lst: Option<Vec<String>>,
    pub runtime_args: Option<String>,
    /// The file that should be touched to ensure cargo re-checks the leaf crate
    /// we're interested in. Likely, something similar to `src/lib.rs`. The
    /// default if this is not present is to touch all .rs files in the
    /// directory that `Cargo.toml` is in.
    #[serde(default)]
    pub touch_file: Option<String>,
    #[serde(default)]
    pub disabled: bool,
    #[serde(default = "default_runs")]
    pub runs: usize,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub enum RuntimeTestType {
    #[default]
    Test,
    Example,
    Binary,
    Bench,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub enum CompileTimeType {
    #[default]
    Single,
    Packages,
}

pub struct BenchmarkSuit {
    pub benchmarks: Vec<Benchamrk>,
}

impl BenchmarkSuit {
    pub fn display_benchmarks(&self) -> String {
        let mut names = String::from("+BenchmarkSuit");
        for b in self.benchmarks.iter() {
            names = names + "\n| ";
            names = names + &b.name;
            names = names + " " + b.path.to_str().unwrap().as_ref();
        }
        names
    }
}
