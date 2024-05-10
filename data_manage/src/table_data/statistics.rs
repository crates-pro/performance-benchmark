use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use collector::mir_analyze::data::table_data::TableDatas;

pub fn calculate_table_stats(
    table_data_path: &PathBuf,
    out_path: PathBuf,
) -> anyhow::Result<PathBuf> {
    let table_data: TableDatas<String, String, f64> =
        serde_json::from_reader(BufReader::new(File::open(table_data_path)?))?;
    serde_json::to_writer(
        BufWriter::new(File::create(&out_path)?),
        &generate_table_statistics(&table_data),
    )?;

    Ok(out_path)
}

fn generate_table_statistics(
    table_data: &TableDatas<String, String, f64>,
) -> TableDatas<String, String, f64> {
    let mut stats = TableDatas::new();
    table_data.keys().into_iter().for_each(|k| {
        stats.insert(k.clone(), HashMap::new());
    });

    table_data.into_iter().for_each(|(metric, data)| {
        let data: Vec<f64> = data.into_iter().map(|(_, d)| *d).collect();
        let stat = stats.get_mut(metric).unwrap();

        stat.insert("max".to_string(), {
            let mut max = 0.;
            data.iter().for_each(|d| {
                if *d > max {
                    max = *d
                }
            });
            max
        });

        stat.insert("min".to_string(), {
            let mut min = f64::MAX;
            data.iter().for_each(|d| {
                if *d < min {
                    min = *d
                }
            });
            min
        });

        stat.insert("geomean".to_string(), {
            let mut psum = 1.;
            data.iter().for_each(|d| psum *= *d);
            psum.powf(1. / data.len() as f64)
        });

        stat.insert("arithmetic-mean".to_string(), {
            data.iter().sum::<f64>() / data.len() as f64
        });
    });

    stats
}
