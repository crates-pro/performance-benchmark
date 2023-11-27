use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use super::compare::FilePair;

/// `Stat` maps benchmark-name to average_statistic.
#[derive(Debug)]
pub struct Stat {
    pub data: HashMap<String, f64>,
    pub metric: String,
    pub profile: String,
}

pub(super) fn compare_stats(file_pair: FilePair) -> anyhow::Result<Stat> {
    let new_stat = get_stats(file_pair.new)?;
    let old_stat = get_stats(file_pair.old)?;

    assert_eq!(new_stat.metric, old_stat.metric);
    assert_eq!(new_stat.profile, old_stat.profile);

    Ok(Stat {
        data: new_stat
            .data
            .into_iter()
            .map(|(n, v)| {
                (
                    n.clone(),
                    (v - *old_stat.data.get(&n).unwrap()) / *old_stat.data.get(&n).unwrap()
                        * 100f64,
                )
            })
            .collect(),
        metric: new_stat.metric.clone(),
        profile: new_stat.profile.clone(),
    })
}

fn get_stats(csv_file: PathBuf) -> anyhow::Result<Stat> {
    let mut stat = vec![];

    let mut reader = BufReader::new(File::open(&csv_file)?);

    let mut buf = String::new();
    let mut first_line = true;
    let mut line_num = 0;

    while reader.read_line(&mut buf)? != 0 {
        if first_line == true {
            buf.split(',').into_iter().for_each(|s| {
                stat.push((s.trim().to_string(), 0));
            });
            first_line = false;
        } else {
            let mut col_num = 0;
            buf.split(',').into_iter().for_each(|s| {
                stat[col_num].1 += s.trim().parse::<u128>().unwrap();
                col_num += 1;
            });
            line_num += 1;
        }

        buf.clear();
    }

    Ok(Stat {
        data: stat
            .into_iter()
            .map(|(s, v)| (s, (v / line_num) as f64))
            .collect(),
        metric: csv_file
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .split('_')
            .collect::<Vec<&str>>()[2]
            .split('.')
            .collect::<Vec<&str>>()[0]
            .to_string(),
        profile: csv_file
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .split('_')
            .collect::<Vec<&str>>()[1]
            .to_string(),
    })
}

mod test {

    /// `test_get_stats` will test
    /// the Stats returned by `find_file_pairs`
    /// is correctly managed.
    #[test]
    fn test_get_stats() {
        use super::get_stats;

        let input_dir = std::path::PathBuf::from(
            "test/csv_transfer/compare/demo_data/current/current_debug_branch-misses.csv",
        );

        let stats = get_stats(input_dir).unwrap();

        let std_stats = vec![
            (String::from("stacks-blockchain-2.0.11.3.0"), 77662668.0),
            (String::from("graph-node-0.24.2"), 470974809.0),
            (String::from("starcoin-1.7.0"), 763275882.0),
            (String::from("conflux-rust-0.2.0"), 192631689.0),
            (String::from("diem-diem-core-v1.4.1"), 159944206.0),
            (String::from("cita-20.2.0"), 662885486.0),
        ];

        std_stats.into_iter().for_each(|(n, v)| {
            assert_eq!(*stats.data.get(&n).unwrap(), v);
        });

        assert_eq!(stats.metric, String::from("branch-misses"));
        assert_eq!(stats.profile, String::from("debug"));
    }

    /// `test_compare_stats` will check the
    /// correctness of calculation of change rate.
    #[test]
    fn test_compare_stats() {
        use std::path::PathBuf;

        use crate::csv_transfer::compare::compare::FilePair;
        use crate::csv_transfer::compare::stat::compare_stats;

        let file_pair = FilePair {
            new: PathBuf::from(
                "test/csv_transfer/compare/demo_data/current/current_debug_branch-misses.csv",
            ),
            old: PathBuf::from(
                "test/csv_transfer/compare/demo_data/old/old_debug_branch-misses.csv",
            ),
        };

        let stats = compare_stats(file_pair).unwrap();

        let std_stats = vec![
            (
                String::from("stacks-blockchain-2.0.11.3.0"),
                0.29552068549878746,
            ),
            (String::from("graph-node-0.24.2"), -0.3116770782225563),
            (String::from("starcoin-1.7.0"), 0.05869893808711376),
            (String::from("conflux-rust-0.2.0"), -0.18588486582409744),
            (String::from("diem-diem-core-v1.4.1"), -0.23109548429019158),
            (String::from("cita-20.2.0"), 0.06178561855019537),
        ];

        std_stats.into_iter().for_each(|(n, v)| {
            assert_eq!(*stats.data.get(&n).unwrap(), v);
        })
    }
}
