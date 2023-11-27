use core::panic;
use std::{
    collections::HashMap,
    fs::{copy, File},
    io::{BufRead, BufReader, BufWriter},
    path::{Path, PathBuf},
    process::Command,
};

use tempfile::TempDir;

pub fn perf_analyze(perf_data: &Path, event_filter_file: &Path) -> anyhow::Result<EventCostMap> {
    let tmp_dir = TempDir::new()?;

    copy(perf_data, tmp_dir.path().join("perf.data"))?;

    let mut cmd = Command::new("perf");
    cmd.current_dir(tmp_dir.path());
    cmd.arg("script");

    let event_filters = parse_event_filters(event_filter_file);

    let tmp_file = PathBuf::from("/media/workstation/cc36671e-05f5-48bd-9b40-1b1c1f396fae/home/fxl/Rust_Performance_Benchmark/out/record/perf.analyze");
    let data = analyze_reader(&tmp_file, event_filters)?;

    std::fs::copy(tmp_file.parent().unwrap().join("analyze_data.json"), PathBuf::from("/media/workstation/cc36671e-05f5-48bd-9b40-1b1c1f396fae/home/fxl/Rust_Performance_Benchmark/out/record/data.json")).unwrap();

    Ok(data)
}

fn analyze_reader(
    analyze_file: &PathBuf,
    event_filters: PerfEventFilters,
) -> anyhow::Result<EventCostMap> {
    let fptr = File::open(analyze_file)?;
    let reader = BufReader::new(fptr);

    let mut event_cost_map = EventCostMap::new();

    let mut symbol_info = String::new();
    let mut metric = String::new();
    let mut cost = String::new();

    reader.lines().for_each(|line| {
        let line = match line {
            Ok(line) => line,
            Err(err) => panic!(
                "Fail to read analyzed perf_data file {:?}. {}",
                analyze_file, err
            ),
        };

        let splits: Vec<&str> = line.split(' ').collect();

        if line.starts_with(" ") {
            symbol_info = String::from(splits[2]);
            let cost = match cost.parse::<i128>() {
                Ok(i) => i,
                Err(e) => {
                    eprintln!("Fail to parse {} to i128. {}", cost, e);
                    return;
                }
            };

            eprintln!(
                "cost={:?}, metric={:?}, symbol={:?}",
                cost, metric, symbol_info
            );
            if event_filters.len() > 0 {
                event_filters.iter().for_each(|event| {
                    add_cost(&mut event_cost_map, event, &metric, cost);
                });
            } else {
                add_cost(&mut event_cost_map, &symbol_info, &metric, cost);
            }
        } else {
            parse_perf_script_line_split(splits, &mut metric, &mut cost, &mut symbol_info);
        }
    });

    let file = File::create(analyze_file.parent().unwrap().join("analyze_data.json"))?;

    let writer = BufWriter::new(file);
    serde_json::to_writer::<BufWriter<_>, EventCostMap>(writer, &event_cost_map)?;
    Ok(event_cost_map)
}

fn parse_event_filters(filter_file: &Path) -> PerfEventFilters {
    if !filter_file.exists() {
        return PerfEventFilters::default();
    }
    let fptr = match File::open(filter_file) {
        Ok(file) => file,
        Err(err) => panic!("Fail to open event_filters file {:?}. {}", filter_file, err),
    };
    let reader = BufReader::new(fptr);

    match serde_json::from_reader(reader) {
        Ok(s) => s,
        Err(err) => panic!(
            "Fail to parse event_filters file {:?}. {}",
            filter_file, err
        ),
    }
}

fn add_cost(event_cost_map: &mut EventCostMap, symbol: &String, metric: &String, cost: i128) {
    let mut updated = false;
    event_cost_map.iter_mut().for_each(|((key, m), value)| {
        if symbol.contains(key) && metric.contains(m) {
            *value += cost as f64;
            updated = true;
        }
    });
    if updated == false {
        event_cost_map.insert((symbol.clone(), metric.clone()), cost as f64);
    }
}

fn parse_perf_script_line_split(
    line_split: Vec<&str>,
    metric: &mut String,
    cost: &mut String,
    symbol_info: &mut String,
) {
    let mut metric_idx = -1;
    let mut i = 0;
    line_split.iter().for_each(|s| {
        if metric_idx > 0 && s.len() > 0 {
            *symbol_info = format!("{} {}", symbol_info, s);
        }

        if s.contains("instructions:u")
            || s.contains("cycles:u")
            || s.contains("cache-misses:u")
            || s.contains("branch-misses:u")
            || s.contains("max-rss:u")
        {
            metric_idx = i as i32;
            *metric = s.to_string();
        }
        i += 1;
    });

    let mut i = 0;
    line_split.iter().for_each(|s| {
        if i < metric_idx {
            if s.len() > 0 {
                *cost = s.to_string();
            }
            i += 1;
        }
    });
}

type PerfEventFilters = Vec<String>;

/// (Symbol name, metric) -> cost
pub(super) type EventCostMap = HashMap<(String, String), f64>;
