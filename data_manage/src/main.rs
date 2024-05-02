use clap::Parser;
use commannds::Cli;
use compare_stats::{compare_stat::compare_stat, compare_stat_2d::compare_stat_2d};
use merge_stats::{merge_runtime_stat::merge_runtime_stats, merge_stat::merge_compile_time_stats};

mod commannds;
mod compare_stats;
mod merge_stats;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        commannds::Commands::MergeStats {
            root_dir,
            profile,
            rustc,
            out_path,
        } => match merge_compile_time_stats(&root_dir, profile, rustc, out_path) {
            Ok(p) => println!("Write merged stats to {}", p.to_str().unwrap()),
            Err(e) => eprintln!("{}", e),
        },
        commannds::Commands::StatsCompare {
            stats_a,
            stats_b,
            out_path,
            metric,
        } => match compare_stat(&stats_a, &stats_b, &metric, out_path) {
            Ok(p) => println!("Plot change rate of stats to {}", p.to_str().unwrap()),
            Err(e) => eprintln!("{}", e),
        },
        commannds::Commands::StatsCompare2d {
            stats_a,
            stats_b,
            metric_a,
            metric_b,
            out_path,
        } => match compare_stat_2d(&stats_a, &stats_b, &metric_a, &metric_b, out_path) {
            Ok(p) => println!("Plot change rate of stats to {}", p.to_str().unwrap()),
            Err(e) => eprintln!("{}", e),
        },
        commannds::Commands::MergeRuntimeStats {
            root_dir,
            rustc,
            out_path,
        } => match merge_runtime_stats(&root_dir, rustc, out_path) {
            Ok(p) => println!("Write merged stats to {}", p.to_str().unwrap()),
            Err(e) => eprintln!("{}", e),
        },
    }
}
