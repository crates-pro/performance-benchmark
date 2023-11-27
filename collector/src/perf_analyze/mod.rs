use std::{
    collections::HashMap,
    fs::{create_dir_all, read_dir},
    path::PathBuf,
};

use crate::{statistics::statistic::Statistics, toolchain::ResultWriter};

use self::{
    analyze::perf_analyze,
    data_process::{merge_perf_datas, split_data_by_metric},
};

use rayon::prelude::*;

mod analyze;
mod data_process;

#[derive(serde::Serialize, Default)]
struct CommitData {
    pub(self) commit_id: String,
    pub(self) benchmark_group_datas: Vec<BenchmarkGroupData>,
}

#[derive(serde::Serialize, Default)]
struct BenchmarkGroupData {
    pub(self) group_name: String,
    pub(self) benchmark_datas: Vec<BenchmarkPerfData>,
}

#[derive(serde::Serialize, Default)]
struct BenchmarkPerfData {
    pub(self) benchmark: String,
    pub(self) metric_data_map: HashMap<String, PerfData>,
}

#[derive(serde::Serialize, Default, Clone)]
struct PerfData {
    pub(self) metric: String,
    pub(self) symbol_statistics_map: HashMap<String, Statistics>,
    pub(self) symbol_data_map: HashMap<String, Vec<f64>>,
}

#[derive(serde::Serialize, Default, Clone)]
struct PerfRawData {
    pub(self) metric: String,
    pub(self) symbol_data_map: HashMap<String, f64>,
}

pub(crate) fn perf_analyzer(data_dir: &PathBuf, out_dir: &PathBuf, event_filter_file: &PathBuf) {
    // open data_dir
    let data_dir = match read_dir(data_dir) {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Fail to read data_dir {:?}. {}", data_dir, e);
            return;
        }
    };

    // get Vec[rustc_commit_dir, commit_data]
    let mut commit_datas = vec![];
    for entry in data_dir {
        match &entry {
            Ok(e) => {
                if e.path().is_dir() {
                    commit_datas.push(match analyze_commit_data(&e.path(), event_filter_file) {
                        Ok(data) => data,
                        Err(err) => {
                            eprintln!("Fail to get commit_data from {:?}. {}", e.path(), err);
                            CommitData::default()
                        }
                    });
                }
            }
            Err(e) => {
                eprintln!("Fail to read {:?} in data_dir. {}", entry, e);
                return;
            }
        };
    }

    // write data to out_dir
    match create_dir_all(out_dir) {
        Ok(_) => commit_datas
            .iter()
            .for_each(|data| match data.write_to(out_dir) {
                Ok(_) => println!(
                    "CommitData {} Saved to directory {:?}.",
                    data.commit_id, out_dir
                ),
                Err(e) => eprintln!("Fail to write data to {:?}. {}", out_dir, e),
            }),
        Err(e) => {
            eprintln!("Fail to create output dir {:?}. {}", out_dir, e);
        }
    };
}

fn analyze_commit_data(
    commit_data_dir: &PathBuf,
    event_filter_file: &PathBuf,
) -> anyhow::Result<CommitData> {
    let mut commit_data = CommitData::default();
    commit_data.commit_id = commit_data_dir
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let commit_data_dir = read_dir(commit_data_dir);
    for entry in commit_data_dir? {
        let entry = entry?;
        if entry.path().is_dir() {
            commit_data
                .benchmark_group_datas
                .push(analyze_benchmark_group_data(
                    &entry.path(),
                    event_filter_file,
                )?);
        }
    }

    Ok(commit_data)
}

fn analyze_benchmark_group_data(
    benchmark_group_dir: &PathBuf,
    event_filter_file: &PathBuf,
) -> anyhow::Result<BenchmarkGroupData> {
    let mut benchmark_group_data = BenchmarkGroupData::default();
    benchmark_group_data.group_name = benchmark_group_dir
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let benchmark_group_dir = read_dir(benchmark_group_dir);
    for entry in benchmark_group_dir? {
        let entry = entry?;
        if entry.path().is_dir() && !entry.path().to_str().unwrap().contains("flamegraph") {
            benchmark_group_data
                .benchmark_datas
                .push(analyze_benchmark_data(&entry.path(), event_filter_file)?);
        }
    }

    Ok(benchmark_group_data)
}

fn analyze_benchmark_data(
    benchmark_dir: &PathBuf,
    event_filter_file: &PathBuf,
) -> anyhow::Result<BenchmarkPerfData> {
    let benchmark_dir_reader = read_dir(benchmark_dir)?;

    let perf_datas: Vec<_> = benchmark_dir_reader
        .par_bridge()
        .filter_map(|entry| {
            let entry = match entry {
                Ok(e) => e,
                Err(e) => {
                    eprintln!("Fail to read files under {:?}. {}", benchmark_dir, e);
                    return None;
                }
            };
            if entry.path().is_file() && entry.path().to_str().unwrap().contains("perf.data") {
                match perf_analyze(&entry.path(), event_filter_file) {
                    Ok(data) => Some(split_data_by_metric(data)),
                    Err(e) => {
                        eprintln!("Fail to analyze data of {:?}. {}", entry.path(), e);
                        None
                    }
                }
            } else {
                None
            }
        })
        .collect();

    Ok(BenchmarkPerfData {
        benchmark: benchmark_dir
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
        metric_data_map: merge_perf_datas(perf_datas),
    })
}

impl CommitData {
    fn write_to(&self, out_dir: &PathBuf) -> anyhow::Result<()> {
        let mut writer = ResultWriter::new(
            out_dir.clone(),
            PathBuf::from(self.commit_id.clone() + ".json"),
        )?;

        writer.write(serde_json::to_string(&self)?)?;

        Ok(())
    }
}
