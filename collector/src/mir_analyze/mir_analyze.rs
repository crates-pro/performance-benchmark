use std::path::PathBuf;

use crate::{
    benchmark::benchmark::Benchamrk, compile_time::discover_benchmark_suit,
    toolchain::LocalToolchain,
};

use super::{
    analyze::count::count_mir, data::analyze_data::Data, mir_generate::generate_mir, mirs::mir::MIR,
};

/// Get all benchmark directories from `benchmark_dir` and
/// generate mir file for each benchmark. Then do analysis
/// on the generated mir file.
pub(crate) fn entry(
    ltc: LocalToolchain,
    benchmarks_dir: PathBuf,
    out_dir: PathBuf,
) -> anyhow::Result<()> {
    let mut data = Data::new();

    for benchmark in &discover_benchmark_suit(&benchmarks_dir)? {
        let mirs = generate_mir(benchmark, &ltc)?;

        do_analyze(mirs, benchmark, &mut data)?;
    }

    data.write_all(out_dir.as_path())?;

    Ok(())
}

fn do_analyze(mirs: Vec<MIR>, benchmark: &Benchamrk, data: &mut Data) -> anyhow::Result<()> {
    data.add_mir_count(&benchmark.name, count_mir(&mirs)?);

    Ok(())
}
