use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, Read},
    path::{Path, PathBuf},
};

pub fn read_dependencies(path: &Path) -> anyhow::Result<Vec<Dependency>> {
    let mut reader = BufReader::new(File::open(path)?);
    let mut dependencies = vec![];

    while let Some(d) = Dependency::from_reader(&mut reader) {
        dependencies.push(d)
    }
    Ok(dependencies)
}

/// Cargo dependecies
#[derive(Debug)]
pub struct Dependency {
    name: String,
    version: String,
}

impl Dependency {
    pub fn path(&self, base: &PathBuf) -> PathBuf {
        base.join(format!("{}-{}", self.name, self.version))
    }

    fn from_reader<T: Read>(reader: &mut BufReader<T>) -> Option<Self> {
        let mut buf = String::new();
        let mut name = String::new();
        let mut version = String::new();

        let mut state = ReaderState::default();
        while let Ok(s) = reader.read_line(&mut buf) {
            if s == 0 {
                break;
            }

            match state {
                ReaderState::Init => {
                    if buf.eq("[[package]]\n") {
                        state = ReaderState::ReadName;
                    }
                }
                ReaderState::ReadName => {
                    let name_specifier = "name = \"";
                    if buf.starts_with(name_specifier) {
                        name = buf.split_off(name_specifier.len());
                        let _ = name.split_off(name.len() - 2);
                        state = ReaderState::ReadVersion;
                    } else {
                        state = ReaderState::Final;
                    }
                }
                ReaderState::ReadVersion => {
                    let version_specifier = "version = \"";
                    if buf.starts_with(version_specifier) {
                        version = buf.split_off(version_specifier.len());
                        let _ = version.split_off(version.len() - 2);
                    }
                    state = ReaderState::Final;
                }
                ReaderState::Final => {
                    if buf.eq("\n") {
                        break;
                    }
                }
            }

            buf.clear();
        }

        let v = Self { name, version };
        if v.is_valid() {
            Some(v)
        } else {
            None
        }
    }

    fn is_valid(&self) -> bool {
        !(self.name.is_empty() || self.version.is_empty())
    }
}

impl Display for Dependency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Dependency {{name: \"{}\", version: \"{}\"}}",
            self.name, self.version
        ))
    }
}

enum ReaderState {
    Init,
    ReadName,
    ReadVersion,
    Final,
}

impl Default for ReaderState {
    fn default() -> Self {
        Self::Init
    }
}

#[cfg(test)]
mod test_cargo_lock_reader {
    use std::{fs::File, io::BufReader, path::PathBuf};

    use super::Dependency;

    impl PartialEq for Dependency {
        fn eq(&self, other: &Self) -> bool {
            self.name.eq(&other.name) && self.version.eq(&other.version)
        }
    }

    #[test]
    fn test_cargo_lock_reader() {
        let lock_file = PathBuf::from("test/src_code_analyze/cargo_lock_reader/cargolock.txt");
        let mut r = BufReader::new(File::open(lock_file).unwrap());

        let mut dependencies = vec![
            Dependency {
                name: "adler".to_string(),
                version: "1.0.2".to_string(),
            },
            Dependency {
                name: "aho-corasick".to_string(),
                version: "1.1.2".to_string(),
            },
            Dependency {
                name: "android-tzdata".to_string(),
                version: "0.1.1".to_string(),
            },
            Dependency {
                name: "android_system_properties".to_string(),
                version: "0.1.5".to_string(),
            },
            Dependency {
                name: "anyhow".to_string(),
                version: "1.0.75".to_string(),
            },
            Dependency {
                name: "approx".to_string(),
                version: "0.5.1".to_string(),
            },
            Dependency {
                name: "ascii-canvas".to_string(),
                version: "3.0.0".to_string(),
            },
            Dependency {
                name: "atty".to_string(),
                version: "0.2.14".to_string(),
            },
            Dependency {
                name: "autocfg".to_string(),
                version: "1.1.0".to_string(),
            },
            Dependency {
                name: "bit-set".to_string(),
                version: "0.5.3".to_string(),
            },
            Dependency {
                name: "bit-vec".to_string(),
                version: "0.6.3".to_string(),
            },
            Dependency {
                name: "bitflags".to_string(),
                version: "1.3.2".to_string(),
            },
            Dependency {
                name: "bitflags".to_string(),
                version: "2.4.1".to_string(),
            },
            Dependency {
                name: "bumpalo".to_string(),
                version: "3.15.3".to_string(),
            },
            Dependency {
                name: "bytemuck".to_string(),
                version: "1.14.3".to_string(),
            },
            Dependency {
                name: "byteorder".to_string(),
                version: "1.5.0".to_string(),
            },
            Dependency {
                name: "camino".to_string(),
                version: "1.1.6".to_string(),
            },
            Dependency {
                name: "cargo-platform".to_string(),
                version: "0.1.5".to_string(),
            },
            Dependency {
                name: "cargo_metadata".to_string(),
                version: "0.18.1".to_string(),
            },
            Dependency {
                name: "cc".to_string(),
                version: "1.0.88".to_string(),
            },
            Dependency {
                name: "cfg-if".to_string(),
                version: "1.0.0".to_string(),
            },
            Dependency {
                name: "chrono".to_string(),
                version: "0.4.34".to_string(),
            },
            Dependency {
                name: "clap".to_string(),
                version: "3.2.25".to_string(),
            },
            Dependency {
                name: "clap_derive".to_string(),
                version: "3.2.25".to_string(),
            },
            Dependency {
                name: "clap_lex".to_string(),
                version: "0.2.4".to_string(),
            },
            Dependency {
                name: "collector".to_string(),
                version: "0.1.0".to_string(),
            },
            Dependency {
                name: "color_quant".to_string(),
                version: "1.1.0".to_string(),
            },
            Dependency {
                name: "const-cstr".to_string(),
                version: "0.3.0".to_string(),
            },
            Dependency {
                name: "core-foundation".to_string(),
                version: "0.9.4".to_string(),
            },
            Dependency {
                name: "core-foundation-sys".to_string(),
                version: "0.8.6".to_string(),
            },
            Dependency {
                name: "core-graphics".to_string(),
                version: "0.22.3".to_string(),
            },
            Dependency {
                name: "core-graphics-types".to_string(),
                version: "0.1.3".to_string(),
            },
            Dependency {
                name: "core-text".to_string(),
                version: "19.2.0".to_string(),
            },
            Dependency {
                name: "crc32fast".to_string(),
                version: "1.4.0".to_string(),
            },
            Dependency {
                name: "crossbeam-deque".to_string(),
                version: "0.8.3".to_string(),
            },
            Dependency {
                name: "crossbeam-epoch".to_string(),
                version: "0.9.15".to_string(),
            },
            Dependency {
                name: "crossbeam-utils".to_string(),
                version: "0.8.16".to_string(),
            },
            Dependency {
                name: "crunchy".to_string(),
                version: "0.2.2".to_string(),
            },
            Dependency {
                name: "csv".to_string(),
                version: "1.3.0".to_string(),
            },
            Dependency {
                name: "csv-core".to_string(),
                version: "0.1.11".to_string(),
            },
            Dependency {
                name: "dirs-next".to_string(),
                version: "2.0.0".to_string(),
            },
            Dependency {
                name: "dirs-sys-next".to_string(),
                version: "0.1.2".to_string(),
            },
            Dependency {
                name: "dlib".to_string(),
                version: "0.5.2".to_string(),
            },
            Dependency {
                name: "dwrote".to_string(),
                version: "0.11.0".to_string(),
            },
            Dependency {
                name: "either".to_string(),
                version: "1.11.0".to_string(),
            },
            Dependency {
                name: "ena".to_string(),
                version: "0.14.2".to_string(),
            },
            Dependency {
                name: "env_logger".to_string(),
                version: "0.10.1".to_string(),
            },
            Dependency {
                name: "equivalent".to_string(),
                version: "1.0.1".to_string(),
            },
            Dependency {
                name: "errno".to_string(),
                version: "0.3.8".to_string(),
            },
            Dependency {
                name: "fastrand".to_string(),
                version: "2.0.1".to_string(),
            },
            Dependency {
                name: "fdeflate".to_string(),
                version: "0.3.4".to_string(),
            },
            Dependency {
                name: "filetime".to_string(),
                version: "0.2.22".to_string(),
            },
            Dependency {
                name: "fixedbitset".to_string(),
                version: "0.4.2".to_string(),
            },
            Dependency {
                name: "flate2".to_string(),
                version: "1.0.29".to_string(),
            },
            Dependency {
                name: "float-ord".to_string(),
                version: "0.2.0".to_string(),
            },
            Dependency {
                name: "font-kit".to_string(),
                version: "0.11.0".to_string(),
            },
            Dependency {
                name: "foreign-types".to_string(),
                version: "0.3.2".to_string(),
            },
            Dependency {
                name: "foreign-types-shared".to_string(),
                version: "0.1.1".to_string(),
            },
            Dependency {
                name: "freetype".to_string(),
                version: "0.7.2".to_string(),
            },
            Dependency {
                name: "freetype-sys".to_string(),
                version: "0.20.1".to_string(),
            },
            Dependency {
                name: "getrandom".to_string(),
                version: "0.2.12".to_string(),
            },
            Dependency {
                name: "gif".to_string(),
                version: "0.12.0".to_string(),
            },
            Dependency {
                name: "hashbrown".to_string(),
                version: "0.12.3".to_string(),
            },
            Dependency {
                name: "hashbrown".to_string(),
                version: "0.14.3".to_string(),
            },
            Dependency {
                name: "heck".to_string(),
                version: "0.4.1".to_string(),
            },
            Dependency {
                name: "hermit-abi".to_string(),
                version: "0.1.19".to_string(),
            },
            Dependency {
                name: "hermit-abi".to_string(),
                version: "0.3.3".to_string(),
            },
            Dependency {
                name: "humantime".to_string(),
                version: "2.1.0".to_string(),
            },
            Dependency {
                name: "iana-time-zone".to_string(),
                version: "0.1.60".to_string(),
            },
            Dependency {
                name: "iana-time-zone-haiku".to_string(),
                version: "0.1.2".to_string(),
            },
            Dependency {
                name: "image".to_string(),
                version: "0.24.9".to_string(),
            },
            Dependency {
                name: "indexmap".to_string(),
                version: "1.9.3".to_string(),
            },
            Dependency {
                name: "indexmap".to_string(),
                version: "2.2.6".to_string(),
            },
            Dependency {
                name: "is-terminal".to_string(),
                version: "0.4.9".to_string(),
            },
            Dependency {
                name: "itertools".to_string(),
                version: "0.11.0".to_string(),
            },
            Dependency {
                name: "itoa".to_string(),
                version: "1.0.9".to_string(),
            },
            Dependency {
                name: "jobserver".to_string(),
                version: "0.1.27".to_string(),
            },
            Dependency {
                name: "jpeg-decoder".to_string(),
                version: "0.3.1".to_string(),
            },
            Dependency {
                name: "js-sys".to_string(),
                version: "0.3.69".to_string(),
            },
            Dependency {
                name: "lalrpop".to_string(),
                version: "0.20.2".to_string(),
            },
            Dependency {
                name: "lalrpop-util".to_string(),
                version: "0.20.2".to_string(),
            },
            Dependency {
                name: "lazy_static".to_string(),
                version: "1.4.0".to_string(),
            },
            Dependency {
                name: "libc".to_string(),
                version: "0.2.153".to_string(),
            },
            Dependency {
                name: "libloading".to_string(),
                version: "0.8.3".to_string(),
            },
            Dependency {
                name: "libredox".to_string(),
                version: "0.0.1".to_string(),
            },
            Dependency {
                name: "linux-raw-sys".to_string(),
                version: "0.4.13".to_string(),
            },
            Dependency {
                name: "lock_api".to_string(),
                version: "0.4.12".to_string(),
            },
            Dependency {
                name: "log".to_string(),
                version: "0.4.20".to_string(),
            },
            Dependency {
                name: "matrixmultiply".to_string(),
                version: "0.3.8".to_string(),
            },
            Dependency {
                name: "memchr".to_string(),
                version: "2.6.4".to_string(),
            },
            Dependency {
                name: "memoffset".to_string(),
                version: "0.9.0".to_string(),
            },
            Dependency {
                name: "miniz_oxide".to_string(),
                version: "0.7.2".to_string(),
            },
            Dependency {
                name: "miow".to_string(),
                version: "0.6.0".to_string(),
            },
            Dependency {
                name: "nalgebra".to_string(),
                version: "0.32.5".to_string(),
            },
            Dependency {
                name: "nalgebra-macros".to_string(),
                version: "0.2.1".to_string(),
            },
            Dependency {
                name: "new_debug_unreachable".to_string(),
                version: "1.0.6".to_string(),
            },
            Dependency {
                name: "num-complex".to_string(),
                version: "0.4.5".to_string(),
            },
            Dependency {
                name: "num-integer".to_string(),
                version: "0.1.46".to_string(),
            },
            Dependency {
                name: "num-rational".to_string(),
                version: "0.4.1".to_string(),
            },
            Dependency {
                name: "num-traits".to_string(),
                version: "0.2.18".to_string(),
            },
            Dependency {
                name: "once_cell".to_string(),
                version: "1.18.0".to_string(),
            },
            Dependency {
                name: "os_str_bytes".to_string(),
                version: "6.6.1".to_string(),
            },
            Dependency {
                name: "parking_lot".to_string(),
                version: "0.12.2".to_string(),
            },
            Dependency {
                name: "parking_lot_core".to_string(),
                version: "0.9.10".to_string(),
            },
            Dependency {
                name: "paste".to_string(),
                version: "1.0.14".to_string(),
            },
            Dependency {
                name: "pathfinder_geometry".to_string(),
                version: "0.5.1".to_string(),
            },
            Dependency {
                name: "pathfinder_simd".to_string(),
                version: "0.5.3".to_string(),
            },
            Dependency {
                name: "petgraph".to_string(),
                version: "0.6.4".to_string(),
            },
            Dependency {
                name: "phf_shared".to_string(),
                version: "0.10.0".to_string(),
            },
            Dependency {
                name: "pico-args".to_string(),
                version: "0.5.0".to_string(),
            },
            Dependency {
                name: "pkg-config".to_string(),
                version: "0.3.30".to_string(),
            },
            Dependency {
                name: "plotters".to_string(),
                version: "0.3.5".to_string(),
            },
            Dependency {
                name: "plotters-backend".to_string(),
                version: "0.3.5".to_string(),
            },
            Dependency {
                name: "plotters-bitmap".to_string(),
                version: "0.3.3".to_string(),
            },
            Dependency {
                name: "plotters-svg".to_string(),
                version: "0.3.5".to_string(),
            },
            Dependency {
                name: "png".to_string(),
                version: "0.17.13".to_string(),
            },
            Dependency {
                name: "precomputed-hash".to_string(),
                version: "0.1.1".to_string(),
            },
            Dependency {
                name: "proc-macro-error".to_string(),
                version: "1.0.4".to_string(),
            },
            Dependency {
                name: "proc-macro-error-attr".to_string(),
                version: "1.0.4".to_string(),
            },
            Dependency {
                name: "proc-macro2".to_string(),
                version: "1.0.70".to_string(),
            },
            Dependency {
                name: "quote".to_string(),
                version: "1.0.33".to_string(),
            },
            Dependency {
                name: "rawpointer".to_string(),
                version: "0.2.1".to_string(),
            },
            Dependency {
                name: "rayon".to_string(),
                version: "1.8.0".to_string(),
            },
            Dependency {
                name: "rayon-core".to_string(),
                version: "1.12.0".to_string(),
            },
            Dependency {
                name: "redox_syscall".to_string(),
                version: "0.3.5".to_string(),
            },
            Dependency {
                name: "redox_syscall".to_string(),
                version: "0.4.1".to_string(),
            },
            Dependency {
                name: "redox_syscall".to_string(),
                version: "0.5.1".to_string(),
            },
            Dependency {
                name: "redox_users".to_string(),
                version: "0.4.4".to_string(),
            },
            Dependency {
                name: "regex".to_string(),
                version: "1.10.2".to_string(),
            },
            Dependency {
                name: "regex-automata".to_string(),
                version: "0.4.3".to_string(),
            },
            Dependency {
                name: "regex-syntax".to_string(),
                version: "0.8.2".to_string(),
            },
            Dependency {
                name: "rustc_version".to_string(),
                version: "0.4.0".to_string(),
            },
            Dependency {
                name: "rustix".to_string(),
                version: "0.38.31".to_string(),
            },
            Dependency {
                name: "rustversion".to_string(),
                version: "1.0.14".to_string(),
            },
            Dependency {
                name: "ryu".to_string(),
                version: "1.0.15".to_string(),
            },
            Dependency {
                name: "safe_arch".to_string(),
                version: "0.7.1".to_string(),
            },
            Dependency {
                name: "same-file".to_string(),
                version: "1.0.6".to_string(),
            },
            Dependency {
                name: "scopeguard".to_string(),
                version: "1.2.0".to_string(),
            },
            Dependency {
                name: "semver".to_string(),
                version: "1.0.20".to_string(),
            },
            Dependency {
                name: "serde".to_string(),
                version: "1.0.193".to_string(),
            },
            Dependency {
                name: "serde_derive".to_string(),
                version: "1.0.193".to_string(),
            },
            Dependency {
                name: "serde_json".to_string(),
                version: "1.0.108".to_string(),
            },
            Dependency {
                name: "simba".to_string(),
                version: "0.8.1".to_string(),
            },
            Dependency {
                name: "simd-adler32".to_string(),
                version: "0.3.7".to_string(),
            },
            Dependency {
                name: "siphasher".to_string(),
                version: "0.3.11".to_string(),
            },
            Dependency {
                name: "smallvec".to_string(),
                version: "1.13.2".to_string(),
            },
            Dependency {
                name: "string_cache".to_string(),
                version: "0.8.7".to_string(),
            },
            Dependency {
                name: "strsim".to_string(),
                version: "0.10.0".to_string(),
            },
            Dependency {
                name: "syn".to_string(),
                version: "1.0.109".to_string(),
            },
            Dependency {
                name: "syn".to_string(),
                version: "2.0.39".to_string(),
            },
            Dependency {
                name: "tempfile".to_string(),
                version: "3.8.1".to_string(),
            },
            Dependency {
                name: "term".to_string(),
                version: "0.7.0".to_string(),
            },
            Dependency {
                name: "termcolor".to_string(),
                version: "1.4.0".to_string(),
            },
            Dependency {
                name: "textwrap".to_string(),
                version: "0.16.0".to_string(),
            },
            Dependency {
                name: "thiserror".to_string(),
                version: "1.0.50".to_string(),
            },
            Dependency {
                name: "thiserror-impl".to_string(),
                version: "1.0.50".to_string(),
            },
            Dependency {
                name: "tiny-keccak".to_string(),
                version: "2.0.2".to_string(),
            },
            Dependency {
                name: "ttf-parser".to_string(),
                version: "0.17.1".to_string(),
            },
            Dependency {
                name: "typenum".to_string(),
                version: "1.17.0".to_string(),
            },
            Dependency {
                name: "unicode-ident".to_string(),
                version: "1.0.12".to_string(),
            },
            Dependency {
                name: "unicode-xid".to_string(),
                version: "0.2.4".to_string(),
            },
            Dependency {
                name: "version_check".to_string(),
                version: "0.9.4".to_string(),
            },
            Dependency {
                name: "walkdir".to_string(),
                version: "2.4.0".to_string(),
            },
            Dependency {
                name: "wasi".to_string(),
                version: "0.11.0+wasi-snapshot-preview1".to_string(),
            },
            Dependency {
                name: "wasm-bindgen".to_string(),
                version: "0.2.92".to_string(),
            },
            Dependency {
                name: "wasm-bindgen-backend".to_string(),
                version: "0.2.92".to_string(),
            },
            Dependency {
                name: "wasm-bindgen-macro".to_string(),
                version: "0.2.92".to_string(),
            },
            Dependency {
                name: "wasm-bindgen-macro-support".to_string(),
                version: "0.2.92".to_string(),
            },
            Dependency {
                name: "wasm-bindgen-shared".to_string(),
                version: "0.2.92".to_string(),
            },
            Dependency {
                name: "web-sys".to_string(),
                version: "0.3.69".to_string(),
            },
            Dependency {
                name: "weezl".to_string(),
                version: "0.1.8".to_string(),
            },
            Dependency {
                name: "wide".to_string(),
                version: "0.7.16".to_string(),
            },
            Dependency {
                name: "winapi".to_string(),
                version: "0.3.9".to_string(),
            },
            Dependency {
                name: "winapi-i686-pc-windows-gnu".to_string(),
                version: "0.4.0".to_string(),
            },
            Dependency {
                name: "winapi-util".to_string(),
                version: "0.1.6".to_string(),
            },
            Dependency {
                name: "winapi-x86_64-pc-windows-gnu".to_string(),
                version: "0.4.0".to_string(),
            },
            Dependency {
                name: "windows-core".to_string(),
                version: "0.52.0".to_string(),
            },
            Dependency {
                name: "windows-sys".to_string(),
                version: "0.48.0".to_string(),
            },
            Dependency {
                name: "windows-sys".to_string(),
                version: "0.52.0".to_string(),
            },
            Dependency {
                name: "windows-targets".to_string(),
                version: "0.48.5".to_string(),
            },
            Dependency {
                name: "windows-targets".to_string(),
                version: "0.52.0".to_string(),
            },
            Dependency {
                name: "windows_aarch64_gnullvm".to_string(),
                version: "0.48.5".to_string(),
            },
            Dependency {
                name: "windows_aarch64_gnullvm".to_string(),
                version: "0.52.0".to_string(),
            },
            Dependency {
                name: "windows_aarch64_msvc".to_string(),
                version: "0.48.5".to_string(),
            },
            Dependency {
                name: "windows_aarch64_msvc".to_string(),
                version: "0.52.0".to_string(),
            },
            Dependency {
                name: "windows_i686_gnu".to_string(),
                version: "0.48.5".to_string(),
            },
            Dependency {
                name: "windows_i686_gnu".to_string(),
                version: "0.52.0".to_string(),
            },
            Dependency {
                name: "windows_i686_msvc".to_string(),
                version: "0.48.5".to_string(),
            },
            Dependency {
                name: "windows_i686_msvc".to_string(),
                version: "0.52.0".to_string(),
            },
            Dependency {
                name: "windows_x86_64_gnu".to_string(),
                version: "0.48.5".to_string(),
            },
            Dependency {
                name: "windows_x86_64_gnu".to_string(),
                version: "0.52.0".to_string(),
            },
            Dependency {
                name: "windows_x86_64_gnullvm".to_string(),
                version: "0.48.5".to_string(),
            },
            Dependency {
                name: "windows_x86_64_gnullvm".to_string(),
                version: "0.52.0".to_string(),
            },
            Dependency {
                name: "windows_x86_64_msvc".to_string(),
                version: "0.48.5".to_string(),
            },
            Dependency {
                name: "windows_x86_64_msvc".to_string(),
                version: "0.52.0".to_string(),
            },
            Dependency {
                name: "wio".to_string(),
                version: "0.2.2".to_string(),
            },
            Dependency {
                name: "yeslogic-fontconfig-sys".to_string(),
                version: "3.2.0".to_string(),
            },
        ]
        .into_iter();

        while let Some(d) = Dependency::from_reader(&mut r) {
            assert_eq!(dependencies.next().unwrap(), d,)
        }
        assert_eq!(dependencies.next(), None);
    }
}
