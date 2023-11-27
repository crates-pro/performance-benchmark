#!/bin/bash
home_path="/media/workstation/cc36671e-05f5-48bd-9b40-1b1c1f396fae/home/fxl"

collector="${home_path}/Rust_Performance_Benchmark/collector/target/release/collector"

time="runtime"

PR="Replace_dominators_algorithm_with_simple_Lengauer-Tarjan"

data_dir="/media/workstation/My Passport/fxl/perf_out/${time}/${PR}"

filter="${home_path}/Rust_Performance_Benchmark/collector/filter.json"

out_dir="/media/workstation/My Passport/fxl/perf_analyze/${time}/${PR}"

# export RAYON_NUM_THREADS=10

${collector} analyze_local \
--data-dir "${data_dir}" \
--event-filter-file "${filter}" \
--out-dir "${out_dir}"
