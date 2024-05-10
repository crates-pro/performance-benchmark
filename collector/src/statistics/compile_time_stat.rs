use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    benchmark::{profile::Profile, scenario::Scenario},
    execute::Stats,
    statistics::statistic::Statistics,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompileTimeResult {
    pub benchmark: String,
    pub iteration: usize,
    pub profile: Profile,
    pub scenario: Scenario,
    pub stats: Stats,
}

impl CompileTimeResult {
    pub fn new(
        benchmark: String,
        iteration: usize,
        profile: Profile,
        scenario: Scenario,
        stats: Stats,
    ) -> Self {
        CompileTimeResult {
            benchmark,
            iteration,
            profile,
            scenario,
            stats,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompileTimeBenchResult {
    pub benchmark: String,
    pub iterations: usize,
    pub result_vec: Vec<CompileTimeResult>,
}

impl CompileTimeBenchResult {
    pub fn new(benchmark: String, iterations: usize) -> Self {
        CompileTimeBenchResult {
            benchmark,
            iterations,
            result_vec: vec![],
        }
    }
    pub fn add_result(&mut self, result: CompileTimeResult) -> &Self {
        self.result_vec.push(result);
        self
    }

    pub fn get_benchmark(&self) -> String {
        self.benchmark.clone()
    }

    pub fn get_iterations(&self) -> usize {
        self.iterations
    }

    pub fn get_stats_ref_by_profile(&self, profile: &Profile) -> Vec<&Stats> {
        self.result_vec
            .iter()
            .filter_map(|r| {
                if r.profile == *profile {
                    Some(&r.stats)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_stats_with_profile_scenario(&self) -> HashMap<(Profile, Scenario), Vec<Stats>> {
        let mut map = HashMap::<(Profile, Scenario), Vec<Stats>>::new();
        self.result_vec.iter().for_each(|r| {
            if map.contains_key(&(r.profile, r.scenario)) {
                let stats = map.get_mut(&(r.profile, r.scenario)).unwrap();
                stats.push(r.stats.clone());
            } else {
                map.insert((r.profile, r.scenario), vec![r.stats.clone()]);
            }
        });
        map
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CompileTimeResultSet {
    pub id: String,
    pub results: Vec<CompileTimeBenchResult>,
}

impl CompileTimeResultSet {
    pub fn new(id: String, results: Vec<CompileTimeBenchResult>) -> Self {
        CompileTimeResultSet { id, results }
    }

    pub fn calculate_statistics(&self) -> CompileTimeStatistics {
        let mut statistics = CompileTimeStatistics::new();
        self.results.iter().for_each(|result| {
            let stats_map = result.get_stats_with_profile_scenario();

            stats_map.iter().for_each(|((profile, scenario), stats)| {
                let mut statistic_vec = Vec::<(String, Statistics)>::new();
                let mut data_map = HashMap::<String, Vec<f64>>::new();

                stats.iter().for_each(|stat| {
                    stat.stats.iter().for_each(|(label, value)| {
                        if data_map.contains_key(label) {
                            data_map.get_mut(label).unwrap().push(value.clone());
                        } else {
                            data_map.insert(label.clone(), vec![value.clone()]);
                        }
                    });
                });

                data_map.iter().for_each(|(label, vals)| {
                    statistic_vec.push((label.clone(), Statistics::from(vals.clone())));
                });

                statistics.push(CompileTimeStatistic {
                    name: result.benchmark.clone(),
                    profile: profile.clone(),
                    scenario: scenario.clone(),
                    iterations: result.iterations as u32,
                    statistic_vec,
                });
            });
        });
        statistics
    }

    pub fn get_ref_results(&self) -> &Vec<CompileTimeBenchResult> {
        &self.results
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CompileTimeStatistic {
    pub name: String,
    pub profile: Profile,
    pub scenario: Scenario,
    pub iterations: u32,
    pub statistic_vec: Vec<(String, Statistics)>,
}

pub type CompileTimeStatistics = Vec<CompileTimeStatistic>;
