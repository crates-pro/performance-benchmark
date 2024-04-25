use std::{
    fs::File,
    io::{BufReader, Read},
};

use lalrpop_util::lalrpop_mod;

use super::{basic_block, mir::{MIRs, ModuledIdentifier}, terminator::Terminator,scope::Scope, function_pattern::*, io_function::*, parallelism::*, oop_pattern::*};

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



#[test]
fn test_dev() {
    let test_file = File::open("test/mir_analyze/mir/dev.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}

#[test]
fn test_binary_op() {
    let test_file = File::open("test/mir_analyze/mir/binary_unary_op.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}

#[test]
fn test_aggregate() {
    let test_file = File::open("test/mir_analyze/mir/aggregate.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_closure() {
    let test_file = File::open("test/mir_analyze/mir/closure.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);


}
#[test]
fn test_runiq() {
    let test_file = File::open("test/mir_analyze/mir/texteditor/runiq.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_ruplacer() {
    let test_file = File::open("test/mir_analyze/mir/texteditor/ruplacer.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_xsv() {
    let test_file = File::open("test/mir_analyze/mir/texteditor/xsv.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_amp() {
    let test_file = File::open("test/mir_analyze/mir/texteditor/amp.mir").unwrap();

    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_grex() {
    let test_file = File::open("test/mir_analyze/mir/texteditor/grex.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_kibi() {
    let test_file = File::open("test/mir_analyze/mir/texteditor/kibi.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_kiro() {
    let test_file = File::open("test/mir_analyze/mir/texteditor/kiro.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_pepper() {
    let test_file = File::open("test/mir_analyze/mir/texteditor/pepper.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_systemstat() {
    let test_file = File::open("test/mir_analyze/mir/system_programing/systemstat.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_system76() {
    let test_file = File::open("test/mir_analyze/mir/system_programing/system76-power.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_systemd() {
    let test_file = File::open("test/mir_analyze/mir/system_programing/systemd.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_coreutils() {
    let test_file = File::open("test/mir_analyze/mir/system_programing/coreutils.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_bat() {
    let test_file = File::open("test/mir_analyze/mir/fs/bat.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_grex() {
    let test_file = File::open("test/mir_analyze/mir/texteditor/grex.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_kibi() {
    let test_file = File::open("test/mir_analyze/mir/texteditor/kibi.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_kiro() {
    let test_file = File::open("test/mir_analyze/mir/texteditor/kiro.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_pepper() {
    let test_file = File::open("test/mir_analyze/mir/texteditor/pepper.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_systemstat() {
    let test_file = File::open("test/mir_analyze/mir/system_programing/systemstat.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_system76() {
    let test_file = File::open("test/mir_analyze/mir/system_programing/system76-power.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_systemd() {
    let test_file = File::open("test/mir_analyze/mir/system_programing/systemd.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_coreutils() {
    let test_file = File::open("test/mir_analyze/mir/system_programing/coreutils.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_bat() {
    let test_file = File::open("test/mir_analyze/mir/fs/bat.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_broot() {
    let test_file = File::open("test/mir_analyze/mir/fs/broot.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_exa() {
    let test_file = File::open("test/mir_analyze/mir/fs/exa.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_fd() {
    let test_file = File::open("test/mir_analyze/mir/fs/fd.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_lsd() {
    let test_file = File::open("test/mir_analyze/mir/fs/lsd.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_rg() {
    let test_file = File::open("test/mir_analyze/mir/fs/rg.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_zoxide() {
    let test_file = File::open("test/mir_analyze/mir/fs/zoxide.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_lucid() {
    let test_file = File::open("test/mir_analyze/mir/db/lucid.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_rocksdb() {
    let test_file = File::open("test/mir_analyze/mir/db/rocksdb.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_skysh() {
    let test_file = File::open("test/mir_analyze/mir/db/skysh.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_sled() {
    let test_file = File::open("test/mir_analyze/mir/db/sled.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_toydb() {
    let test_file = File::open("test/mir_analyze/mir/db/toydb.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_http() {
    let test_file = File::open("test/mir_analyze/mir/web/http.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_relay() {
    let test_file = File::open("test/mir_analyze/mir/web/relay.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_rocket() {
    let test_file = File::open("test/mir_analyze/mir/web/Rocket.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_web() {
    let test_file = File::open("test/mir_analyze/mir/web/web.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_deno() {
    let test_file = File::open("test/mir_analyze/mir/compiler/deno.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_rustlings() {
    let test_file = File::open("test/mir_analyze/mir/compiler/rustlings.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_wasmer() {
    let test_file = File::open("test/mir_analyze/mir/compiler/wasmer.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_blockchain_core() {
    let test_file = File::open("test/mir_analyze/mir/blockchain/blockchain_core.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_starcoin() {
    let test_file = File::open("test/mir_analyze/mir/blockchain/starcoin.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_graph() {
    let test_file = File::open("test/mir_analyze/mir/blockchain/graph.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_diem_client() {
    let test_file = File::open("test/mir_analyze/mir/blockchain/diem_core/diem_client.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_diem_wallet() {
    let test_file = File::open("test/mir_analyze/mir/blockchain/diem_core/diem_wallet.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_conflux() {
    let test_file = File::open("test/mir_analyze/mir/blockchain/conflux.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_ckb() {
    let test_file = File::open("test/mir_analyze/mir/blockchain/ckb.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_cita_auth() {
    let test_file = File::open("test/mir_analyze/mir/blockchain/cita/cita_auth.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_cita_jsonrpc() {
    let test_file = File::open("test/mir_analyze/mir/blockchain/cita/cita_jsonrpc.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_cita_chain() {
    let test_file = File::open("test/mir_analyze/mir/blockchain/cita/cita_chain.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_common_types() {
    let test_file = File::open("test/mir_analyze/mir/blockchain/cita/common-types.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_core() {
    let test_file = File::open("test/mir_analyze/mir/blockchain/cita/core.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_cita_network() {
    let test_file = File::open("test/mir_analyze/mir/blockchain/cita/cita_network.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_cita_executor() {
    let test_file = File::open("test/mir_analyze/mir/blockchain/cita/cita_executor.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_core_executor() {
    let test_file = File::open("test/mir_analyze/mir/blockchain/cita/core_executor.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_create_key_addr() {
    let test_file = File::open("test/mir_analyze/mir/blockchain/cita/create_key_addr.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_chain_executor_mock() {
    let test_file = File::open("test/mir_analyze/mir/blockchain/cita/chain_executor_mock.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_runiq() {
    let test_file = File::open("test/mir_analyze/mir/texteditor/runiq.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_ruplacer() {
    let test_file = File::open("test/mir_analyze/mir/texteditor/ruplacer.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_xsv() {
    let test_file = File::open("test/mir_analyze/mir/texteditor/xsv.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_amp() {
    let test_file = File::open("test/mir_analyze/mir/texteditor/amp.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_grex() {
    let test_file = File::open("test/mir_analyze/mir/texteditor/grex.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_kibi() {
    let test_file = File::open("test/mir_analyze/mir/texteditor/kibi.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_kiro() {
    let test_file = File::open("test/mir_analyze/mir/texteditor/kiro.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_pepper() {
    let test_file = File::open("test/mir_analyze/mir/texteditor/pepper.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_systemstat() {
    let test_file = File::open("test/mir_analyze/mir/system_programing/systemstat.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_system76() {
    let test_file = File::open("test/mir_analyze/mir/system_programing/system76-power.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_systemd() {
    let test_file = File::open("test/mir_analyze/mir/system_programing/systemd.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_coreutils() {
    let test_file = File::open("test/mir_analyze/mir/system_programing/coreutils.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_bat() {
    let test_file = File::open("test/mir_analyze/mir/fs/bat.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_broot() {
    let test_file = File::open("test/mir_analyze/mir/fs/broot.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_exa() {
    let test_file = File::open("test/mir_analyze/mir/fs/exa.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_fd() {
    let test_file = File::open("test/mir_analyze/mir/fs/fd.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_lsd() {
    let test_file = File::open("test/mir_analyze/mir/fs/lsd.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_rg() {
    let test_file = File::open("test/mir_analyze/mir/fs/rg.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
#[test]
fn test_zoxide() {
    let test_file = File::open("test/mir_analyze/mir/fs/zoxide.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_lucid() {
    let test_file = File::open("test/mir_analyze/mir/db/lucid.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_rocksdb() {
    let test_file = File::open("test/mir_analyze/mir/db/rocksdb.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_skysh() {
    let test_file = File::open("test/mir_analyze/mir/db/skysh.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_sled() {
    let test_file = File::open("test/mir_analyze/mir/db/sled.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_toydb() {
    let test_file = File::open("test/mir_analyze/mir/db/toydb.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_http() {
    let test_file = File::open("test/mir_analyze/mir/web/http.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_relay() {
    let test_file = File::open("test/mir_analyze/mir/web/relay.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_rocket() {
    let test_file = File::open("test/mir_analyze/mir/web/Rocket.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_web() {
    let test_file = File::open("test/mir_analyze/mir/web/web.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_deno() {
    let test_file = File::open("test/mir_analyze/mir/compiler/deno.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_rustlings() {
    let test_file = File::open("test/mir_analyze/mir/compiler/rustlings.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_wasmer() {
    let test_file = File::open("test/mir_analyze/mir/compiler/wasmer.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_blockchain_core() {
    let test_file = File::open("test/mir_analyze/mir/blockchain/blockchain_core.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_starcoin() {
    let test_file = File::open("test/mir_analyze/mir/blockchain/starcoin.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_graph() {
    let test_file = File::open("test/mir_analyze/mir/blockchain/graph.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_diem_client() {
    let test_file = File::open("test/mir_analyze/mir/blockchain/diem_core/diem_client.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_diem_wallet() {
    let test_file = File::open("test/mir_analyze/mir/blockchain/diem_core/diem_wallet.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_conflux() {
    let test_file = File::open("test/mir_analyze/mir/blockchain/conflux.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_ckb() {
    let test_file = File::open("test/mir_analyze/mir/blockchain/ckb.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_cita_auth() {
    let test_file = File::open("test/mir_analyze/mir/blockchain/cita/cita_auth.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_cita_jsonrpc() {
    let test_file = File::open("test/mir_analyze/mir/blockchain/cita/cita_jsonrpc.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_cita_chain() {
    let test_file = File::open("test/mir_analyze/mir/blockchain/cita/cita_chain.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_common_types() {
    let test_file = File::open("test/mir_analyze/mir/blockchain/cita/common-types.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_core() {
    let test_file = File::open("test/mir_analyze/mir/blockchain/cita/core.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_cita_network() {
    let test_file = File::open("test/mir_analyze/mir/blockchain/cita/cita_network.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_cita_executor() {
    let test_file = File::open("test/mir_analyze/mir/blockchain/cita/cita_executor.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_core_executor() {
    let test_file = File::open("test/mir_analyze/mir/blockchain/cita/core_executor.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_create_key_addr() {
    let test_file = File::open("test/mir_analyze/mir/blockchain/cita/create_key_addr.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}
#[test]
fn test_chain_executor_mock() {
    let test_file = File::open("test/mir_analyze/mir/blockchain/cita/chain_executor_mock.mir").unwrap();
    let result = parse_mir(test_file).unwrap();
    println!("{:?}", result);
}


