use std::{
    fs::read_dir,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use crate::{benchmark::profile::Profile, execute::Stats, toolchain::Compiler};

use super::BinaryProcess;

pub struct BinaryPackageProcess<'a> {
    pub compiler: Compiler<'a>,
    pub processor_name: String,
    pub cwd: &'a Path,
    pub profile: Profile,
    pub incremental: bool,
    pub manifest_path: String,
    pub cargo_args: Vec<String>,
    pub rustc_args: Vec<String>,
    pub touch_file: Option<String>,
    pub packages: Vec<String>,
}

impl<'a> BinaryProcess for BinaryPackageProcess<'a> {
    fn run_rustc(&self) -> anyhow::Result<Option<Stats>> {
        for package in &self.packages {
            let mut cmd = Command::new(Path::new(self.compiler.cargo));
            cmd.current_dir(self.cwd)
                .env("RUSTC", &*self.compiler.rustc)
                .env("CARGO_INCREMENTAL", "0")
                .env("RUSTC_BOOTSTRAP", "1")
                .arg("rustc")
                .arg("--manifest-path")
                .arg(&self.manifest_path)
                .arg("--profile")
                .arg(self.profile.to_string())
                .arg("--package")
                .arg(package)
                .args(&self.cargo_args);

            cmd.stdout(Stdio::null()).stderr(Stdio::null());

            cmd.spawn()?
                .wait()
                .expect(format!("Fail to compile {}.", self.processor_name).as_str());
        }

        let mut binary_size = 0;

        let mut target_dir = PathBuf::from(self.cwd);
        if self.manifest_path.contains('/') {
            let segment = self.manifest_path.split('/');
            let toml = segment.clone().last().unwrap();
            segment.for_each(|s| {
                if s != toml {
                    target_dir = target_dir.join(s);
                }
            });
        }
        target_dir = target_dir.join("target").join(self.profile.to_string());

        let dir = read_dir(target_dir)?;
        for entry in dir {
            let entry = entry?;
            if !self.is_filtered_file_name(entry.file_name()) {
                binary_size += entry.metadata()?.len();
            }
        }

        if binary_size == 0 {
            Ok(None)
        } else {
            let mut stats = Stats::new();
            stats.stats.insert(
                "binary_size".to_string(),
                binary_size as f64 / (1 << 20) as f64,
            );

            Ok(Some(stats))
        }
    }
}
