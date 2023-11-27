use std::{
    fs::{self, File},
    io::BufWriter,
    path::PathBuf,
};

use self::miner::{get_miners, output_csv, SynthesisInfo};

mod derive_debug_miner;
mod miner;

/// `run_miners` will excecute all the miners registered in `get_miners()`
/// and write the collected data to `out_path` in csv form.
/// `bench_dir` is a directory that contains several projects.
pub(crate) fn run_miners(bench_dir: PathBuf, out_path: PathBuf) {
    let mut synthesis_info = SynthesisInfo::new();

    if let Ok(dir) = fs::read_dir(&bench_dir) {
        for entry in dir {
            if let Ok(e) = entry {
                let proj_name = String::from(e.file_name().to_str().unwrap());

                synthesis_info.insert(
                    e.file_name().to_str().unwrap().to_string(),
                    get_miners()
                        .iter()
                        .map(|miner| match miner.mine_from_project(&e.path()) {
                            Ok(i) => i,
                            Err(e) => {
                                eprintln!("Fail to mine from project {}.\n{}", proj_name, e);
                                (miner.morpheme_name(), -1)
                            }
                        })
                        .collect(),
                );
            }
        }
    } else {
        eprintln!("Fail to read bench_dir {:?}.", bench_dir);
        return;
    }

    if let Ok(file) = File::create(&out_path) {
        let writer = BufWriter::new(file);
        output_csv(synthesis_info, writer);
    } else {
        eprintln!("Fail to create output file {:?}", out_path);
    }
}
