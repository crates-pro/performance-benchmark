use std::{
    collections::HashMap,
    ops::{Add, AddAssign},
    process::{self, Output},
};

use serde::{Deserialize, Serialize};

pub fn process_benchmark_output(output: Output) -> Result<Stats, DeserializeStatError> {
    let stdout = String::from_utf8(output.stdout.clone()).expect("utf8 output");
    let mut stats = Stats::new();
    let mut perf_output = false;
    for line in stdout.lines() {
        // filter out cargo test output msg.
        if perf_output == false {
            if line.contains("instructions") {
                perf_output = true;
            } else {
                continue;
            }
        }
        if line.contains("ignored") {
            continue;
        }
        // github.com/torvalds/linux/blob/bc78d646e708/tools/perf/Documentation/perf-stat.txt#L281
        macro_rules! get {
            ($e: expr) => {
                match $e {
                    Some(s) => s,
                    None => {
                        log::warn!("unhandled line: {}", line);
                        continue;
                    }
                }
            };
        }
        let mut parts = line.split(';').map(|s| s.trim());
        let cnt = get!(parts.next());
        let _unit = get!(parts.next());
        let name = get!(parts.next());
        let _time = get!(parts.next());
        let pct = get!(parts.next());
        if cnt == "<not supported>" || cnt.len() == 0 {
            continue;
        }
        if !pct.starts_with("100.") {
            panic!(
                "measurement of `{}` only active for {}% of the time",
                name, pct
            );
        }
        stats.insert(
            name.to_owned(),
            cnt.parse()
                .map_err(|e| DeserializeStatError::ParseError(cnt.to_string(), e))?,
        );
    }

    log::info!("{:?}", stdout.lines());
    log::info!("{:?}", stats.stats);
    if stats.is_empty() {
        return core::result::Result::Err(DeserializeStatError::NoOutput(output));
    }
    Ok(stats)
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Stats {
    pub stats: HashMap<String, f64>,
}

impl Default for Stats {
    fn default() -> Self {
        Stats::new()
    }
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            stats: HashMap::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, f64)> + '_ {
        self.stats.iter().map(|(k, v)| (k.as_str(), *v))
    }

    pub fn is_empty(&self) -> bool {
        self.stats.is_empty()
    }

    pub fn insert(&mut self, stat: String, value: f64) {
        self.stats.insert(stat, value);
    }

    pub fn add_or_insert(&mut self, stat: String, value: f64) {
        match self.stats.get_mut(&stat) {
            Some(e) => *e += value,
            None => self.insert(stat, value),
        }
    }
}

impl Add for Stats {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut res = Self::new();
        for (label, left_val) in self.stats.iter() {
            let right_val = rhs.stats.get(label);
            match right_val {
                Some(right_val) => res.insert(label.clone(), left_val + right_val),
                None => res.insert(label.clone(), left_val.clone()),
            }
        }
        res
    }
}

impl AddAssign for Stats {
    fn add_assign(&mut self, rhs: Self) {
        for (label, right_val) in rhs.stats.iter() {
            let left_val = self.stats.get_mut(label);
            match left_val {
                Some(left_val) => *left_val += right_val,
                None => self.insert(label.clone(), right_val.clone()),
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DeserializeStatError {
    #[error("could not deserialize empty output to stats, output: {:?}", .0)]
    NoOutput(process::Output),
    #[error("could not parse `{}` as a float", .0)]
    ParseError(String, #[source] ::std::num::ParseFloatError),
    #[error("could not process xperf data")]
    XperfError(#[from] anyhow::Error),
}
