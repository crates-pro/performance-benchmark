use std::{
    fs::File,
    io::{BufReader, Read},
};

use lalrpop_util::lalrpop_mod;

use super::mir::MIRs;

lalrpop_mod!(pub mir_parser, "/mir_analyze/mir/mir.rs");

pub fn parse_mir(mir_file: File) -> anyhow::Result<MIRs> {
    let mut reader = BufReader::new(mir_file);
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    match mir_parser::mirsParser::new().parse(buf.as_str()) {
        Ok(mirs) => Ok(mirs),
        Err(e) => panic!("{}", e),
    }
}
pub struct TestFile {
    pub name: &'static str,
    pub path: &'static str,
}

pub const TEST_FILES: [TestFile; 46] = [
    /*TestFile { name: "dev", path: "test/mir_analyze/mir/dev.mir" },
    TestFile { name: "binary_op", path: "test/mir_analyze/mir/binary_unary_op.mir" },
    TestFile { name: "aggregate", path: "test/mir_analyze/mir/aggregate.mir" },
    TestFile { name: "closure", path: "test/mir_analyze/mir/closure.mir" },*/
    TestFile {
        name: "runiq",
        path: "test/mir_analyze/mir/texteditor/runiq.mir",
    },
    TestFile {
        name: "ruplacer",
        path: "test/mir_analyze/mir/texteditor/ruplacer.mir",
    },
    TestFile {
        name: "xsv",
        path: "test/mir_analyze/mir/texteditor/xsv.mir",
    },
    TestFile {
        name: "amp",
        path: "test/mir_analyze/mir/texteditor/amp.mir",
    },
    TestFile {
        name: "grex",
        path: "test/mir_analyze/mir/texteditor/grex.mir",
    },
    TestFile {
        name: "kibi",
        path: "test/mir_analyze/mir/texteditor/kibi.mir",
    },
    TestFile {
        name: "kiro",
        path: "test/mir_analyze/mir/texteditor/kiro.mir",
    },
    TestFile {
        name: "pepper",
        path: "test/mir_analyze/mir/texteditor/pepper.mir",
    },
    TestFile {
        name: "systemstat",
        path: "test/mir_analyze/mir/system_programing/systemstat.mir",
    },
    TestFile {
        name: "system76",
        path: "test/mir_analyze/mir/system_programing/system76-power.mir",
    },
    TestFile {
        name: "systemd",
        path: "test/mir_analyze/mir/system_programing/systemd.mir",
    },
    TestFile {
        name: "coreutils",
        path: "test/mir_analyze/mir/system_programing/coreutils.mir",
    },
    TestFile {
        name: "bat",
        path: "test/mir_analyze/mir/fs/bat.mir",
    },
    TestFile {
        name: "broot",
        path: "test/mir_analyze/mir/fs/broot.mir",
    },
    TestFile {
        name: "exa",
        path: "test/mir_analyze/mir/fs/exa.mir",
    },
    TestFile {
        name: "fd",
        path: "test/mir_analyze/mir/fs/fd.mir",
    },
    TestFile {
        name: "lsd",
        path: "test/mir_analyze/mir/fs/lsd.mir",
    },
    TestFile {
        name: "rg",
        path: "test/mir_analyze/mir/fs/rg.mir",
    },
    TestFile {
        name: "zoxide",
        path: "test/mir_analyze/mir/fs/zoxide.mir",
    },
    TestFile {
        name: "lucid",
        path: "test/mir_analyze/mir/db/lucid.mir",
    },
    TestFile {
        name: "rocksdb",
        path: "test/mir_analyze/mir/db/rocksdb.mir",
    },
    TestFile {
        name: "skysh",
        path: "test/mir_analyze/mir/db/skysh.mir",
    },
    TestFile {
        name: "sled",
        path: "test/mir_analyze/mir/db/sled.mir",
    },
    TestFile {
        name: "toydb",
        path: "test/mir_analyze/mir/db/toydb.mir",
    },
    TestFile {
        name: "http",
        path: "test/mir_analyze/mir/web/http.mir",
    },
    TestFile {
        name: "relay",
        path: "test/mir_analyze/mir/web/relay.mir",
    },
    TestFile {
        name: "Rocket",
        path: "test/mir_analyze/mir/web/Rocket.mir",
    },
    TestFile {
        name: "web",
        path: "test/mir_analyze/mir/web/web.mir",
    },
    TestFile {
        name: "rustlings",
        path: "test/mir_analyze/mir/compiler/rustlings.mir",
    },
    TestFile {
        name: "wasmer",
        path: "test/mir_analyze/mir/compiler/wasmer.mir",
    },
    TestFile {
        name: "blockchain_core",
        path: "test/mir_analyze/mir/blockchain/blockchain_core.mir",
    },
    TestFile {
        name: "starcoin",
        path: "test/mir_analyze/mir/blockchain/starcoin.mir",
    },
    TestFile {
        name: "graph",
        path: "test/mir_analyze/mir/blockchain/graph.mir",
    },
    TestFile {
        name: "diem_client",
        path: "test/mir_analyze/mir/blockchain/diem_core/diem_client.mir",
    },
    TestFile {
        name: "diem_wallet",
        path: "test/mir_analyze/mir/blockchain/diem_core/diem_wallet.mir",
    },
    TestFile {
        name: "conflux",
        path: "test/mir_analyze/mir/blockchain/conflux.mir",
    },
    TestFile {
        name: "ckb",
        path: "test/mir_analyze/mir/blockchain/ckb.mir",
    },
    TestFile {
        name: "cita_auth",
        path: "test/mir_analyze/mir/blockchain/cita/cita_auth.mir",
    },
    TestFile {
        name: "cita_jsonrpc",
        path: "test/mir_analyze/mir/blockchain/cita/cita_jsonrpc.mir",
    },
    TestFile {
        name: "cita_chain",
        path: "test/mir_analyze/mir/blockchain/cita/cita_chain.mir",
    },
    TestFile {
        name: "common_types",
        path: "test/mir_analyze/mir/blockchain/cita/common-types.mir",
    },
    TestFile {
        name: "core",
        path: "test/mir_analyze/mir/blockchain/cita/core.mir",
    },
    TestFile {
        name: "cita_network",
        path: "test/mir_analyze/mir/blockchain/cita/cita_network.mir",
    },
    TestFile {
        name: "cita_executor",
        path: "test/mir_analyze/mir/blockchain/cita/cita_executor.mir",
    },
    TestFile {
        name: "create_key_addr",
        path: "test/mir_analyze/mir/blockchain/cita/create_key_addr.mir",
    },
    TestFile {
        name: "chain_executor_mock",
        path: "test/mir_analyze/mir/blockchain/cita/chain_executor_mock.mir",
    },
];

pub const TEST_RUSTC: [TestFile; 28] = [
    TestFile {
        name: "encoding",
        path: "test/rustperf/encoding.mir",
    },
    TestFile {
        name: "rayon",
        path: "test/rustperf/rayon.mir",
    },
    TestFile {
        name: "unicode",
        path: "test/rustperf/unicode.mir",
    },
    /*TestFile {
        name: "tuple_stress",
        path: "test/rustperf/tuple_stress.mir",
    },*/
    TestFile {
        name: "piston_image",
        path: "test/rustperf/piston_image.mir",
    },
    /*TestFile {
        name: "cranelift",
        path: "test/rustperf/cranelift.mir",
    },

    TestFile {
        name: "diesel",
        path: "test/rustperf/diesel.mir",
    },*/
    TestFile {
        name: "wg_grammer",
        path: "test/rustperf/wg_grammer.mir",
    },
    TestFile {
        name: "many_assoc",
        path: "test/rustperf/many_assoc.mir",
    },
    TestFile {
        name: "serde",
        path: "test/rustperf/serde.mir",
    },
    TestFile {
        name: "serde_drive",
        path: "test/rustperf/serde_drive.mir",
    },
    /*TestFile {
        name: "coercions",
        path: "test/rustperf/coercions.mir",
    },*/
    TestFile {
        name: "regress",
        path: "test/rustperf/regress.mir",
    },
    TestFile {
        name: "regex",
        path: "test/rustperf/regex.mir",
    },
    TestFile {
        name: "match_stress",
        path: "test/rustperf/match_stress.mir",
    },
    TestFile {
        name: "exa",
        path: "test/rustperf/exa.mir",
    },
    TestFile {
        name: "libc",
        path: "test/rustperf/libc.mir",
    },
    TestFile {
        name: "html5ever2",
        path: "test/rustperf/html5ever2.mir",
    },
    TestFile {
        name: "html5ever",
        path: "test/rustperf/html5ever.mir",
    },
    TestFile {
        name: "clap",
        path: "test/rustperf/clap.mir",
    },
    TestFile {
        name: "await-call-tree",
        path: "test/rustperf/await_call_tree.mir",
    },
    TestFile {
        name: "deeply_nested_multi",
        path: "test/rustperf/deeply_nested_multi.mir",
    },
    TestFile {
        name: "issue_1",
        path: "test/rustperf/issue_1.mir",
    },
    TestFile {
        name: "projection_caching",
        path: "test/rustperf/projection_caching.mir",
    },
    TestFile {
        name: "wf_projection",
        path: "test/rustperf/wf_projection.mir",
    },
    TestFile {
        name: "tt_munther",
        path: "test/rustperf/tt_munther.mir",
    },
    TestFile {
        name: "webpush",
        path: "test/rustperf/webpush.mir",
    },
    TestFile {
        name: "token_stream",
        path: "test/rustperf/token_stream.mir",
    },
    TestFile {
        name: "issue_2",
        path: "test/rustperf/issue_2.mir",
    },
    TestFile {
        name: "inflate",
        path: "test/rustperf/inflate.mir",
    },
    TestFile {
        name: "syn_1",
        path: "test/rustperf/syn_1.mir",
    },
    TestFile {
        name: "syn_2",
        path: "test/rustperf/syn_2.mir",
    },
];

fn run_test(file_path: &str) {
    let test_file = File::open(file_path).unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}

#[test]
fn test_files() {
    for test_file in TEST_FILES.iter() {
        println!("Running test for {}", test_file.name);
        run_test(test_file.path);
        println!();
    }
}

#[test]
fn test_rustc() {
    for test_file in TEST_RUSTC.iter() {
        println!("Running test for {}", test_file.name);
        run_test(test_file.path);
        println!();
    }
}

#[test]
fn test_dev() {
    run_test("test/mir_analyze/mir/dev.mir");
    println!();
}
