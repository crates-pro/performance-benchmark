use std::{
    fs::{create_dir_all, File},
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use crate::benchmark::profile::Profiles;
use crate::statistics::compile_time_stat::CompileTimeResultSet;

pub fn read_compile_time_json(path: &PathBuf) -> anyhow::Result<CompileTimeResultSet> {
    let file = File::open(path)?;
    let file = BufReader::new(file);
    let res = serde_json::from_reader(file)?;
    Ok(res)
}

pub fn create_compile_time_csv(
    out: &PathBuf,
    rustc_id: &str,
    data: &CompileTimeResultSet,
    profiles: &Profiles,
) -> anyhow::Result<()> {
    if data.get_ref_results().len() <= 0 {
        eprintln!("{} has empty data set, will be skipped.", out.display());
        return Ok(());
    }
    create_dir_all(out)?;

    let results = data.get_ref_results();

    let header: Vec<String> = results.iter().map(|r| r.get_benchmark()).collect();

    let metrics: Vec<&String> = results
        .first()
        .unwrap()
        .get_stats_ref_by_profile(profiles.profiles.first().unwrap())
        .first()
        .unwrap()
        .stats
        .iter()
        .map(|element| element.0)
        .collect();

    for metric in metrics {
        for profile in &profiles.profiles {
            let file = File::create(out.join(
                rustc_id.to_string() + "_" + profile.to_string().as_str() + "_" + metric + ".csv",
            ))?;
            let mut csv_writer = csv::Writer::from_writer(BufWriter::new(file));

            csv_writer.write_record(&header)?;

            for i in 0..results.first().unwrap().get_iterations() {
                let datas: Vec<String> = results
                    .iter()
                    .map(|r| {
                        // eprintln!("{}[{}] {} {}", r.get_benchmark(), i, rustc_id, metric);
                        if i >= r.get_stats_ref_by_profile(profile).len() {
                            return String::from("0");
                        }
                        match r.get_stats_ref_by_profile(profile)[i].stats.get(metric) {
                            Some(f) => f.to_string(),
                            None => {
                                eprintln!("Corrupted output data to write {}!", out.display());
                                // corrupted = true;
                                String::from("0")
                            }
                        }
                    })
                    .collect();

                csv_writer.write_record(&datas)?;
            }
        }
    }

    Ok(())
}
