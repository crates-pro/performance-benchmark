use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};

use collector::{
    mir_analyze::data::table_data::TableDatas, statistics::compile_time_stat::CompileTimeStatistics,
};

pub fn merge_metrics_on_table_data(
    table_data_path: &Path,
    out_path: &Path,
    old_metrics: &Vec<String>,
    merged_metric: &String,
) -> anyhow::Result<PathBuf> {
    serde_json::to_writer(
        BufWriter::new(File::create(out_path)?),
        &merge_metrics(
            serde_json::from_reader(BufReader::new(File::open(table_data_path)?))?,
            old_metrics,
            merged_metric,
        ),
    )?;

    Ok(out_path.to_path_buf())
}

pub fn merge_metrics_from_compile_time_stats_to_table_data(
    table_data_path: &Path,
    stats_path: &Path,
    out_path: &Path,
    new_metrics: Vec<String>,
) -> anyhow::Result<PathBuf> {
    serde_json::to_writer(
        BufWriter::new(File::create(out_path)?),
        &merge_metrics_from_compile_time_stats(
            serde_json::from_reader(BufReader::new(File::open(table_data_path)?))?,
            serde_json::from_reader(BufReader::new(File::open(stats_path)?))?,
            new_metrics,
        ),
    )?;

    Ok(out_path.to_path_buf())
}

fn merge_metrics(
    data: TableDatas<String, String, f64>,
    old_metrics: &Vec<String>,
    merged_metric: &String,
) -> TableDatas<String, String, f64> {
    let mut merged_metrics = TableDatas::new();

    data.into_iter().for_each(|(m, d)| {
        if old_metrics.contains(&m) {
            match merged_metrics.get_mut(merged_metric) {
                Some(v) => {
                    v.iter_mut().for_each(|(k, v)| {
                        *v += d.get(k).unwrap();
                    });
                }
                None => {
                    merged_metrics.insert(merged_metric.clone(), d);
                }
            }
        } else {
            merged_metrics.insert(m, d);
        }
    });
    merged_metrics
}

fn merge_metrics_from_compile_time_stats(
    mut data: TableDatas<String, String, f64>,
    stats: CompileTimeStatistics,
    new_metrics: Vec<String>,
) -> TableDatas<String, String, f64> {
    let stats: HashMap<String, HashMap<String, _>> = stats
        .into_iter()
        .map(|s| {
            (
                s.name,
                s.statistic_vec
                    .into_iter()
                    .map(|(name, stats)| (name, stats))
                    .collect(),
            )
        })
        .collect();

    new_metrics.into_iter().for_each(|m| {
        data.insert(
            m.clone(),
            stats
                .iter()
                .map(|(b, stats_map)| (b.clone(), stats_map.get(&m).unwrap().geometric_mean))
                .collect(),
        );
    });

    data
}

#[cfg(test)]
mod test_merge_metrics {
    use std::{
        fs::{remove_file, File},
        io::BufReader,
        path::PathBuf,
    };

    use collector::mir_analyze::data::table_data::TableDatas;

    use crate::table_data::merge_metrics::merge_metrics_from_compile_time_stats_to_table_data;

    use super::merge_metrics_on_table_data;

    #[test]
    fn test_merge_metrics_on_table_data() {
        let table_data_path = PathBuf::from("test/table_data/data/mir-analysis.json");
        let out_data_path = PathBuf::from("test/table_data/merged_metrics.json");
        let old_metrics = vec![
            String::from("oop_noc"),
            String::from("oop_pbf"),
            String::from("oop_dfc"),
        ];
        let merged_metric = String::from("oop_pattern");

        assert_eq!(
            merge_metrics_on_table_data(
                table_data_path.as_path(),
                out_data_path.as_path(),
                &old_metrics,
                &merged_metric
            )
            .unwrap(),
            out_data_path
        );

        let merged_data: TableDatas<String, String, f64> =
            serde_json::from_reader(BufReader::new(File::open(&out_data_path).unwrap())).unwrap();
        assert!(merged_data.contains_key(&merged_metric));
        old_metrics.iter().for_each(|m| {
            assert!(!merged_data.contains_key(m));
        });

        remove_file(out_data_path).unwrap();
    }

    #[test]
    fn test_merge_metrics_from_compile_time_stats_to_table_data() {
        let table_data_path = PathBuf::from("test/table_data/data/mir-analysis.json");
        let stats_path = PathBuf::from("test/table_data/data/merged_statstics_current.json");
        let out_data_path = PathBuf::from("test/table_data/merged_stats_metrics.json");
        let new_metrics = vec![
            String::from("instructions:u"),
            String::from("branch-misses"),
            String::from("cache-misses"),
        ];

        assert_eq!(
            merge_metrics_from_compile_time_stats_to_table_data(
                table_data_path.as_path(),
                stats_path.as_path(),
                out_data_path.as_path(),
                new_metrics.clone()
            )
            .unwrap(),
            out_data_path
        );

        let merged_data: TableDatas<String, String, f64> =
            serde_json::from_reader(BufReader::new(File::open(&out_data_path).unwrap())).unwrap();
        new_metrics.iter().for_each(|m| {
            assert!(merged_data.contains_key(m));
        });

        remove_file(out_data_path).unwrap();
    }
}
