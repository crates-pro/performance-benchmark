use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

use crate::morpheme_miner::derive_debug_miner::derive_debug_miner::DeriveDebugMiner;

/// MinerFile is a trait that describes how a morpheme will be mined from a single file.
pub(crate) trait MorphemeMiner {
    /// The default implementation of `mine_from_project` will
    /// gather all the mining results of files from the proj_dir.
    fn mine_from_project(&self, proj_dir: &PathBuf) -> anyhow::Result<InfoProject> {
        let mut results = (self.morpheme_name(), 0);

        for entry in fs::read_dir(proj_dir)? {
            let entry = entry?;

            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    // run `mine_from_project` recursively if current entry is a directry.
                    results = union_info(&results, &self.mine_from_project(&entry.path())?);
                } else if metadata.is_file() {
                    // Only run `mine_from_file` for rust files.
                    if if let Some(file_name) = entry.file_name().to_str() {
                        file_name.ends_with(".rs")
                    } else {
                        false
                    } {
                        let file = File::open(entry.path())?;
                        results = union_info(&results, &self.mine_from_file(BufReader::new(file)));
                    }
                }
            }
        }

        Ok(results)
    }

    fn mine_from_file(&self, reader: BufReader<File>) -> InfoFile;

    fn morpheme_name(&self) -> String;
}

pub(crate) type Miners = Vec<Box<dyn MorphemeMiner>>;

/// `InfoFile` describes the times of specific morpheme appears in a single file.
pub(crate) type InfoFile = (String, i128);

/// `InfoProject` describes the times of specific morpheme appears in a project.
pub(crate) type InfoProject = (String, i128);

/// `SynthesisInfo` describes the statistics of all morphemes of a specific project.
pub(crate) type SynthesisInfo = HashMap<String, Vec<InfoProject>>;

/// `get_miners()` will return a vec of MorphemeMiner that shall be executed later.
pub(crate) fn get_miners() -> Miners {
    vec![Box::new(DeriveDebugMiner {})]
}

/// `output_csv` will write the collected `SynthesisInfo` to a csv file.
pub(crate) fn output_csv(info: SynthesisInfo, mut writer: BufWriter<File>) {
    if info.is_empty() {
        eprintln!("Empty result info");
        return;
    }

    // Sort all statistics by morpheme and proj_name.
    let mut info = info
        .into_iter()
        .collect::<Vec<_>>()
        .into_iter()
        .map(|(proj_name, mut morphemes)| {
            morphemes.sort_by(|(a, _), (b, _)| a.cmp(b));
            (proj_name, morphemes)
        })
        .collect::<Vec<_>>();
    info.sort_by(|(a, _), (b, _)| a.cmp(b));

    // The first line consists of names of morphemes in info.
    writer
        .write_all(
            format!(
                "{}\n",
                info.first()
                    .unwrap()
                    .1
                    .iter()
                    .map(|morpheme_count| { morpheme_count.0.clone() })
                    .collect::<Vec<_>>()
                    .iter()
                    .map(|morpheme| { format!(",{}", morpheme) })
                    .collect::<String>()
            )
            .as_bytes(),
        )
        .unwrap();

    // The following lines are statistics of each project.
    info.into_iter().for_each(|(proj_name, info)| {
        writer
            .write_all(
                format!(
                    "{}{}\n",
                    proj_name,
                    info.iter()
                        .map(|(_, count)| { format!(",{}", *count) })
                        .collect::<String>()
                )
                .as_bytes(),
            )
            .unwrap();
    });
}

/// `union_info` performs add & union operation on two `HashMap<String, i128>`.
fn union_info(info_1: &(String, i128), info_2: &(String, i128)) -> (String, i128) {
    let mut result = info_1.clone();
    result.1 += info_2.1;
    result
}

/// `test_output_csv_with_derive_debug_miner` will check
/// `output_csv` by using `DeriveDebugMiner`.
///
/// Step1. create output directry & run `DeriveDebugMiner`;
///
/// Step2. check output csv file;
///
/// Step3. remove output directory;
#[test]
fn test_run_miners_with_derive_debug_miner() {
    use super::run_miners;
    use std::io::BufRead;

    // Step1. create output directry & run `DeriveDebugMiner`;
    let test_dir = PathBuf::from("test/MorphemeMiner/test_run_miners_with_derive_debug_miner");
    let csv_file =
        PathBuf::from("test/MorphemeMiner/test_run_miners_with_derive_debug_miner/morpheme.csv");

    fs::create_dir(&test_dir).unwrap();
    run_miners(
        PathBuf::from("test/MorphemeMiner/DeriveDebugMiner"),
        csv_file.clone(),
    );

    // Step2. check output csv file;
    let mut stdout = vec![",Derive Debug\n", "proj_1,1\n", "proj_2,4\n"];
    stdout.reverse();
    let file = File::open(csv_file).unwrap();
    let mut reader = BufReader::new(file);

    let mut buf = String::new();
    while let Ok(size) = reader.read_line(&mut buf) {
        if size == 0 {
            break;
        }

        assert_eq!(stdout.pop().unwrap(), buf);

        buf.clear();
    }

    assert!(stdout.is_empty());

    // Step3. remove output directory;
    fs::remove_dir_all(test_dir).unwrap();
}
