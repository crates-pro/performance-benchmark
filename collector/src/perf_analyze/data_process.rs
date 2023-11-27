use std::{
    collections::{HashMap, HashSet},
    ops::AddAssign,
};

use crate::statistics::statistic::Statistics;

use super::{analyze::EventCostMap, PerfData, PerfRawData};

/// Split EventCostMap into PerfDatas by metric,
/// returns HashMap<String, PerfData> mapping metric into PerfData.
pub(super) fn split_data_by_metric(raw_data: EventCostMap) -> HashMap<String, PerfRawData> {
    let mut metric_data_map: HashMap<String, PerfRawData> = HashMap::new();

    // Get all different metrics in row_data, and create an empty PerfData for it.
    raw_data
        .iter()
        .map(|((_, metric), _)| metric.clone())
        .collect::<HashSet<String>>()
        .iter()
        .for_each(|metric| {
            metric_data_map.insert(
                metric.clone(),
                PerfRawData {
                    metric: metric.clone(),
                    symbol_data_map: HashMap::<String, f64>::default(),
                },
            );
        });

    // Iter all data in raw_data, add it to PerfData according to its metric.
    raw_data.iter().for_each(|((symbol, metric), val)| {
        let perf_data = metric_data_map.get_mut(metric).unwrap();
        perf_data.symbol_data_map.insert(symbol.clone(), *val);
    });

    metric_data_map
}

/// Add up PerfData of same metric into one and calculate the
pub(super) fn merge_perf_datas(
    datas: Vec<HashMap<String, PerfRawData>>,
) -> HashMap<String, PerfData> {
    let mut merged_datas = HashMap::new();
    // aggregate PerfRawData of the same metric
    datas.iter().for_each(|data| {
        data.iter().for_each(|(metric, data)| {
            match merged_datas.get_mut(metric) {
                Some(merged_data) => *merged_data += data.clone(),
                None => {
                    let mut perf_data = PerfData::default();
                    perf_data.metric = metric.clone();
                    perf_data += data.clone();

                    merged_datas.insert(metric.clone(), perf_data);
                }
            };
        })
    });

    // calculate statistics for each PerfData
    merged_datas.iter_mut().for_each(|(_, perf_data)| {
        perf_data.symbol_data_map.iter().for_each(|(metric, vals)| {
            perf_data
                .symbol_statistics_map
                .insert(metric.clone(), Statistics::from(vals.clone()));
        })
    });

    merged_datas
}

impl AddAssign<PerfRawData> for PerfData {
    fn add_assign(&mut self, rhs: PerfRawData) {
        assert!(self.metric == rhs.metric);
        rhs.symbol_data_map.iter().for_each(|(metric, r_val)| {
            match self.symbol_data_map.get_mut(metric) {
                Some(l_val) => l_val.push(*r_val),
                None => {
                    self.symbol_data_map.insert(metric.clone(), vec![*r_val]);
                }
            }
        })
    }
}
