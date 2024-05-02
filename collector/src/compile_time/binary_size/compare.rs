use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf};

use crate::{benchmark::profile::Profile, statistics::compile_time_stat::CompileTimeBenchResult};

use super::BINARY_SIZE_LABEL;

/// Compare changes of binary sizes between 2 input files.
/// Step1. Read in 2 input files.
/// Step2. Calculate the change rate.
pub fn compare_binary_size(
    binary_size_a: &PathBuf,
    binary_size_b: &PathBuf,
    profile: Profile,
) -> HashMap<String, f64> {
    let read_binary_size = |p: &PathBuf| {
        let data: Vec<CompileTimeBenchResult> =
            serde_json::from_reader(BufReader::new(File::open(p).unwrap())).unwrap();
        data.into_iter()
            .map(move |i| {
                (
                    i.get_benchmark(),
                    *i.get_stats_ref_by_profile(&profile)
                        .first()
                        .unwrap()
                        .stats
                        .get(BINARY_SIZE_LABEL)
                        .unwrap(),
                )
            })
            .collect::<HashMap<_, _>>()
    };

    let binary_size_a = read_binary_size(binary_size_a);
    let binary_size_b = read_binary_size(binary_size_b);

    // Calculate differences of 2 inputs.
    let difference: HashMap<String, f64> = binary_size_a
        .clone()
        .iter()
        .map(|(b, lhs)| (b.clone(), lhs - binary_size_b.get(b).unwrap()))
        .collect();

    // Calculate change ratio.
    difference
        .iter()
        .map(|(b, d)| (b.clone(), { (d / binary_size_a.get(b).unwrap()) * 100. }))
        .collect()
}

#[cfg(test)]
mod test_compare {
    use std::path::PathBuf;

    use crate::benchmark::profile::Profile;

    use super::compare_binary_size;

    #[test]
    fn test_compare_binary_size() {
        let binary_size_a = PathBuf::from("test/binary_size/compare/binary_size_1.json");
        let binary_size_b = PathBuf::from("test/binary_size/compare/binary_size_2.json");

        assert_ne!(
            0,
            compare_binary_size(&binary_size_a, &binary_size_b, Profile::Release).len()
        );
    }
}
