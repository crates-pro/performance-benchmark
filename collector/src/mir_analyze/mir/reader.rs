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

#[test]
fn test_dev() {
    let test_file = File::open("test/mir_analyze/mir/dev.mir").unwrap();
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
fn test_binary_op() {
    let test_file = File::open("test/mir_analyze/mir/binary_unary_op.mir").unwrap();
    let result = parse_mir(test_file).unwrap();

    println!("{:?}", result);
}
