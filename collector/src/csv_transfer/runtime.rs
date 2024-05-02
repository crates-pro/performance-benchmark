use std::{
    fs::{create_dir_all, File},
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use crate::statistics::runtime_stat::RuntimeResultVec;

pub fn read_runtime_json(path: &PathBuf) -> anyhow::Result<RuntimeResultVec> {
    let file = File::open(path)?;
    let file = BufReader::new(file);
    let res = serde_json::from_reader(file)?;
    Ok(res)
}

pub fn create_runtime_csv(
    out: &PathBuf,
    rustc_id: &str,
    data: &RuntimeResultVec,
) -> anyhow::Result<()> {
    if data.0.len() <= 0 {
        eprintln!("{} has empty data set, will be skipped.", out.display());
        return Ok(());
    }
    create_dir_all(out)?;

    let results = &data.0;

    let header: Vec<String> = results.iter().map(|r| r.name.clone()).collect();

    let metrics: Vec<String> = results
        .first()
        .unwrap()
        .stats
        .first()
        .unwrap()
        .iter()
        .map(|stat| stat.0.to_string())
        .collect();

    for metric in metrics {
        let file =
            File::create(out.join(rustc_id.to_string() + "_release" + "_" + &metric + ".csv"))?;

        let mut csv_writer = csv::Writer::from_writer(BufWriter::new(file));

        csv_writer.write_record(&header)?;

        for i in 0..results.first().unwrap().stats.len() {
            let datas: Vec<String> = results
                .iter()
                .map(|r| match r.stats[i].stats.get(&metric) {
                    Some(f) => f.to_string(),
                    None => panic!("Corrupted output data!"),
                })
                .collect();

            csv_writer.write_record(&datas)?;
        }
    }
    Ok(())
}
