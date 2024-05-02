use std::{
    collections::HashMap,
    path::PathBuf,
    process::{Command, Stdio},
};

use super::stats::{calculate_change_rate, read_stats};

pub fn compare_stat(
    stats_a: &PathBuf,
    stats_b: &PathBuf,
    metric: &String,
    out_path: PathBuf,
) -> anyhow::Result<PathBuf> {
    let stats_a = read_stats(stats_a, metric)?;
    let stats_b = read_stats(stats_b, metric)?;

    // Calculate change rate of stats_a on stats_b
    let change_rate = calculate_change_rate(&stats_a, &stats_b);

    plot_compare(&change_rate, out_path, metric)
}

fn plot_compare(
    data: &HashMap<String, f64>,
    out_path: PathBuf,
    metric: &String,
) -> anyhow::Result<PathBuf> {
    let mut cmd = Command::new("python");
    cmd.arg("src/plotters/plotter_cmp.py")
        .arg(
            data.into_iter()
                .map(|(k, v)| format!("{},{}", k, v))
                .collect::<Vec<String>>()
                .join(";"),
        )
        .arg(&out_path)
        .arg(metric);
    cmd.stdout(Stdio::inherit());
    cmd.spawn().unwrap().wait().unwrap();

    Ok(out_path)
}

#[cfg(test)]
mod test_compare_stat {
    use std::{
        fs::{self, remove_file},
        path::PathBuf,
    };

    use super::compare_stat;

    /// test for compare_stat
    ///
    /// Step1. compare stats of metric `instructions` in `test/compare_stat/stat`.
    ///
    /// Step2. plot and check the compare result.
    ///
    /// Step3. clean up.
    #[test]
    fn test_compare_stat() {
        let stat_1 = PathBuf::from("test/compare_stat/stat/merged_statstics_current.json");
        let stat_2 = PathBuf::from("test/compare_stat/stat/merged_statstics_old.json");
        let metric = String::from("instructions:u");
        let out_path = PathBuf::from("test/compare_stat/compare_stat.jpeg");

        assert_eq!(
            out_path.clone(),
            compare_stat(&stat_1, &stat_2, &metric, out_path.clone()).unwrap()
        );

        fs::metadata(&out_path).unwrap();
        remove_file(out_path).unwrap();
    }
}
