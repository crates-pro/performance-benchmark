use std::{
    collections::HashMap,
    fs::File,
    io::BufReader,
    path::PathBuf,
    process::{Command, Stdio},
};

use crate::{
    benchmark::profile::Profile, compile_time::binary_size::BINARY_SIZE_LABEL,
    statistics::compile_time_stat::CompileTimeBenchResult,
};

pub fn plot(
    data_file_a: PathBuf,
    data_file_b: PathBuf,
    label_a: String,
    label_b: String,
    out_path: PathBuf,
    profile: Profile,
) -> anyhow::Result<()> {
    let data_a: Vec<CompileTimeBenchResult> =
        serde_json::from_reader(BufReader::new(File::open(data_file_a)?))?;

    let data_b: Vec<CompileTimeBenchResult> =
        serde_json::from_reader(BufReader::new(File::open(data_file_b)?))?;

    let get_benchmark_binary_size = |d: &Vec<CompileTimeBenchResult>| {
        d.iter()
            .map(|d| {
                d.get_benchmark()
                    + ","
                    + d.get_stats_ref_by_profile(&profile)
                        .first()
                        .unwrap()
                        .stats
                        .get(&BINARY_SIZE_LABEL.to_string())
                        .unwrap()
                        .to_string()
                        .as_str()
            })
            .collect::<Vec<String>>()
            .join(";")
    };

    let benchmark_binary_size_a = get_benchmark_binary_size(&data_a);
    let benchmark_binary_size_b = get_benchmark_binary_size(&data_b);

    let mut cmd = Command::new("python");
    cmd.arg("src/compile_time/binary_size/plot/plotter.py")
        .arg(benchmark_binary_size_a)
        .arg(benchmark_binary_size_b)
        .arg(label_a)
        .arg(label_b)
        .arg(&out_path);
    cmd.stdout(Stdio::inherit());
    cmd.spawn().unwrap().wait().unwrap();

    Ok(())
}

pub fn plot_compare(data: &HashMap<String, f64>, out_path: PathBuf) -> anyhow::Result<PathBuf> {
    let mut cmd = Command::new("python");
    cmd.arg("src/compile_time/binary_size/plot/plotter_cmp.py")
        .arg(
            data.into_iter()
                .map(|(k, v)| format!("{},{}", k, v))
                .collect::<Vec<String>>()
                .join(";"),
        )
        .arg(&out_path);
    cmd.stdout(Stdio::inherit());
    cmd.spawn().unwrap().wait().unwrap();

    Ok(out_path)
}

#[cfg(test)]
mod test_binary_size_plotter {
    use std::{
        fs::{self, remove_file},
        path::PathBuf,
    };

    use crate::{
        benchmark::profile::Profile, compile_time::binary_size::compare::compare_binary_size,
    };

    use super::{plot, plot_compare};

    #[test]
    fn test_plotter() {
        let data_path_a = PathBuf::from("test/binary_size/plotter/merged_binary_size.json");
        let data_path_b =
            PathBuf::from("test/binary_size/plotter/merged_rustc_perf_binary_size.json");
        let file_path = PathBuf::from("test/binary_size/plotter/merged_binary_size.jpg");

        plot(
            data_path_a,
            data_path_b,
            "A".to_string(),
            "B".to_string(),
            file_path.clone(),
            Profile::Release,
        )
        .unwrap();

        fs::metadata(&file_path).unwrap();
        remove_file(file_path).unwrap();
    }

    #[test]
    fn test_plotter_cmp() {
        let binary_size_a = PathBuf::from("test/binary_size/compare/binary_size_1.json");
        let binary_size_b = PathBuf::from("test/binary_size/compare/binary_size_2.json");
        let out_path = PathBuf::from("test/binary_size/compare/compare.jpg");

        plot_compare(
            &compare_binary_size(&binary_size_a, &binary_size_b, Profile::Release),
            out_path.clone(),
        )
        .unwrap();

        fs::metadata(&out_path).unwrap();
        remove_file(out_path).unwrap();
    }
}
