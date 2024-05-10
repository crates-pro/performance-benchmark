use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

use super::data::{calculate_change_rate, read_data, ChangeRate};

pub fn compare_data(
    data_a: &PathBuf,
    data_b: &PathBuf,
    metric: &String,
    out_path: PathBuf,
) -> anyhow::Result<PathBuf> {
    let data_a = read_data(data_a, metric)?;
    let data_b = read_data(data_b, metric)?;

    // Calculate change rate of stats_a on stats_b
    let change_rate = calculate_change_rate(&data_a, &data_b);

    plot_boxplot_compare(&change_rate, out_path, metric)
}

fn plot_boxplot_compare(
    data: &ChangeRate,
    out_path: PathBuf,
    metric: &String,
) -> anyhow::Result<PathBuf> {
    let mut cmd = Command::new("python");
    cmd.arg("src/plotters/plotter_cmp_boxplot.py")
        .arg(
            data.into_iter()
                .map(|(k, v)| {
                    format!(
                        "{}:{}",
                        k,
                        v.into_iter()
                            .map(|d| d.to_string())
                            .collect::<Vec<String>>()
                            .join(",")
                    )
                })
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

    use super::compare_data;

    /// test for compare_stat
    ///
    /// Step1. compare stats of metric `instructions` in `test/compare_stat/stat`.
    ///
    /// Step2. plot and check the compare result.
    ///
    /// Step3. clean up.
    #[test]
    fn test_compare_data() {
        let stat_1 = PathBuf::from("test/compare_data/stat/merged-runtime_data_current.json");
        let stat_2 = PathBuf::from("test/compare_data/stat/merged-runtime_data_old.json");
        let metric = String::from("wall-time");
        let out_path = PathBuf::from("test/compare_data/compare_data.jpeg");

        assert_eq!(
            out_path.clone(),
            compare_data(&stat_1, &stat_2, &metric, out_path.clone()).unwrap()
        );

        fs::metadata(&out_path).unwrap();
        remove_file(out_path).unwrap();
    }
}
