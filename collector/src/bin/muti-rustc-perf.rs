use std::{
    fs::{create_dir_all, File},
    io::{BufReader, BufWriter, Write},
    path::PathBuf,
    process::{Command, Output},
};

use clap::Parser;
use serde::Deserialize;

fn main() {
    let cli = Cli::parse();
    match cli.subcommand {
        Commands::MutiCompileTime {
            compilers_config,
            benchmark_dir,
            out,
            collector,
            iterations,
            flamegraph,
            cargo,
        } => {
            let compilers = match parse_compiler_json(&compilers_config) {
                Ok(compilers) => compilers,
                Err(err) => {
                    eprintln!("Fail to parse {:?}\n{}", compilers_config, err);
                    return;
                }
            };

            compilers.iter().for_each(|compiler| {
                compiler.paths.iter().for_each(|path| {
                    let path_string = String::from(path.to_str().unwrap());
                    let mut path_cycle = path_string.split('/').cycle();
                    let mut commit_id = path_cycle.next().unwrap();
                    let mut tmp = commit_id;
                    while tmp != "bin" {
                        commit_id = tmp;
                        tmp = path_cycle.next().unwrap();
                    }

                    let out = out.join(&compiler.name).join(commit_id);
                    match run_collector(
                        &collector,
                        &String::from("bench_local"),
                        &benchmark_dir,
                        &out,
                        iterations,
                        flamegraph,
                        &cargo,
                        &path,
                    ) {
                        Ok(output) => {
                            println!(
                                "Successfully benched rustc {:?} with benchmark_suit {:?}.",
                                compiler, benchmark_dir
                            );
                            match write_output(&out, &output) {
                                Ok(_) => (),
                                Err(err) => eprintln!("Fail to write output to {:?}\n{}", out, err),
                            };
                        }
                        Err(err) => eprintln!("Fail to run collector.\n{}", err),
                    }
                })
            });
        }
        Commands::MutiRuntime {
            compilers_config,
            benchmark_dir,
            out,
            collector,
            iterations,
            flamegraph,
            cargo,
        } => {
            let compilers = match parse_compiler_json(&compilers_config) {
                Ok(compilers) => compilers,
                Err(err) => {
                    eprintln!("Fail to parse {:?}\n{}", compilers_config, err);
                    return;
                }
            };

            compilers.iter().for_each(|compiler| {
                compiler.paths.iter().for_each(|path| {
                    let path_string = String::from(path.to_str().unwrap());
                    let mut path_cycle = path_string.split('/').into_iter();
                    let mut commit_id = path_cycle.next().unwrap();
                    let mut tmp = commit_id;
                    while tmp != "bin" {
                        commit_id = tmp;
                        tmp = path_cycle.next().unwrap();
                    }

                    let out = out.join(&compiler.name).join(commit_id);
                    match run_collector(
                        &collector,
                        &String::from("bench_runtime_local"),
                        &benchmark_dir,
                        &out,
                        iterations,
                        flamegraph,
                        &cargo,
                        &path,
                    ) {
                        Ok(output) => {
                            println!(
                                "Successfully benched rustc {:?} with benchmark_suit {:?}.",
                                compiler, benchmark_dir
                            );
                            match write_output(&out, &output) {
                                Ok(_) => (),
                                Err(err) => eprintln!("Fail to write output to {:?}\n{}", out, err),
                            };
                        }
                        Err(err) => eprintln!("Fail to run collector.\n{}", err),
                    }
                })
            });
        }
    }
}

fn parse_compiler_json(compilers_config: &PathBuf) -> anyhow::Result<Vec<RustCompiler>> {
    let fptr = File::open(compilers_config)?;
    let reader = BufReader::new(fptr);

    let compilers = serde_json::from_reader(reader)?;

    Ok(compilers)
}

fn write_output(dir: &PathBuf, output: &Output) -> anyhow::Result<()> {
    create_dir_all(dir)?;

    let fptr = File::create(dir.join("log.txt"))?;

    println!("log file {:?}", dir.join("log.txt"));

    let mut writer = BufWriter::new(fptr);

    writer.write("stdout = \n".as_bytes())?;
    writer.write_all(&output.stdout)?;

    writer.write("\n\nstderr = \n".as_bytes())?;
    writer.write_all(&output.stderr)?;

    Ok(())
}

fn run_collector(
    collector: &PathBuf,
    subcommand: &String,
    benchmark_dir: &PathBuf,
    out: &PathBuf,
    iterations: u32,
    flamegraph: u32,
    cargo: &PathBuf,
    rustc: &PathBuf,
) -> anyhow::Result<Output> {
    let mut cmd = Command::new(collector);
    cmd.arg(subcommand)
        .arg("--bench-dir")
        .arg(benchmark_dir)
        .arg("--out-dir")
        .arg(out)
        .arg("--iterations")
        .arg(iterations.to_string())
        .arg("--flamegraph")
        .arg(flamegraph.to_string())
        .arg("--cargo")
        .arg(cargo)
        .arg(rustc);

    match cmd.output() {
        Ok(output) => Ok(output),
        Err(err) => anyhow::Result::Err(err.into()),
    }
}

#[derive(Debug, clap::Parser)]
#[clap(about, version, author)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: Commands,
}

#[derive(Debug, clap::Subcommand)]
#[clap(rename_all = "snake_case")]
pub enum Commands {
    MutiCompileTime {
        #[clap(long = "compilers-config")]
        compilers_config: PathBuf,

        #[clap(long = "benchmark-dir")]
        benchmark_dir: PathBuf,

        #[clap(long = "out")]
        out: PathBuf,

        #[clap(long = "collector")]
        collector: PathBuf,

        #[clap(long = "iterations")]
        iterations: u32,

        #[clap(long = "flamegraph")]
        flamegraph: u32,

        #[clap(long = "cargo")]
        cargo: PathBuf,
    },
    MutiRuntime {
        #[clap(long = "compilers-config")]
        compilers_config: PathBuf,

        #[clap(long = "benchmark-dir")]
        benchmark_dir: PathBuf,

        #[clap(long = "out")]
        out: PathBuf,

        #[clap(long = "collector")]
        collector: PathBuf,

        #[clap(long = "iterations")]
        iterations: u32,

        #[clap(long = "flamegraph")]
        flamegraph: u32,

        #[clap(long = "cargo")]
        cargo: PathBuf,
    },
}

#[derive(Deserialize, Debug)]
struct RustCompiler {
    name: String,
    paths: Vec<PathBuf>,
}
