use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf};

use anyhow::bail;
use collector::statistics::{
    compile_time_stat::CompileTimeStatistics, runtime_stat::RuntimeStatistics,
    statistic::Statistics,
};

pub type LabeledStats = HashMap<String, Statistics>;
pub type ChangeRate = HashMap<String, f64>;

pub fn read_stats(stats_file: &PathBuf, metric: &String) -> anyhow::Result<LabeledStats> {
    // Filter out non-relevant metrics and reshape stats into Hashmap

    match serde_json::from_reader::<_, CompileTimeStatistics>(BufReader::new(File::open(
        stats_file,
    )?)) {
        Ok(s) => return Ok(reshape_stat(s, metric)),
        Err(_) => (),
    }

    match serde_json::from_reader::<_, RuntimeStatistics>(BufReader::new(File::open(stats_file)?)) {
        Ok(s) => return Ok(reshape_runtime_stat(s, metric)),
        Err(e) => bail!(e),
    }
}

pub fn calculate_change_rate(stats_a: &LabeledStats, stats_b: &LabeledStats) -> ChangeRate {
    stats_a
        .into_iter()
        .map(|(k, v)| {
            let geom_a = v.geometric_mean;
            let geom_b = stats_b.get(k).unwrap().geometric_mean;

            (k.clone(), (geom_a - geom_b) / geom_b * 100.)
        })
        .collect()
}

fn reshape_stat(stats: CompileTimeStatistics, metric: &String) -> HashMap<String, Statistics> {
    stats
        .into_iter()
        .map(|s| {
            (
                s.name,
                s.statistic_vec
                    .into_iter()
                    .find(|(m, _)| m == metric)
                    .unwrap()
                    .1,
            )
        })
        .collect()
}

fn reshape_runtime_stat(stats: RuntimeStatistics, metric: &String) -> HashMap<String, Statistics> {
    stats
        .into_iter()
        .map(|s| {
            (
                s.name,
                s.statistic_vec
                    .into_iter()
                    .find(|(m, _)| m == metric)
                    .unwrap()
                    .1,
            )
        })
        .collect()
}
