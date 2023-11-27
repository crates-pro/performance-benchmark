use std::path::PathBuf;

use crate::{benchmark::profile::Profiles, toolchain::LocalToolchain};

pub(crate) fn entry(
    ltc: LocalToolchain,
    profiles: Profiles,
    benchmark_dir: PathBuf,
    out_dir: PathBuf,
) -> anyhow::Result<()> {
    unimplemented!();
}
