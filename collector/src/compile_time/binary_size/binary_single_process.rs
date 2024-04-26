use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use crate::{benchmark::profile::Profile, execute::Stats, toolchain::Compiler};

use super::{BinaryProcess, BINARY_SIZE_LABEL};

pub struct BinarySingleProcess<'a> {
    pub compiler: Compiler<'a>,
    pub processor_name: String,
    pub cwd: &'a Path,
    pub profile: Profile,
    pub incremental: bool,
    pub manifest_path: String,
    pub cargo_args: Vec<String>,
    pub rustc_args: Vec<String>,
    pub touch_file: Option<String>,
    pub target_path: Option<PathBuf>,
}

impl<'a> BinaryProcess for BinarySingleProcess<'a> {
    fn run_rustc(&self) -> anyhow::Result<Option<Stats>> {
        let mut cmd = Command::new(Path::new(self.compiler.cargo));

        cmd.current_dir(self.cwd)
            .env("RUSTC", &*self.compiler.rustc)
            .env("CARGO_INCREMENTAL", "0")
            .env("RUSTC_BOOTSTRAP", "1")
            .arg("rustc")
            .arg("--manifest-path")
            .arg(&self.manifest_path)
            .args(&self.cargo_args);

        match self.profile {
            Profile::Check => {
                cmd.arg("--profile").arg("check");
            }
            Profile::Debug => (),
            Profile::Doc => unimplemented!(),
            Profile::Release => {
                cmd.arg("--release");
            }
        }

        cmd.stdout(Stdio::null()).stderr(Stdio::null());

        cmd.spawn()?
            .wait()
            .expect(format!("Fail to compile {}.", self.processor_name).as_str());

        let target_dir = if let Some(target_path) = &self.target_path {
            PathBuf::from(self.cwd)
                .join(target_path)
                .join("target")
                .join(self.profile.to_string())
        } else {
            PathBuf::from(self.cwd)
                .join("target")
                .join(self.profile.to_string())
        };

        let binary_size = self.get_binary_size(&target_dir).unwrap();

        if binary_size == 0 {
            Result::Err(anyhow::Error::msg(format!(
                "Build target not found in `{}`.",
                target_dir.to_str().unwrap()
            )))
        } else {
            let mut stats = Stats::new();
            stats.stats.insert(
                BINARY_SIZE_LABEL.to_string(),
                binary_size as f64 / (1 << 20) as f64,
            );

            Ok(Some(stats))
        }
    }
}
