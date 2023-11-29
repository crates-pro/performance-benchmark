use std::{collections::HashMap, fs::read_dir, path::PathBuf};

use crate::csv_transfer::get_sub_dir;

use super::{
    stat::compare_stats,
    writer::{write_csv, write_tex},
};

/// `FilePair` is used to store csv files of current & old rustc versions
/// with the same metric and profile.
pub(super) struct FilePair {
    pub new: PathBuf,
    pub old: PathBuf,
}

/// `do_compare` will compare statistics grouped by profile & metric,
/// calculate the change rates and write them to csv file & tex file.
pub(crate) fn do_compare(dir: PathBuf) -> anyhow::Result<()> {
    find_file_pairs(dir.clone())?.into_iter().for_each(|pair| {
        let stat = compare_stats(pair).unwrap();

        write_csv(&stat, dir.clone()).unwrap();
        write_tex(&stat, dir.clone()).unwrap()
    });

    Ok(())
}

/// `find_file_pairs` will find the csv files of current & old rustc versions
/// and group them into pairs by different metrics & profiles.
fn find_file_pairs(dir: PathBuf) -> anyhow::Result<Vec<FilePair>> {
    let mut file_map = HashMap::new();

    let mut current_dir = None;
    let mut old_dir = None;

    get_sub_dir(&dir).iter().for_each(|d| {
        if d.to_str().unwrap().contains("current") {
            current_dir = Some(d.clone());
        }
        if d.to_str().unwrap().contains("old") {
            old_dir = Some(d.clone());
        }
    });

    if let Some(current_dir) = current_dir {
        if let Some(old_dir) = old_dir {
            for entry in read_dir(current_dir)? {
                let entry = entry?;
                if entry.file_name().to_str().unwrap().ends_with(".csv") {
                    file_map.insert(
                        String::from(entry.file_name().to_str().unwrap()),
                        String::new(),
                    );
                }
            }

            for entry in read_dir(old_dir)? {
                let entry = entry?;
                if entry.file_name().to_str().unwrap().ends_with(".csv") {
                    if let Some(pair) = file_map.get_mut(&String::from(
                        entry
                            .file_name()
                            .to_str()
                            .unwrap()
                            .replace("old", "current"),
                    )) {
                        *pair = String::from(entry.file_name().to_str().unwrap());
                    } else {
                        file_map.remove(&String::from(
                            entry
                                .file_name()
                                .to_str()
                                .unwrap()
                                .replace("old", "current"),
                        ));
                        eprintln!(
                            "csv file for {} not found.",
                            entry
                                .file_name()
                                .to_str()
                                .unwrap()
                                .replace("old", "current")
                        );
                    }
                }
            }
        } else {
            eprintln!("Statistics for old version not found.");
        }
    } else {
        eprintln!("Statistics for current version not found.");
    }

    Ok(file_map
        .into_iter()
        .map(|(a, b)| FilePair {
            new: dir.to_path_buf().join("current").join(a),
            old: dir.to_path_buf().join("old").join(b),
        })
        .collect())
}

#[cfg(test)]
mod test {

    use std::path::PathBuf;

    use crate::csv_transfer::compare::compare::FilePair;

    impl PartialEq for FilePair {
        fn eq(&self, other: &Self) -> bool {
            self.new.to_str().unwrap().eq(other.new.to_str().unwrap())
                && self.old.to_str().unwrap().eq(other.old.to_str().unwrap())
        }
    }

    #[allow(dead_code)]
    fn assert_file_eq(path: &PathBuf, std_path: &PathBuf) {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let f = File::open(path).unwrap();
        let std_f = File::open(std_path).unwrap();

        let mut f_reader = BufReader::new(f);
        let mut std_reader = BufReader::new(std_f);

        let mut buf_f = String::new();
        let mut buf_std = String::new();

        while std_reader.read_line(&mut buf_std).unwrap() > 0 {
            f_reader.read_line(&mut buf_f).unwrap();

            assert_eq!(buf_f, buf_std);

            buf_f.clear();
            buf_std.clear();
        }
    }

    /// `test_find_file_pairs` will test
    /// the FilePairs returned by `find_file_pairs`
    /// is correctly grouped.
    #[test]
    fn test_find_file_pairs() {
        use super::find_file_pairs;
        use std::path::PathBuf;

        let input_dir = std::path::PathBuf::from("test/csv_transfer/compare/demo_data");

        let pairs = find_file_pairs(input_dir).unwrap();

        let anwser_pairs = vec![
            FilePair {
                new: PathBuf::from("test/csv_transfer/compare/demo_data/current/current_debug_cache-misses.csv"),
                old: PathBuf::from("test/csv_transfer/compare/demo_data/old/old_debug_cache-misses.csv"),
            },
            FilePair {
                new: PathBuf::from("test/csv_transfer/compare/demo_data/current/current_debug_branch-misses.csv"),
                old: PathBuf::from("test/csv_transfer/compare/demo_data/old/old_debug_branch-misses.csv"),
            },
            FilePair {
                new: PathBuf::from("test/csv_transfer/compare/demo_data/current/current_debug_context-switches.csv"),
                old: PathBuf::from("test/csv_transfer/compare/demo_data/old/old_debug_context-switches.csv"),
            },
        ];

        anwser_pairs.iter().for_each(|p| {
            assert!(pairs.contains(p));
        });
    }

    /// `test_do_compare` will check the entire procedure of comparing data collected on several benchmarks.
    #[test]
    fn test_do_compare() {
        use super::do_compare;
        use std::path::PathBuf;

        let dir = PathBuf::from("test/csv_transfer/compare/demo_data");
        do_compare(dir).unwrap();

        vec![
            (
                PathBuf::from("test/csv_transfer/compare/demo_data/debug_branch-misses.csv"),
                PathBuf::from("test/csv_transfer/compare/demo_data/debug_branch-misses std.csv"),
            ),
            (
                PathBuf::from("test/csv_transfer/compare/demo_data/debug_cache-misses.csv"),
                PathBuf::from("test/csv_transfer/compare/demo_data/debug_cache-misses std.csv"),
            ),
            (
                PathBuf::from("test/csv_transfer/compare/demo_data/debug_context-switches.csv"),
                PathBuf::from("test/csv_transfer/compare/demo_data/debug_context-switches std.csv"),
            ),
            (
                PathBuf::from("test/csv_transfer/compare/demo_data/debug_branch-misses.tex"),
                PathBuf::from("test/csv_transfer/compare/demo_data/debug_branch-misses std.tex"),
            ),
            (
                PathBuf::from("test/csv_transfer/compare/demo_data/debug_cache-misses.tex"),
                PathBuf::from("test/csv_transfer/compare/demo_data/debug_cache-misses std.tex"),
            ),
            (
                PathBuf::from("test/csv_transfer/compare/demo_data/debug_context-switches.tex"),
                PathBuf::from("test/csv_transfer/compare/demo_data/debug_context-switches std.tex"),
            ),
        ]
        .into_iter()
        .for_each(|(path, std_path)| {
            assert_file_eq(&path, &std_path);

            std::fs::remove_file(path).unwrap();
        });
    }
}
