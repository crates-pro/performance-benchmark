use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{execute::Stats, statistics::statistic::Statistics};

#[derive(Debug, Serialize, Deserialize)]
pub struct RuntimeResultVec(pub Vec<RuntimeResult>);

impl RuntimeResultVec {
    pub fn calculate_statistics(&self) -> RuntimeStatistics {
        let mut statistics = RuntimeStatistics::new();
        if self.0.len() == 0 {
            return statistics;
        }
        self.0.iter().for_each(|result| {
            let mut stat_map = HashMap::<String, Vec<f64>>::new();

            result.stats.iter().for_each(|stat| {
                stat.stats.iter().for_each(|(label, val)| {
                    if stat_map.contains_key(label) {
                        stat_map.get_mut(label).unwrap().push(val.clone());
                    } else {
                        stat_map.insert(label.clone(), vec![val.clone()]);
                    }
                });
            });

            let mut statistic_vec = vec![];
            stat_map.iter().for_each(|(label, vals)| {
                statistic_vec.push((label.clone(), Statistics::from(vals.clone())));
            });
            statistics.push(RuntimeStatistic {
                name: result.name.clone(),
                statistic_vec,
            });
        });
        statistics
    }
}

/// Stats gathered by several iterations of a single benchmark.
#[derive(Debug, Serialize, Deserialize)]
pub struct RuntimeResult {
    pub name: String,
    pub stats: Vec<Stats>,
}

impl RuntimeResult {
    pub fn new(name: String) -> Self {
        RuntimeResult {
            name,
            stats: vec![],
        }
    }

    pub fn append(&mut self, stats: Stats) -> &Self {
        self.stats.push(stats);
        self
    }
}

#[derive(Serialize, Deserialize)]
pub struct RuntimeStatistic {
    pub name: String,
    pub statistic_vec: Vec<(String, Statistics)>,
}

pub type RuntimeStatistics = Vec<RuntimeStatistic>;
