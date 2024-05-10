use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf};

use anyhow::bail;
use collector::statistics::{
    compile_time_stat::CompileTimeResultSet, runtime_stat::RuntimeResultVec,
};

pub type LabeledData = HashMap<String, Vec<f64>>;
pub type ChangeRate = HashMap<String, Vec<f64>>;

pub fn read_data(data_file: &PathBuf, metric: &String) -> anyhow::Result<LabeledData> {
    // Filter out non-relevant metrics and reshape stats into Hashmap

    match serde_json::from_reader::<_, CompileTimeResultSet>(BufReader::new(File::open(data_file)?))
    {
        Ok(s) => return Ok(reshape_data(s, metric)),
        Err(_) => (),
    }

    match serde_json::from_reader::<_, RuntimeResultVec>(BufReader::new(File::open(data_file)?)) {
        Ok(s) => return Ok(reshape_runtime_data(s, metric)),
        Err(e) => bail!(e),
    }
}

pub fn calculate_change_rate(data_a: &LabeledData, data_b: &LabeledData) -> ChangeRate {
    data_a
        .into_iter()
        .map(|(k, u)| {
            (k.clone(), {
                let mut v = data_b.get(k).unwrap().clone();
                v.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let mut u = u.clone();
                u.sort_by(|a, b| a.partial_cmp(b).unwrap());

                let mut v = v.into_iter();
                assert_eq!(u.len(), v.len());

                u.into_iter()
                    .map(|a| {
                        let b = v.next().unwrap();
                        (a - b) / b * 100.
                    })
                    .collect()
            })
        })
        .collect()
}

fn reshape_data(data: CompileTimeResultSet, metric: &String) -> LabeledData {
    data.results
        .into_iter()
        .map(|s| {
            (
                s.benchmark,
                s.result_vec
                    .into_iter()
                    .map(|d| {
                        d.stats
                            .stats
                            .into_iter()
                            .find(|(m, _)| m == metric)
                            .unwrap()
                    })
                    .map(|x| x.1)
                    .collect(),
            )
        })
        .collect()
}

fn reshape_runtime_data(data: RuntimeResultVec, metric: &String) -> LabeledData {
    data.0
        .into_iter()
        .map(|s| {
            (
                s.name,
                s.stats
                    .into_iter()
                    .map(|stats| stats.stats.into_iter().find(|(m, _)| m == metric).unwrap())
                    .map(|x| x.1)
                    .collect(),
            )
        })
        .collect()
}
