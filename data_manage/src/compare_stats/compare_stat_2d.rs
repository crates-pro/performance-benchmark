use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

use super::stats::{calculate_change_rate, read_stats, ChangeRate};

pub fn compare_stat_2d(
    stats_a: &PathBuf,
    stats_b: &PathBuf,
    metric_1: &String,
    metric_2: &String,
    out_path: PathBuf,
) -> anyhow::Result<PathBuf> {
    // Calculate change rate of stats_a on stats_b
    let change_rate_1 = calculate_change_rate(
        &read_stats(stats_a, metric_1)?,
        &read_stats(stats_b, metric_1)?,
    );
    let change_rate_2 = calculate_change_rate(
        &read_stats(stats_a, metric_2)?,
        &read_stats(stats_b, metric_2)?,
    );

    plot_compare(&change_rate_1, &change_rate_2, out_path, metric_1, metric_2)
}

fn plot_compare(
    data_1: &ChangeRate,
    data_2: &ChangeRate,
    out_path: PathBuf,
    metric_1: &String,
    metric_2: &String,
) -> anyhow::Result<PathBuf> {
    let mut cmd = Command::new("python");
    cmd.arg("src/plotters/plotter_cmp_2d.py")
        .arg(
            data_1
                .into_iter()
                .map(|(k, v)| format!("{},{}", k, v))
                .collect::<Vec<String>>()
                .join(";"),
        )
        .arg(
            data_2
                .into_iter()
                .map(|(k, v)| format!("{},{}", k, v))
                .collect::<Vec<String>>()
                .join(";"),
        )
        .arg(metric_1)
        .arg(metric_2)
        .arg(&out_path);
    cmd.stdout(Stdio::inherit());
    cmd.spawn().unwrap().wait().unwrap();

    Ok(out_path)
}

#[cfg(test)]
mod test_compare_stat_2d {
    use std::{
        fs::{self, remove_file},
        path::PathBuf,
    };

    use super::compare_stat_2d;

    /// test for compare_stat
    ///
    /// Step1. compare stats of metric `instructions` in `test/compare_stat/stat`.
    ///
    /// Step2. plot and check the compare result.
    ///
    /// Step3. clean up.
    #[test]
    fn test_compare_stat_2d() {
        let stat_1 = PathBuf::from("test/compare_stat/stat/merged_statstics_current.json");
        let stat_2 = PathBuf::from("test/compare_stat/stat/merged_statstics_old.json");
        let metric_1 = String::from("instructions:u");
        let metric_2 = String::from("wall-time");
        let out_path = PathBuf::from("test/compare_stat/compare_stat_2d.jpeg");

        assert_eq!(
            out_path.clone(),
            compare_stat_2d(&stat_1, &stat_2, &metric_1, &metric_2, out_path.clone()).unwrap()
        );

        fs::metadata(&out_path).unwrap();
        remove_file(out_path).unwrap();
    }
}
