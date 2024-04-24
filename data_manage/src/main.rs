use clap::Parser;
use commannds::Cli;
use merge_stat::merge_compile_time_stats;

mod commannds;
mod merge_stat;

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
    }
}
