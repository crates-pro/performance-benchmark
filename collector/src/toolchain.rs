use std::{
    fmt::Display,
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
};

use anyhow::{Context, Ok};
use log::debug;

use crate::benchmark::profile::{Profile, Profiles};

#[derive(Debug, Copy, Clone)]
pub struct Compiler<'a> {
    pub rustc: &'a Path,
    pub cargo: &'a Path,
    pub is_nightly: bool,
}

impl<'a> Compiler<'a> {
    pub fn from_toolchain(tc: &'a LocalToolchain) -> Compiler<'a> {
        Compiler {
            rustc: &tc.rustc,
            cargo: &tc.cargo,
            is_nightly: true,
        }
    }
}

#[derive(Debug)]
pub struct LocalToolchain {
    pub rustc: PathBuf,
    pub cargo: PathBuf,
    pub flame_graph: PathBuf,
    pub id: String,
}

impl Display for LocalToolchain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rustc_s = String::from("rustc: ") + self.rustc.to_str().unwrap();
        let cargo_s = String::from("cargo: ") + self.cargo.to_str().unwrap();
        f.write_str((rustc_s + "\n" + &cargo_s).as_str())?;
        std::fmt::Result::Ok(())
    }
}

/// Get a toolchain from the input.
/// - `rustc`: check if the given one is acceptable.
/// - `rustdoc`: if one is given, check if it is acceptable. Otherwise, if
///   the `Doc` profile is requested, look for one next to the given `rustc`.
/// - `cargo`: if one is given, check if it is acceptable. Otherwise, look
///   for the nightly Cargo via `rustup`.
pub fn get_local_toolchain(
    rustc: &str,
    cargo: Option<&Path>,
    id: Option<&str>,
    id_suffix: &str,
) -> anyhow::Result<LocalToolchain> {
    // `+`-prefixed rustc is an indicator to fetch the rustc of the toolchain
    // specified. This follows the similar pattern used by rustup's binaries
    // (e.g., `rustc +stage1`).
    let (rustc, id) = if let Some(toolchain) = rustc.strip_prefix('+') {
        let output = Command::new("rustup")
            .args(&["which", "rustc", "--toolchain", &toolchain])
            .output()
            .context("failed to run `rustup which rustc`")?;

        // Looks like a commit hash? Try to install it...
        if !output.status.success() && toolchain.len() == 40 {
            // No such toolchain exists, so let's try to install it with
            // rustup-toolchain-install-master.

            if Command::new("rustup-toolchain-install-master")
                .arg("-V")
                .output()
                .is_err()
            {
                anyhow::bail!("rustup-toolchain-install-master is not installed but must be");
            }

            if !Command::new("rustup-toolchain-install-master")
                .arg(&toolchain)
                .status()
                .context("failed to run `rustup-toolchain-install-master`")?
                .success()
            {
                anyhow::bail!(
                    "commit-like toolchain {} did not install successfully",
                    toolchain
                )
            }
        }

        let output = Command::new("rustup")
            .args(&["which", "rustc", "--toolchain", &toolchain])
            .output()
            .context("failed to run `rustup which rustc`")?;

        if !output.status.success() {
            anyhow::bail!("did not manage to obtain toolchain {}", toolchain);
        }

        let s = String::from_utf8(output.stdout)
            .context("failed to convert `rustup which rustc` output to utf8")?;

        let rustc = PathBuf::from(s.trim());
        debug!("found rustc: {:?}", &rustc);

        // When the id comes from a +toolchain, the suffix is *not* added.
        let id = if let Some(id) = id {
            let mut id = id.to_owned();
            id.push_str(id_suffix);
            id
        } else {
            toolchain.to_owned()
        };
        (rustc, id)
    } else {
        let rustc = PathBuf::from(rustc)
            .canonicalize()
            .with_context(|| format!("failed to canonicalize rustc executable {:?}", rustc))?;

        // When specifying rustc via a path, the suffix is always added to the
        // id.
        let mut id = if let Some(id) = id {
            id.to_owned()
        } else {
            "Id".to_string()
        };
        id.push_str(id_suffix);

        (rustc, id)
    };

    let cargo = if let Some(cargo) = &cargo {
        cargo
            .canonicalize()
            .with_context(|| format!("failed to canonicalize cargo executable {:?}", cargo))?
    } else {
        // Use the nightly cargo from `rustup`.
        let output = Command::new("rustup")
            .args(&["which", "cargo", "--toolchain=nightly"])
            .output()
            .context("failed to run `rustup which cargo --toolchain=nightly`")?;
        if !output.status.success() {
            anyhow::bail!(
                "`rustup which cargo --toolchain=nightly` exited with status {}\nstderr={}",
                output.status,
                String::from_utf8_lossy(&output.stderr)
            )
        }
        let s = String::from_utf8(output.stdout)
            .context("failed to convert `rustup which cargo --toolchain=nightly` output to utf8")?;

        let cargo = PathBuf::from(s.trim());
        debug!("found cargo: {:?}", &cargo);
        cargo
    };

    let flame_graph = {
        let output = Command::new("which")
            .arg("flamegraph")
            .output()
            .context("failed to run `which flamegraph")?;
        let s = String::from_utf8(output.stdout)
            .context("failed to convert `which flamegraph` output to utf8")?;

        PathBuf::from(s.trim())
    };

    Ok(LocalToolchain {
        rustc,
        cargo,
        flame_graph,
        id,
    })
}

#[derive(Debug, clap::Parser)]
#[clap(about, version, author)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, clap::Args)]
pub struct LocalOptions {
    /// The path to the local rustc to measure
    // Not a `PathBuf` because it can be a file path *or* a `+`-prefixed
    // toolchain name, and `PathBuf` doesn't work well for the latter.
    pub rustc: String,

    /// Identifier to associate benchmark results with
    #[clap(long)]
    pub id: Option<String>,

    /// The path to the local Cargo to use
    #[clap(long, parse(from_os_str))]
    pub cargo: Option<PathBuf>,
    // /// Exclude all benchmarks matching a prefix in this comma-separated list
    // #[clap(long)]
    // pub exclude: Option<String>,

    // /// Include only benchmarks matching a prefix in this comma-separated list
    // #[clap(long)]
    // pub include: Option<String>,
}

#[derive(Debug, clap::Subcommand)]
#[clap(rename_all = "snake_case")]
pub enum Commands {
    /// Benchmarks the performance of a local rustc.
    BenchLocal {
        #[clap(flatten)]
        local: LocalOptions,

        /// The number of iterations to do for each benchmark
        #[clap(long, default_value = "1")]
        iterations: usize,

        #[clap(long = "perf-tool", default_value = "perf-stat")]
        perf_tool: PerfTool,

        #[clap(long = "event-filter-file", default_value = "")]
        event_filter_file: PathBuf,

        #[clap(long = "profile", default_value = "debug,release")]
        profiles: Profiles,

        /// The path of benchmark dir
        #[clap(long = "bench-dir", default_value = "../benchmarks/compile-time")]
        bench_dir: PathBuf,

        /// The path of output file
        #[clap(long = "out-dir", default_value = "results")]
        out_dir: PathBuf,

        /// Flag of Flamegraph. Make $flamegraph$ > 0 if flamegraph is needed.
        #[clap(long, default_value = "1")]
        flamegraph: i32,
    },
    /// Benchmarks the performance of programs generated by a local rustc.
    BenchRuntimeLocal {
        #[clap(flatten)]
        local: LocalOptions,

        /// How many iterations of each benchmark should be executed.
        #[clap(long, default_value = "5")]
        iterations: u32,

        #[clap(long = "perf-tool", default_value = "perf-stat")]
        perf_tool: PerfTool,

        #[clap(long = "event-filter-file", default_value = "")]
        event_filter_file: PathBuf,

        /// The path of benchmark dir
        #[clap(long = "bench-dir", default_value = "../benchmarks/runtime/")]
        bench_dir: PathBuf,

        /// The path of output file
        #[clap(long = "out-dir", default_value = "results")]
        out_dir: PathBuf,

        /// Flag of Flamegraph. Make $flamegraph$ > 0 if flamegraph is needed.
        #[clap(long, default_value = "1")]
        flamegraph: i32,
    },

    /// Generate MIR with a local rustc.
    GenerateMir {
        #[clap(flatten)]
        local: LocalOptions,

        /// The path of benchmark dir
        #[clap(long = "bench-dir", default_value = "../benchmarks/runtime/")]
        bench_dir: PathBuf,

        /// The path of output mirs
        #[clap(long = "out-dir", default_value = "results")]
        out_dir: PathBuf,
    },

    /// Benchmarks the binary size of compiled benchmarks for a local rustc.
    BinaryLocal {
        #[clap(flatten)]
        local: LocalOptions,

        #[clap(long = "profile", default_value = "debug,release")]
        profiles: Profiles,

        /// The path of benchmark dir
        #[clap(long = "bench-dir", default_value = "../benchmarks/compile-time")]
        bench_dir: PathBuf,

        /// The path of output file
        #[clap(long = "out-dir", default_value = "results")]
        out_dir: PathBuf,
    },

    /// Boxplot and scatterplot for a pair of compiled_binary_size data for comparsion.
    BinaryPlot {
        /// The path of compiled_binary_size data 1.
        #[clap(long = "data1")]
        data1: PathBuf,
        /// The path of compiled_binary_size data 2.
        #[clap(long = "data2")]
        data2: PathBuf,
        /// The label of compiled_binary_size data 1. Deprecated when mode is `cmp`.
        #[clap(long = "data1-label", default_value = "")]
        data1_label: String,
        /// The label of compiled_binary_size data 2. Deprecated when mode is `cmp`.
        #[clap(long = "data2-label", default_value = "")]
        data2_label: String,
        /// The profile of data collected.
        #[clap(long = "profile")]
        profile: Profile,
        /// The path of output figure.
        #[clap(long = "out-path")]
        out_path: PathBuf,
        /// The mode of plotter. Either be `default` or be `cmp`(compare).
        #[clap(long = "mode", default_value = "default")]
        mode: BinaryPlotMode,
    },

    /// Trasfer Json outpu to csv output.
    TransferCsvOutput {
        /// origin json output directory
        #[clap(long = "in-dir")]
        in_dir: PathBuf,

        /// output directory
        // #[clap(long = "out-dir")]
        // out_dir: PathBuf,

        /// type = 'compile_time' or 'runtime'
        #[clap(long = "type")]
        ty: String,

        #[clap(long = "profile", default_value = "debug,release")]
        profiles: Profiles,
    },

    /// Analyze perf.data from a local directory.
    AnalyzeLocal {
        /// The path of dir contains perf.data.
        #[clap(long = "data-dir", default_value = "../perf_analyze")]
        data_dir: PathBuf,

        /// The path of output dir.
        #[clap(long = "out-dir", default_value = "results")]
        out_dir: PathBuf,

        /// The path of event-filter file.
        #[clap(long = "event-filter-file", default_value = "event-filter-file.js")]
        event_filter_file: PathBuf,
    },

    /// Get specific morphemes in the benchmark programs.
    MineMorpheme {
        /// The path of benchmark dir
        #[clap(long = "bench-dir", default_value = "../benchmarks/compile-time")]
        bench_dir: PathBuf,

        /// The path of output file
        #[clap(long = "out", default_value = "morphemes.csv")]
        out_path: PathBuf,
    },

    /// Analyze MIRs generated from benchmarks.
    MirAnalyze {
        /// The path of dir contains mir files grouped by benchmark name
        #[clap(long = "mir-dir", default_value = "../benchmarks/compile-time")]
        mir_dir: PathBuf,

        /// The path of output file
        #[clap(long = "out-path", default_value = "results")]
        out_path: PathBuf,
    },
}

#[derive(Debug)]
pub enum PerfTool {
    BenchTool(Bencher),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Bencher {
    PerfStat,
    PerfRecord,
    // PerfStatSelfProfile,
    // XperfStat,
    // XperfStatSelfProfile,
}

impl PerfTool {
    pub fn name(&self) -> String {
        match self {
            PerfTool::BenchTool(b) => format!("{:?}", b),
        }
    }

    pub fn get_bencher(&self) -> Bencher {
        match self {
            PerfTool::BenchTool(bencher) => bencher.clone(),
        }
    }
}

impl FromStr for PerfTool {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "perf-stat" => std::result::Result::Ok(PerfTool::BenchTool(Bencher::PerfStat)),
            "perf-record" => std::result::Result::Ok(PerfTool::BenchTool(Bencher::PerfRecord)),
            _ => Err(format!("Unrecognized PerfTool {}", s)),
        }
    }
}

#[derive(Debug)]
pub enum BinaryPlotMode {
    DefaultMode,
    CompareMode,
}

impl FromStr for BinaryPlotMode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "default" => std::result::Result::Ok(Self::DefaultMode),
            "cmp" => std::result::Result::Ok(Self::CompareMode),
            _ => Err(format!("Unrecognized BinaryPlotMode {}. BinaryPlotMode should be either `default` or `cmp`", s)),
        }
    }
}

pub struct ResultWriter {
    fptr: File,
}

impl ResultWriter {
    pub fn new(dir: PathBuf, file_name: PathBuf) -> anyhow::Result<Self> {
        create_dir_all(&dir).with_context(|| format!("fail to create dir for {:?}", dir))?;
        let rw = ResultWriter {
            fptr: File::create(&dir.join(file_name))
                .with_context(|| format!("Fail to create output file {:?}.", dir))?,
        };
        Ok(rw)
    }

    pub fn write(&mut self, buf: String) -> anyhow::Result<&Self> {
        self.fptr
            .write(buf.as_bytes())
            .with_context(|| format!("fail to write file {:?}.", self.fptr))?;
        Ok(self)
    }
}
