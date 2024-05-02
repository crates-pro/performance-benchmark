use super::{mir::MIRs, terminator::Terminator};

fn contains_io_keywords(input: &str) -> bool {
    // 定义与 I/O 操作相关的关键字数组
    let io_keywords = [
        // 网络 I/O 操作
        "UdpSocket",
        "TcpStream",
        "TcpListener",
        // 标准输入/输出操作
        "stdin",
        "stdout",
        "stderr",
        // 读写操作
        "read",
        "write",
        "BufRead",
        "seek",
        "copy",
        "flush",
        "read_to_string",
        "read_to_end",
        // 文件操作
        "open",
        "create",
        //"std::fs::read",
        //"std::fs::write",
    ];
    for keyword in &io_keywords {
        if input.contains(keyword) {
            return true;
        }
    }
    false
}

pub fn count_io_metrics(mir_file: &MIRs) -> i32 {
    let mut io_count = 0;
    let functions = &mir_file.functions;
    for function in functions {
        let bbs = &function.bbs;
        for basic_block in bbs {
            let terminator = &basic_block.terminator;
            match terminator {
                Some(terminator) => match terminator {
                    Terminator::Call(call_data) => {
                        let callee = &call_data.callee;
                        for moduled_name in callee {
                            if contains_io_keywords(&moduled_name) {
                                io_count += 1;
                                break;
                            }
                        }
                    }
                    _ => {}
                },
                None => {}
            }
        }
    }
    // println!("{:?}", io_count);
    io_count
}
