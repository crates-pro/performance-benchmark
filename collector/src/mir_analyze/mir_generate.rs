use std::{
    fs::{copy, create_dir_all, read_dir},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use crate::{benchmark::benchmark::Benchamrk, toolchain::LocalToolchain};

pub fn generate_mir(
    benchmark: &Benchamrk,
    ltc: &LocalToolchain,
    out_path: &Path,
) -> anyhow::Result<Vec<PathBuf>> {
    println!("Generating MIR for `{}`", benchmark.name);

    let out_dir = out_path.join(&benchmark.name);
    create_dir_all(&out_dir)?;

    match benchmark
        .config
        .compile_time_type
        .clone()
        .unwrap_or_default()
    {
        crate::benchmark::benchmark::CompileTimeType::Single => {
            single_genrate_mir(benchmark, ltc, out_dir.as_path())
        }
        crate::benchmark::benchmark::CompileTimeType::Packages => {
            package_generate_mir(benchmark, ltc, out_dir.as_path())
        }
    }
}

fn single_genrate_mir(
    benchmark: &Benchamrk,
    ltc: &LocalToolchain,
    out_path: &Path,
) -> anyhow::Result<Vec<PathBuf>> {
    let tmp_dir = benchmark.make_temp_dir(&benchmark.path)?;

    let mut cmd = Command::new(Path::new(&ltc.cargo));
    cmd
        // Not all cargo invocations (e.g. `cargo clean`) need all of these
        // env vars set, but it doesn't hurt to have them.
        .env("RUSTC", &ltc.rustc)
        // We separately pass -Cincremental to the leaf crate --
        // CARGO_INCREMENTAL is cached separately for both the leaf crate
        // and any in-tree dependencies, and we don't want that; it wastes
        // time.
        .env("CARGO_INCREMENTAL", "0")
        // We need to use -Z flags (for example, to force enable ICH
        // verification) so unconditionally enable unstable features, even
        // on stable compilers.
        .env("RUSTC_BOOTSTRAP", "1")
        .current_dir(tmp_dir.path())
        .arg("rustc")
        .arg("--release")
        .arg("--manifest-path")
        .arg(
            &benchmark
                .config
                .cargo_toml
                .clone()
                .unwrap_or_else(|| String::from("Cargo.toml")),
        );
    if let Some(opts) = &benchmark.config.cargo_opts {
        cmd.args(opts.split(" ").collect::<Vec<&str>>());
    }
    cmd.arg("--").arg("--emit=mir");

    cmd.stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect(format!("Fail to compile {}.", benchmark.name).as_str())
        .wait()?;

    // find mir file
    let mut mir_files = vec![];
    for entry in read_dir(PathBuf::from(
        tmp_dir.path().join("target").join("release").join("deps"),
    ))? {
        let entry = entry?;

        if let Some(file_name) = entry.file_name().to_str() {
            if file_name.ends_with(".mir") {
                let dst_path = out_path.join(entry.file_name());
                copy(entry.path(), &dst_path)?;
                mir_files.push(dst_path);
            }
        }
    }
    Ok(mir_files)
}

fn package_generate_mir(
    benchmark: &Benchamrk,
    ltc: &LocalToolchain,
    out_path: &Path,
) -> anyhow::Result<Vec<PathBuf>> {
    let tmp_dir = benchmark.make_temp_dir(&benchmark.path)?;

    for package in benchmark.config.packages.clone().unwrap() {
        let mut cmd = Command::new(Path::new(&ltc.cargo));
        cmd
            // Not all cargo invocations (e.g. `cargo clean`) need all of these
            // env vars set, but it doesn't hurt to have them.
            .env("RUSTC", &ltc.rustc)
            // We need to use -Z flags (for example, to force enable ICH
            // verification) so unconditionally enable unstable features, even
            // on stable compilers.
            .env("RUSTC_BOOTSTRAP", "1")
            .current_dir(tmp_dir.path())
            .arg("rustc")
            .arg("--release")
            .arg("--manifest-path")
            .arg(
                &benchmark
                    .config
                    .cargo_toml
                    .clone()
                    .unwrap_or_else(|| String::from("Cargo.toml")),
            )
            .arg("--package")
            .arg(package);
        if let Some(opts) = &benchmark.config.cargo_opts {
            cmd.args(opts.split(" ").collect::<Vec<&str>>());
        }
        cmd.arg("--").arg("--emit=mir");

        cmd.stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect(format!("Fail to compile {}.", benchmark.name).as_str())
            .wait()?;
    }

    let mut mir_files = vec![];
    for entry in read_dir(PathBuf::from(
        tmp_dir.path().join("target").join("release").join("deps"),
    ))? {
        let entry = entry?;

        if let Some(file_name) = entry.file_name().to_str() {
            if file_name.ends_with(".mir") {
                let dst_path = out_path.join(entry.file_name());
                copy(entry.path(), &dst_path)?;
                mir_files.push(dst_path);
            }
        }
    }

    Ok(mir_files)
}

#[cfg(test)]
mod test {
    use std::{
        fs::remove_dir_all,
        path::{Path, PathBuf},
    };

    use crate::{
        benchmark::benchmark::{Benchamrk, BenchmarkConfig},
        toolchain::LocalToolchain,
    };

    use super::generate_mir;

    /// Test mir generation on a single benchmark program.
    #[test]
    fn test_generate_mir() {
        let benchmark = Benchamrk {
            name: "condvar".to_string(),
            path: PathBuf::from("test/mir_analyze/run_analyze/benchmarks/condvar"),
            config: BenchmarkConfig {
                cargo_opts: None,
                cargo_rustc_opts: None,
                cargo_toml: None,
                compile_time_type: None,
                packages: None,
                runtime_test_type: None,
                example_lst: None,
                runtime_args: None,
                touch_file: None,
                disabled: false,
                runs: 0,
                target_path: None,
                runtime_cargo_toml: None,
                runtime_test_packages: None,
            },
        };
        let out_dir = Path::new("test/mir_analyze/run_analyze/out");

        let v = generate_mir(
            &benchmark,
            &LocalToolchain {
                rustc: PathBuf::from("rustc"),
                cargo: PathBuf::from("cargo"),
                flame_graph: PathBuf::from(""),
                id: 0.to_string(),
            },
            &out_dir,
        )
        .unwrap();

        assert!(v.len() > 0);
        v.into_iter().for_each(|f| {
            f.metadata().unwrap();
        });

        remove_dir_all(out_dir.join("condvar")).unwrap();
    }

    /// Test mir generation when handling benchmark programs made of several packages.
    #[test]
    fn test_generate_mir_packaged_benchmark() {
        let benchmark = Benchamrk::new(
            "muti-package".to_string(),
            PathBuf::from("test/mir_analyze/run_analyze/benchmarks/muti-package"),
        )
        .unwrap();

        let out_dir = Path::new("test/mir_analyze/run_analyze/out");

        let v = generate_mir(
            &benchmark,
            &LocalToolchain {
                rustc: PathBuf::from("rustc"),
                cargo: PathBuf::from("cargo"),
                flame_graph: PathBuf::from(""),
                id: 0.to_string(),
            },
            out_dir,
        )
        .unwrap();

        assert!(v.len() > 0);
        v.into_iter().for_each(|f| {
            f.metadata().unwrap();
        });
        remove_dir_all(out_dir.join("muti-package")).unwrap();
    }
}
