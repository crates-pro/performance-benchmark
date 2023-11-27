use std::{
    path::{Path, PathBuf},
    process::Command,
};

use crate::{
    benchmark::profile::Profile,
    execute::{process_benchmark_output, Stats},
    toolchain::{Compiler, PerfTool},
    utils::{
        self,
        command::{command_discard_output, command_output},
    },
};

use super::{CompileTimeProcessor, FAKE_FLAMEGRAPH, FAKE_RUSTC};

pub struct CargoPackageProcess<'a> {
    pub compiler: Compiler<'a>,
    pub processor_name: String,
    pub cwd: &'a Path,
    pub profile: Profile,
    pub incremental: bool,
    pub manifest_path: String,
    pub cargo_args: Vec<String>,
    pub rustc_args: Vec<String>,
    pub touch_file: Option<String>,
    pub packages: Vec<String>,
}

impl<'a> CargoPackageProcess<'a> {
    fn base_command(&self, cwd: &Path, subcommand: &str, package: &String) -> Command {
        let mut cmd = Command::new(Path::new(self.compiler.cargo));
        cmd
            // Not all cargo invocations (e.g. `cargo clean`) need all of these
            // env vars set, but it doesn't hurt to have them.
            .env("RUSTC", &*FAKE_RUSTC)
            .env("RUSTC_REAL", &self.compiler.rustc)
            // We separately pass -Cincremental to the leaf crate --
            // CARGO_INCREMENTAL is cached separately for both the leaf crate
            // and any in-tree dependencies, and we don't want that; it wastes
            // time.
            .env("CARGO_INCREMENTAL", "0")
            // We need to use -Z flags (for example, to force enable ICH
            // verification) so unconditionally enable unstable features, even
            // on stable compilers.
            .env("RUSTC_BOOTSTRAP", "1")
            .current_dir(cwd)
            .arg(subcommand)
            .arg("--package")
            .arg(package)
            .arg("--manifest-path")
            .arg(&self.manifest_path);

        cmd
    }

    /// Create a command to excecute flamegraph like
    /// `flamegraph --flamechart --output ../../../FlameGraphs/out.svg -- target/release/helloworld`
    fn base_flame_graph_command(&self, cwd: &Path, out: &Path, package: &String) -> Command {
        let mut cmd = Command::new(&*FAKE_FLAMEGRAPH);

        let mut flame_graph_file_name = self.processor_name.clone() + "_pkg_" + package;
        flame_graph_file_name += format!("_{:?}.svg", self.profile).as_str();
        cmd.current_dir(cwd)
            .env("CARGO_INCREMENTAL", "0")
            .env("RUSTC_BOOTSTRAP", "1")
            .arg("--output")
            .arg(out.join(flame_graph_file_name))
            .arg("--")
            .arg(&self.compiler.cargo)
            .arg("rustc")
            .env("RUSTC", self.compiler.rustc)
            .arg("--package")
            .arg(package)
            .arg("--manifest-path")
            .arg(&self.manifest_path);
        cmd
    }
}

impl<'a> CompileTimeProcessor for CargoPackageProcess<'a> {
    fn run_rustc(
        &mut self,
        perf_tool: &PerfTool,
        _event_filter_file: &PathBuf,
        needs_final: bool,
    ) -> anyhow::Result<Option<Stats>> {
        let cargo_subcommand = "rustc";

        let mut stats_sum = Stats::default();

        let mut i = 1;
        for package in &self.packages {
            eprintln!(
                "bench package package [{:?} {}/{}] {}...",
                self.profile,
                i,
                self.packages.len(),
                package
            );
            i += 1;

            let mut cmd = self.base_command(self.cwd, cargo_subcommand, package);

            match self.profile {
                Profile::Check => {
                    cmd.arg("--profile").arg("check");
                }
                Profile::Debug => {}
                Profile::Doc => {}
                Profile::Release => {
                    cmd.arg("--release");
                }
            }
            cmd.args(&self.cargo_args);

            if std::env::var_os("CARGO_RECORD_TIMING").is_some() {
                cmd.arg("-Zunstable-options");
                cmd.arg("-Ztimings");
            }
            cmd.arg("--");
            // --wrap-rustc-with is not a valid rustc flag. But rustc-fake
            // recognizes it, strips it (and its argument) out, and uses it as an
            // indicator that the rustc invocation should be profiled. This works
            // out nicely because `cargo rustc` only passes arguments after '--'
            // onto rustc for the final crate, which is exactly the crate for which
            // we want to wrap rustc.
            if needs_final {
                let perf_tool_name = perf_tool.name();
                match perf_tool.get_bencher() {
                    crate::toolchain::Bencher::PerfStat => (),
                    crate::toolchain::Bencher::PerfRecord => {
                        cmd.arg("--output")
                            .arg(self.cwd.join(format!("{}_perf.data", package)));
                    }
                }
                // If we're using a processor, we expect that only the crate
                // we're interested in benchmarking will be built, not any
                // dependencies.
                // cmd.env("EXPECT_ONLY_WRAPPED_RUSTC", "1");
                cmd.arg("--wrap-rustc-with");
                cmd.arg(perf_tool_name);
                cmd.args(&self.rustc_args);
                // If we're not going to be in a processor, then there's no
                // point ensuring that we recompile anything -- that just wastes
                // time.

                // Touch all the files under the Cargo.toml of the manifest we're
                // benchmarking, so as to not refresh dependencies, which may be
                // in-tree (e.g., in the case of the servo crates there are a lot of
                // other components).
                if let Some(file) = &self.touch_file {
                    utils::fs::touch(&self.cwd.join(Path::new(&file)))?;
                } else {
                    utils::fs::touch_all(
                        &self.cwd.join(
                            Path::new(&self.manifest_path)
                                .parent()
                                .expect("manifest has parent"),
                        ),
                    )?;
                }
            } else {
                // If we're not going to record the final rustc, then there's
                // absolutely no point in waiting for it to build. This will
                // have the final rustc just immediately exit(0) without
                // actually running it.
                cmd.arg("--skip-this-rustc");
            }
            if self.incremental {
                cmd.arg("-C");
                let mut incr_arg = std::ffi::OsString::from("incremental=");
                incr_arg.push(self.cwd.join("incremental-state"));
                cmd.arg(incr_arg);
            }

            log::info!("cwd: {:?}", self.cwd);
            log::info!("env: {:?}", cmd.get_envs());
            log::info!("cmd: {:?}", cmd);
            let output = command_output(&mut cmd);
            match output {
                Ok(output) => {
                    log::debug!("output: \n{:?}", output);
                    match perf_tool.get_bencher() {
                        crate::toolchain::Bencher::PerfStat => {
                            let stats = process_benchmark_output(output);
                            match stats {
                                Ok(stats) => stats_sum += stats,
                                Err(err) => {
                                    eprintln!(
                                        "Fail to bench package {} [{:?}]. Skip",
                                        package, self.profile
                                    );
                                    eprintln!("error msg: {:?}", err);
                                    continue;
                                }
                            }
                        }
                        crate::toolchain::Bencher::PerfRecord => (),
                    }
                }
                Err(err) => {
                    eprintln!(
                        "Fail to bench package {} [{:?}]. Skip",
                        package, self.profile
                    );
                    eprintln!("error msg: {:?}", err);
                    continue;
                }
            }
        }

        match perf_tool.get_bencher() {
            crate::toolchain::Bencher::PerfStat => return Ok(Some(stats_sum)),
            crate::toolchain::Bencher::PerfRecord => return Ok(None),
        }
    }

    fn draw_flame_graph(&self, dir: &Path) -> anyhow::Result<()> {
        let mut i = 1;
        for package in &self.packages {
            eprintln!(
                "drawing flamegraph for package [{:?} {}/{}] {}...",
                self.profile,
                i,
                self.packages.len(),
                package
            );
            i += 1;

            let mut cmd = self.base_flame_graph_command(self.cwd, dir, package);
            match self.profile {
                Profile::Check => {
                    cmd.arg("--profile").arg("check");
                }
                Profile::Debug => {}
                Profile::Doc => {}
                Profile::Release => {
                    cmd.arg("--profile").arg("release");
                }
            };

            cmd.args(&self.cargo_args);
            if std::env::var_os("CARGO_RECORD_TIMING").is_some() {
                cmd.arg("-Zunstable-options");
                cmd.arg("-Ztimings");
            }
            cmd.arg("--");
            cmd.args(&self.rustc_args);

            log::info!("{:?}", cmd);
            log::info!("{:?}", cmd.get_current_dir());

            command_discard_output(&mut cmd)?;
        }

        Ok(())
    }

    fn gen_pkg(&self) -> anyhow::Result<()> {
        // When using cargo_package_processor, there is no need to
        // generate pkgid, because we specify its package name when calling
        // run_rustc
        Ok(())
    }

    fn increment(&mut self, incr: bool) {
        self.incremental = incr;
    }
}
