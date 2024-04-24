use clap::Parser;
use commannds::Cli;
use compare_stat::compare_stat;
use merge_stat::merge_compile_time_stats;

mod commannds;
mod compare_stat;
mod merge_stat;
mod plotters;

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
        } => compare_stat(&stats_a, &stats_b, &metric, out_path),
    }
}
