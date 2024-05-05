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
    /// Merge compile-time data of the same rustc version from different benchmark groups
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

    /// Merge runtime data of the same rustc version from different benchmark groups
    /// and calculate their statistics.
    MergeRuntimeStats {
        /// Path to root of Stats dir
        #[clap(long = "root-dir")]
        root_dir: PathBuf,

        /// Version of rustc
        #[clap(long = "rust-ver")]
        rustc: String,

        /// The path of output file
        #[clap(long = "out-path", default_value = "results")]
        out_path: PathBuf,
    },

    /// Compare 2 different stats on one metric and plot their change rate on geometric mean.
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

    /// Compare 2 different stats on 2 metrics and plot their change rate on geometric mean.
    StatsCompare2d {
        /// The first stats.
        #[clap(long = "stats-1")]
        stats_a: PathBuf,

        /// The second stats.
        #[clap(long = "stats-2")]
        stats_b: PathBuf,

        /// The metric that needs comparison.
        #[clap(long = "metric-a")]
        metric_a: String,

        /// The metric that needs comparison.
        #[clap(long = "metric-b")]
        metric_b: String,

        /// The path of output file
        #[clap(long = "out-path", default_value = "results")]
        out_path: PathBuf,
    },

    /// Merge several metrics into a new metric of a table data fmt file.
    MergeTableMetrics {
        /// The path of table data fmt file.
        #[clap(long = "table-data")]
        table_data_path: PathBuf,
        /// The path of output file.
        #[clap(long = "out-path")]
        out_path: PathBuf,
        /// Metrics need to be merged.
        #[clap(long = "old-metrics")]
        old_metrics: Vec<String>,
        /// New metric merged from old-metrics.
        #[clap(long = "merged-metric")]
        merged_metric: String,
    },

    /// Merge compile-time stats into a table data fmt file.
    MergeCompileTimeStatsToTable {
        /// The path of table data fmt file.
        #[clap(long = "table-data")]
        table_data_path: PathBuf,
        /// The path of compile-time stats fmt file.
        #[clap(long = "stats")]
        stats_path: PathBuf,
        /// The path of output file.
        #[clap(long = "out-path")]
        out_path: PathBuf,
        /// Metrics merged from stats fmt file.
        #[clap(long = "new_metrics")]
        new_metrics: Vec<String>,
    },
}
