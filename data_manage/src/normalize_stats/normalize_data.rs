use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use collector::statistics::{
    compile_time_stat::{CompileTimeResultSet, CompileTimeStatistics},
    statistic::Statistics,
};

pub fn normalize_stat(stats: &PathBuf, out_path: PathBuf) -> anyhow::Result<PathBuf> {
    let normalized_stat = normalize_compile_time_stat(
        &serde_json::from_reader(BufReader::new(File::open(&stats)?))?,
        &"wall-time".to_string(),
    );
    serde_json::to_writer(BufWriter::new(File::create(&out_path)?), &normalized_stat)?;

    Ok(out_path)
}

pub fn normalize_compile_time_stat(
    stat: &CompileTimeStatistics,
    normalize_metric: &String,
) -> CompileTimeStatistics {
    let mut normalized_stat = stat.clone();
    normalized_stat.iter_mut().for_each(|stat| {
        let n = stat
            .statistic_vec
            .iter()
            .map(|(x, y)| (x, y))
            .collect::<HashMap<&String, &Statistics>>()
            .get(normalize_metric)
            .unwrap()
            .geometric_mean;

        stat.statistic_vec.iter_mut().for_each(|(m, s)| {
            if m != normalize_metric {
                s.geometric_mean /= n;
            }
        });
    });

    normalized_stat
}

pub fn normalize_compile_time_data(
    data: &CompileTimeResultSet,
    normalize_metric: &String,
) -> CompileTimeResultSet {
    let mut normalized_data = data.clone();
    normalized_data.results.iter_mut().for_each(|x| {
        x.result_vec.iter_mut().for_each(|d| {
            let n = *d.stats.stats.get(normalize_metric).unwrap();
            d.stats.stats.iter_mut().for_each(|(m, v)| {
                if m != normalize_metric {
                    *v /= n;
                }
            })
        })
    });

    normalized_data
}
