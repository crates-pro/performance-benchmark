use collector::benchmark::profile::Profile;
use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
#[clap(about, version, author)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, clap::Subcommand)]
#[clap(rename_all = "snake_case")]
pub enum Commands {
    /// Merge Data of the same rustc version from different benchmark groups
    /// and calculate their statistics.
    MergeStats {
        /// Path to root of Stats dir
        #[clap(long = "root-dir")]
        root_dir: PathBuf,

        /// `debug` or `release` or someother profiles of rustc.
        #[clap(long = "profile")]
        profile: Profile,

        /// Version of rustc
        #[clap(long = "rust-ver")]
        rustc: String,

        /// The path of output file
        #[clap(long = "out-path", default_value = "results")]
        out_path: PathBuf,
    },

    /// Compare 2 different stats and plot their change rate on geometric mean.
    StatsCompare {
        /// The first stats.
        #[clap(long = "stats-1")]
        stats_a: PathBuf,

        /// The second stats.
        #[clap(long = "stats-2")]
        stats_b: PathBuf,

        /// The metric that needs comparison.
        #[clap(long = "metric")]
        metric: String,

        /// The path of output file
        #[clap(long = "out-path", default_value = "results")]
        out_path: PathBuf,
    },
}
