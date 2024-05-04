use std::{
    collections::HashMap,
    fmt::Display,
    fs::{read_dir, File},
    io::BufWriter,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context};

use super::{
    data::table_data::TableDatas,
    mir::{
        function_pattern::{count_closure, count_pure_function, higher_function},
        io_function::count_io_metrics,
        oop_pattern::{dfc, lof, pbf, wms_noc_rfs},
        parallelism::{count_parallelism_metrics, count_parallelism_strcut},
        reader::parse_mir,
    },
};

/// Get all benchmark directories from `benchmark_dir` and
/// generate mir file for each benchmark. Then do analysis
/// on the generated mir file.
pub(crate) fn entry(mir_dir: PathBuf, out_path: PathBuf) -> anyhow::Result<PathBuf> {
    let mir_suit = discover_mir_suit(mir_dir.as_path())?;
    println!(
        "Find mir_suit:\n{}",
        mir_suit
            .iter()
            .map(|m| format!("+{}", m.to_string()))
            .collect::<String>()
    );

    let result = do_analyze(&mir_suit);

    serde_json::to_writer(BufWriter::new(File::create(&out_path)?), &result)?;

    Ok(out_path)
}

fn discover_mir_suit(dir: &Path) -> anyhow::Result<Vec<MirSource>> {
    let mut mir_suit = vec![];

    for entry in read_dir(dir)
        .with_context(|| format!("failed to list benchmark dir '{}'", dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        let name = match entry.file_name().into_string() {
            core::result::Result::Ok(s) => s,
            Err(e) => bail!("non-utf8 benchmark name: {:?}", e),
        };
        if !entry.file_type()?.is_dir() {
            println!("File '{}' ignored", name);
            continue;
        }
        mir_suit.push(MirSource::from(path.as_path()));
    }
    if mir_suit.is_empty() {
        eprintln!("Error: no benchmark found in '{}'", dir.display());
    }
    Ok(mir_suit)
}

fn do_analyze(mir_suit: &Vec<MirSource>) -> TableDatas<String, String, i32> {
    let mut table_data = HashMap::new();

    let metrics = vec![
        "io_call",
        "parallelism_call",
        "parallelism_struct",
        "oop_lof",
        "oop_dfc",
        "oop_pbf",
        "oop_wms",
        "oop_rfs",
        "oop_noc",
        "pure_function",
        "closure",
        "higher_function",
    ];

    metrics.iter().for_each(|s| {
        table_data.insert(s.to_string(), HashMap::new());
    });

    for mir_source in mir_suit {
        println!("Analyzing {}...", mir_source.name);

        metrics.iter().for_each(|m| {
            table_data
                .get_mut(&m.to_string())
                .unwrap()
                .insert(mir_source.name.clone(), 0);
        });

        for mir_file in &mir_source.mirs {
            let mir = &parse_mir(File::open(mir_file.clone()).unwrap()).unwrap();
            let wms_noc_rfs = wms_noc_rfs(mir);
            vec![
                ("io_call".to_string(), count_io_metrics(mir)),
                (
                    "parallelism_call".to_string(),
                    count_parallelism_metrics(mir),
                ),
                (
                    "parallelism_struct".to_string(),
                    count_parallelism_strcut(mir),
                ),
                ("oop_lof".to_string(), lof(mir)),
                ("oop_dfc".to_string(), dfc(mir)),
                ("oop_pbf".to_string(), pbf(mir)),
                ("oop_wms".to_string(), wms_noc_rfs[0]),
                ("oop_rfs".to_string(), wms_noc_rfs[1]),
                ("oop_noc".to_string(), wms_noc_rfs[2]),
                ("pure_function".to_string(), count_pure_function(mir)),
                ("closure".to_string(), count_closure(mir)),
                ("higher_function".to_string(), higher_function(mir)),
            ]
            .into_iter()
            .for_each(|(k, v)| {
                *table_data
                    .get_mut(&k)
                    .unwrap()
                    .get_mut(&mir_source.name)
                    .unwrap() += v;
            });
        }
    }

    table_data
}
struct MirSource {
    pub name: String,
    pub path: PathBuf,
    pub mirs: Vec<PathBuf>,
}

impl From<&Path> for MirSource {
    fn from(d: &Path) -> Self {
        assert!(d.is_dir());
        let mut mirs = vec![];

        for entry in read_dir(d).unwrap() {
            let entry = entry.unwrap();
            if entry.path().is_file() && entry.file_name().to_str().unwrap().ends_with(".mir") {
                mirs.push(entry.path())
            }
        }

        Self {
            name: d.file_name().unwrap().to_str().unwrap().to_string(),
            path: d.to_path_buf(),
            mirs,
        }
    }
}

impl Display for MirSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}\n{}",
            self.name,
            self.path.to_str().unwrap(),
            self.mirs
                .iter()
                .map(|m| format!("\t{}\n", m.to_str().unwrap()))
                .collect::<String>()
        )
    }
}
