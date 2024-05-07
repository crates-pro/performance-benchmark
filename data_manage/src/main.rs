use clap::Parser;
use commannds::Cli;
use compare_stats::{compare_stat::compare_stat, compare_stat_2d::compare_stat_2d};
use merge_stats::{merge_runtime_stat::merge_runtime_stats, merge_stat::merge_compile_time_stats};
use pca_analysis::entry::pca_entry;
use table_data::merge_metrics::{
    merge_metrics_from_compile_time_stats_to_table_data, merge_metrics_on_table_data,
};

mod commannds;
mod compare_stats;
mod merge_stats;
mod pca_analysis;
mod table_data;

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
        commannds::Commands::MergeTableMetrics {
            table_data_path,
            out_path,
            old_metrics,
            merged_metric,
        } => match merge_metrics_on_table_data(
            &table_data_path,
            &out_path,
            &old_metrics.split(',').map(|s| s.to_string()).collect(),
            &merged_metric,
        ) {
            Ok(p) => println!("Write merged table data to {}", p.to_str().unwrap()),
            Err(e) => eprintln!("{}", e),
        },
        commannds::Commands::MergeCompileTimeStatsToTable {
            table_data_path,
            stats_path,
            out_path,
            new_metrics,
        } => match merge_metrics_from_compile_time_stats_to_table_data(
            &table_data_path,
            &stats_path,
            out_path.as_path(),
            new_metrics.split(',').map(|s| s.to_string()).collect(),
        ) {
            Ok(p) => println!("Write merged table data to {}", p.to_str().unwrap()),
            Err(e) => eprintln!("{}", e),
        },
        commannds::Commands::PcaAnalysis {
            table_data_path,
            out_dir,
            max_component_num,
        } => match pca_entry(&table_data_path, out_dir, max_component_num) {
            Ok(p) => println!("Write Pca analysis result to {}", p.to_str().unwrap()),
            Err(e) => eprintln!("{}", e),
        },
    }
}
