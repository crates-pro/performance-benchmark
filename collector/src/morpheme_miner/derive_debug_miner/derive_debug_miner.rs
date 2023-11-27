use crate::morpheme_miner::miner::{InfoFile, MorphemeMiner};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DERIVE_DEBUG: &str = "Derive Debug";

/// DeriveDebugMiner will count `#derive[Debug]` usages in a rust project.
pub(crate) struct DeriveDebugMiner {}

impl MorphemeMiner for DeriveDebugMiner {
    /// mine_from_file will iterate each line of a file and count
    /// the usage of `#derive[Debug]`.
    fn mine_from_file(&self, mut reader: BufReader<File>) -> InfoFile {
        let mut count = 0;

        let mut buf = String::new();
        while let Ok(size) = reader.read_line(&mut buf) {
            if size == 0 {
                return (DERIVE_DEBUG.to_string(), count);
            }

            // remove the leading whitespace in buf
            let s = buf.trim_start();

            // skip the line if it is a comment
            if s.starts_with("//") {
                continue;
            }

            // if the line starts with `derive[`, then check whether `Debug` is in this line.
            if s.starts_with("#[derive(") {
                if s.contains("Debug") {
                    count += 1;
                }
            }

            buf.clear();
        }

        (DERIVE_DEBUG.to_string(), count)
    }

    fn morpheme_name(&self) -> String {
        DERIVE_DEBUG.to_string()
    }
}

#[test]
fn test_derive_debug_miner() {
    use std::path::PathBuf;

    let miner = DeriveDebugMiner {};
    if let Ok(result) =
        miner.mine_from_project(&PathBuf::from("test/MorphemeMiner/DeriveDebugMiner/proj_1"))
    {
        assert_eq!(result.1, 1);
    }
    if let Ok(result) =
        miner.mine_from_project(&PathBuf::from("test/MorphemeMiner/DeriveDebugMiner/proj_2"))
    {
        assert_eq!(result.1, 4);
    }
}
