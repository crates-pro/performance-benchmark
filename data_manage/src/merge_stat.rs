use std::{
    fs::{read_dir, File},
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use collector::{
    benchmark::profile::Profile,
    compile_time::result::{CompileTimeResultSet, CompileTimeStatistics},
};

pub fn merge_compile_time_stats(
    root_dir: &PathBuf,
    profile: Profile,
    rustc: String,
    out_path: PathBuf,
) -> anyhow::Result<PathBuf> {
    let mut merged_stats = CompileTimeStatistics::new();

    // Iterate each benchmark group under root dir.
    for bench_group in read_dir(root_dir)? {
        let bench_group = bench_group?;
        // Iterate each rustc stats dir under root dir.
        if bench_group.metadata()?.is_dir() {
            for rustc_dir in read_dir(bench_group.path())? {
                let rustc_dir = rustc_dir?;
                // Find wanted rustc version.
                if rustc_dir.metadata()?.is_dir()
                    && rustc_dir.file_name().to_str().unwrap().to_string() == rustc
                {
                    // Find statistics file
                    for f in read_dir(rustc_dir.path())? {
                        let f = f?;
                        if f.file_name().to_str().unwrap().contains("results.json") {
                            let data: CompileTimeResultSet =
                                serde_json::from_reader(BufReader::new(File::open(f.path())?))?;

                            merged_stats.append(
                                &mut data
                                    .calculate_statistics()
                                    .into_iter()
                                    .filter(|s| s.profile == profile)
                                    .collect(),
                            );
                        }
                    }
                }
            }
        }
    }

    merged_stats.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    serde_json::to_writer(BufWriter::new(File::create(&out_path)?), &merged_stats)?;

    Ok(out_path)
}

#[cfg(test)]
mod test_merge_stat {
    use std::{
        fs::{remove_file, File},
        io::BufReader,
        path::PathBuf,
    };

    use collector::{benchmark::profile::Profile, compile_time::result::CompileTimeStatistics};

    use super::merge_compile_time_stats;

    /// test for merge_stat
    ///
    /// Step1. merge stats in `test/merge_stat/stat` for `rustc_A`.
    ///
    /// Step2. verify length of merged stats.
    ///
    /// Step3. clean up.
    #[test]
    fn test_merge_stat() {
        let root_dir = PathBuf::from("test/merge_stat/stat");
        let profile = Profile::Release;
        let rustc = String::from("rustc_A");
        let out_path = PathBuf::from("test/merge_stat/merge.json");

        assert_eq!(
            merge_compile_time_stats(&root_dir, profile, rustc, out_path.clone()).unwrap(),
            out_path,
        );

        let stats: CompileTimeStatistics =
            serde_json::from_reader(BufReader::new(File::open(&out_path).unwrap())).unwrap();

        assert_eq!(12, stats.len());

        remove_file(out_path).unwrap();
    }
}
