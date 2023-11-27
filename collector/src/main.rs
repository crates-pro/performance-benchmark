use std::{
    env::current_dir,
    fs::create_dir_all,
    path::PathBuf,
    process::{self, Command},
};

use anyhow::{Context, Ok};
use benchmark::scenario::Scenario;
use clap::Parser;
use compile_time::bench_compile_time;
use runtime::bench_runtime;
use toolchain::{Cli, Commands, ResultWriter};

use crate::{
    compile_time::result::CompileTimeResultSet, csv_transfer::sheduler, morpheme_miner::run_miners,
    perf_analyze::perf_analyzer, toolchain::get_local_toolchain,
};

mod benchmark;
mod compile_time;
mod csv_transfer;
mod execute;
mod mir_analyze;
mod morpheme_miner;
mod perf_analyze;
mod runtime;
mod statistics;
mod toolchain;
mod utils;

fn main() {
    match main_result() {
        core::result::Result::Ok(code) => {
            process::exit(code);
        }
        Err(err) => {
            eprintln!("collector error: {:?}", err);
            process::exit(1);
        }
    }
}

fn main_result() -> anyhow::Result<i32> {
    env_logger::init();
    let args = Cli::parse();

    let mut cmd = Command::new("perf");
    let has_perf = cmd.output().is_ok();
    assert!(has_perf);

    let mut cmd = Command::new("flamegraph");
    let has_flamegraph = cmd.output().is_ok();
    assert!(has_flamegraph);

    match args.command {
        Commands::BenchLocal {
            local,
            iterations,
            perf_tool,
            event_filter_file,
            bench_dir,
            profiles,
            out_dir,
            flamegraph,
        } => {
            eprintln!("profiles: {:?}", profiles.profiles);
            let default_scenarios = vec![Scenario::Full];
            let toolch = get_local_toolchain(
                &local.rustc,
                local.cargo.as_deref(),
                local.id.as_deref(),
                "",
            )?;

            println!("{}", toolch);

            create_output_dir(&out_dir)?;

            let flame_graph_dir = out_dir.join("flamegraphs");
            create_output_dir(&flame_graph_dir)?;

            let cwd = current_dir()?;
            let flame_graph_path = cwd.join(flame_graph_dir);
            log::debug!("{:?}", flame_graph_path);

            let mut result_writer =
                ResultWriter::new(out_dir.clone(), PathBuf::from("compile_time_results.json"))
                    .with_context(|| {
                        format!(
                            "Fail to open {} to record results!",
                            out_dir.to_str().unwrap_or("?")
                        )
                    })?;

            let mut statistics_writer = ResultWriter::new(
                out_dir.clone(),
                PathBuf::from("compile_time_statistics.json"),
            )
            .with_context(|| {
                format!(
                    "Fail to open {} to record statistics!",
                    out_dir.to_str().unwrap_or("?")
                )
            })?;

            let results = bench_compile_time(
                &toolch,
                &perf_tool,
                &event_filter_file,
                &profiles.profiles,
                &default_scenarios,
                bench_dir,
                iterations,
                flame_graph_path.as_path().as_ref(),
                flamegraph,
                out_dir.as_path(),
            )?;

            match perf_tool.get_bencher() {
                toolchain::Bencher::PerfStat => {
                    let result_set =
                        CompileTimeResultSet::new(local.id.unwrap_or_default(), results);
                    result_writer.write(serde_json::to_string(&result_set)?)?;

                    let result_statistics = result_set.calculate_statistics();
                    statistics_writer.write(serde_json::to_string(&result_statistics)?)?;
                }
                toolchain::Bencher::PerfRecord => (),
            }

            Ok(0)
        }
        Commands::BenchRuntimeLocal {
            local,
            iterations,
            perf_tool,
            event_filter_file,
            bench_dir,
            out_dir,
            flamegraph,
        } => {
            let ltc = get_local_toolchain(
                &local.rustc,
                local.cargo.as_deref(),
                local.id.as_deref(),
                "",
            )?;

            create_output_dir(&out_dir)?;

            let flame_graph_dir = out_dir.join("flamegraphs");
            create_output_dir(&flame_graph_dir)?;

            let cwd = current_dir()?;
            let flame_graph_path = cwd.join(flame_graph_dir);
            log::debug!("{:?}", flame_graph_path);

            let mut result_writer =
                ResultWriter::new(out_dir.clone(), PathBuf::from("runtime_results.json"))
                    .with_context(|| {
                        format!(
                            "Fail to open {} to record results!",
                            out_dir.to_str().unwrap_or("?")
                        )
                    })?;

            let mut statistics_writer =
                ResultWriter::new(out_dir.clone(), PathBuf::from("runtime_statistics.json"))
                    .with_context(|| {
                        format!(
                            "Fail to open {} to record statistics!",
                            out_dir.to_str().unwrap_or("?")
                        )
                    })?;

            let results = bench_runtime(
                &ltc,
                bench_dir,
                iterations,
                &perf_tool,
                &event_filter_file,
                &flame_graph_path,
                flamegraph,
                &out_dir,
            )?;

            result_writer.write(serde_json::to_string(&results)?)?;

            let statistics = results.calculate_statistics();
            statistics_writer.write(serde_json::to_string(&statistics)?)?;
            Ok(0)
        }
        Commands::TransferCsvOutput {
            in_dir,
            ty,
            profiles,
        } => {
            match ty.as_str() {
                "compile_time" => sheduler(&in_dir, &ty, &profiles),
                "runtime" => sheduler(&in_dir, &ty, &profiles),
                _ => eprintln!(
                    "Unknown type of data '{}'.\nSupported ty value: 'compile_time' or 'runtime'.",
                    ty
                ),
            }
            Ok(0)
        }
        Commands::AnalyzeLocal {
            data_dir,
            out_dir,
            event_filter_file,
        } => {
            perf_analyzer(&data_dir, &out_dir, &event_filter_file);
            Ok(0)
        }
        Commands::MineMorpheme {
            bench_dir,
            out_path,
        } => {
            run_miners(bench_dir, out_path);
            Ok(0)
        }
    }
}

fn create_output_dir(out_dir: &PathBuf) -> anyhow::Result<()> {
    if !out_dir.exists() {
        create_dir_all(&out_dir)
            .with_context(|| format!("Fail to create output dir {:?}", &out_dir))?;
    }
    assert!(out_dir.is_dir());

    Ok(())
}
