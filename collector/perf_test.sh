#!/bin/bash
home_path="/media/workstation/cc36671e-05f5-48bd-9b40-1b1c1f396fae/home/fxl"

collector="${home_path}/Rust_Performance_Benchmark/collector/target/release/collector"

cargo_path="/home/workstation/.rustup/toolchains/1.56-x86_64-unknown-linux-gnu/bin/cargo"

# out_dir="${home_path}/perf_out/${type}/old"

iter=1

# cargo build --release

time="runtime"

PR="Remove_visible_path_calculation_from_allowed_deprecation_lint"

for version in "4be574e6c91_old" # "15483ccf9d0_cur"
do
    rustc_version="${PR}/${version}"
    rustc_path="${home_path}/rustc/targets/${rustc_version}/bin/rustc"
    for type in "db" # "parallel" "system_programing"
    do
        bench_dir="${home_path}/benchmarks_real/compile_time/${type}"
        out_dir="/media/workstation/My Passport/fxl/perf_out/${time}/${rustc_version}/${type}"

        # echo ${collector} bench_local \
        # --bench-dir ${bench_dir} \
        # --cargo ${cargo_path} \
        # --flamegraph 0 --iterations ${iter} \
        # --out-dir "${out_dir}" \
        # --perf-tool perf-record \
        # --profile debug ${rustc_path}

        ${collector} bench_runtime_local \
        --bench-dir ${bench_dir} \
        --cargo ${cargo_path} \
        --flamegraph 0 --iterations ${iter} \
        --out-dir "${out_dir}" \
        --perf-tool perf-record \
        ${rustc_path}
    done
done

echo "Nju@957!" | sudo -S rm -rf /tmp/.tmp*

echo -e "\nDone!"
