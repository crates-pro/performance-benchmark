use std::{
    ffi::OsString,
    mem::ManuallyDrop,
    path::{Path, PathBuf},
};

use crate::{
    benchmark::{
        benchmark::{Benchamrk, BenchmarkSuit, CompileTimeType},
        profile::Profile,
        scenario::Scenario,
    },
    compile_time::{
        binary_size::binary_single_process::BinarySingleProcess, discover_benchmark_suit,
        result::CompileTimeResult,
    },
    execute::Stats,
    toolchain::{Compiler, LocalToolchain},
};

use self::binary_package_process::BinaryPackageProcess;

use super::result::CompileTimeBenchResult;

pub mod binary_package_process;
pub mod binary_single_process;

pub trait BinaryProcess {
    fn run_rustc(&self) -> anyhow::Result<Option<Stats>>;

    /// Files with ".d" suffix or directorys that are not relevant to the
    /// binary target should be filtered out.
    fn is_filtered_file_name(&self, file_name: OsString) -> bool {
        let filted_names = vec!["build", "deps", "examples", "incremental"];
        if let Some(file_name) = file_name.to_str() {
            filted_names.contains(&file_name) | file_name.ends_with(".d")
        } else {
            true
        }
    }
}

pub fn bench_binary_size(
    ltc: &LocalToolchain,
    profiles: &[Profile],
    benchmark_dir: PathBuf,
) -> anyhow::Result<Vec<CompileTimeBenchResult>> {
    let benchmark_suit = BenchmarkSuit {
        benchmarks: discover_benchmark_suit(&benchmark_dir)?,
    };
    println!("{}", benchmark_suit.display_benchmarks());

    let mut results = vec![];
    let mut num_benchmark_left = benchmark_suit.benchmarks.len();
    benchmark_suit.benchmarks.iter().for_each(|b| {
        println!("{} benchmarks waiting.", num_benchmark_left);
        println!("Binary-size benchmarking for '{}'", b.name.as_str());
        num_benchmark_left -= 1;

        match b.bench_binary_size(ltc, profiles) {
            Ok(result) => results.push(result),
            Err(e) => eprintln!("Faile to bench '{}'! {}", b.name, e),
        };
    });

    Ok(results)
}

impl Benchamrk {
    fn bench_binary_size(
        &self,
        ltc: &LocalToolchain,
        profiles: &[Profile],
    ) -> anyhow::Result<CompileTimeBenchResult> {
        if self.config.disabled {
            println!("Skipping {}: disabled", self.name);
        }

        print!("Preparing {}...", self.name);

        let tmp_dir = ManuallyDrop::new(self.make_temp_dir(&self.path)?);

        let mut bench_result = CompileTimeBenchResult::new(self.name.clone(), 1);

        for profile in profiles {
            let binary_size_process = self.make_binray_size_process(
                Compiler::from_toolchain(ltc),
                tmp_dir.path(),
                *profile,
            );

            if let Some(stats) = binary_size_process.run_rustc()? {
                bench_result.add_result(CompileTimeResult::new(
                    self.name.clone(),
                    1,
                    *profile,
                    Scenario::Full,
                    stats,
                ));
            }
        }

        drop(ManuallyDrop::into_inner(tmp_dir));

        println!("Bench success.");

        Ok(bench_result)
    }

    fn make_binray_size_process<'a>(
        &'a self,
        compiler: Compiler<'a>,
        cwd: &'a Path,
        profile: Profile,
    ) -> Box<dyn BinaryProcess + 'a> {
        let compile_type = if let Some(t) = &self.config.compile_time_type {
            t
        } else {
            &CompileTimeType::Single
        };

        match compile_type {
            CompileTimeType::Single => Box::new(BinarySingleProcess {
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

                cargo_args: self
                    .config
                    .cargo_opts
                    .clone()
                    .unwrap_or_default()
                    .split_whitespace()
                    .map(String::from)
                    .collect::<Vec<_>>(),
                rustc_args: self
                    .config
                    .cargo_rustc_opts
                    .clone()
                    .unwrap_or_default()
                    .split_whitespace()
                    .map(String::from)
                    .collect(),
                touch_file: self.config.touch_file.clone(),
            }),
            CompileTimeType::Packages => Box::new(BinaryPackageProcess {
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

                cargo_args: self
                    .config
                    .cargo_opts
                    .clone()
                    .unwrap_or_default()
                    .split_whitespace()
                    .map(String::from)
                    .collect::<Vec<_>>(),
                rustc_args: self
                    .config
                    .cargo_rustc_opts
                    .clone()
                    .unwrap_or_default()
                    .split_whitespace()
                    .map(String::from)
                    .collect(),
                touch_file: self.config.touch_file.clone(),
                packages: self.config.packages.clone().unwrap(),
            }),
        }
    }
}

#[cfg(test)]
mod test_binary_size {
    use std::{path::PathBuf, process::Command};

    use anyhow::Context;

    use crate::{benchmark::profile::Profile, toolchain::LocalToolchain};

    use super::bench_binary_size;

    fn get_rustc() -> anyhow::Result<String> {
        let output = Command::new("which")
            .arg("rustc")
            .output()
            .context("failed to run `which rustc`")?;

        if !output.status.success() {
            anyhow::bail!("did not manage to obtain rustc");
        }

        let s = String::from_utf8(output.stdout)
            .context("failed to convert `which rustc` output to utf8")?;
        Ok(s.trim_end().to_string())
    }

    fn get_cargo() -> anyhow::Result<String> {
        let output = Command::new("which")
            .arg("cargo")
            .output()
            .context("failed to run `which cargo`")?;

        if !output.status.success() {
            anyhow::bail!("did not manage to obtain cargo");
        }

        let s = String::from_utf8(output.stdout)
            .context("failed to convert `which cargo` output to utf8")?;
        Ok(s.trim_end().to_string())
    }

    #[test]
    fn test_bench_binary_size() {
        let results = bench_binary_size(
            &LocalToolchain {
                rustc: PathBuf::from(get_rustc().unwrap()),
                cargo: PathBuf::from(get_cargo().unwrap()),
                flame_graph: PathBuf::new(),
                id: String::new(),
            },
            &[Profile::Release],
            PathBuf::from("test/binary_size/benchmarks"),
        )
        .unwrap();

        results.iter().for_each(|r| {
            let r = r.get_stats_ref_by_profile(&Profile::Release);
            assert_ne!(r.is_empty(), true);
            r.iter()
                .for_each(|s| s.stats.values().for_each(|v| assert!(*v > 0.)));
        })
    }
}
