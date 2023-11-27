use std::{
    env, fs,
    process::{Command, Stdio},
    time::{Duration, Instant},
};

fn main() {
    let mut arg_os = env::args_os();
    let _name = arg_os.next().unwrap().into_string().unwrap();

    let mut args = arg_os.collect::<Vec<_>>();

    let elf = env::var_os("RUNTIME_ELF").unwrap();

    raise_process_priority();

    if let Some(pos) = args.iter().position(|arg| arg == "--wrap-rustc-with") {
        let mut cmd;
        // Strip out the flag and its argument, and run rustc under the wrapper
        // program named by the argument.
        args.remove(pos);
        let wrapper = args.remove(pos);
        let wrapper = wrapper.to_str().unwrap();

        raise_process_priority();

        // These strings come from `PerfTool::name()`.
        match wrapper {
            "PerfStat" | "PerfStatSelfProfile" => {
                cmd = Command::new("perf");
                let has_perf = cmd.output().is_ok();
                assert!(has_perf);
                cmd.arg("stat")
                    // perf respects this environment variable for e.g., percents in
                    // the output, but we want standard output on all systems.
                    .env("LC_NUMERIC", "C")
                    .arg("-x;")
                    .arg("-e")
                    .arg("instructions:u,cycles:u,task-clock,cpu-clock,faults,context-switches,branch-misses,cache-misses")
                    .arg("--log-fd")
                    .arg("1")
                    .arg("setarch")
                    .arg(std::env::consts::ARCH)
                    .arg("-R")
                    .arg(&elf)
                    .args(&args)
                    .stderr(Stdio::null());

                let prof_out_dir = std::env::current_dir().unwrap().join("self-profile-output");
                if wrapper == "PerfStatSelfProfile" {
                    cmd.arg(&format!(
                        "-Zself-profile={}",
                        prof_out_dir.to_str().unwrap()
                    ));
                    let _ = fs::remove_dir_all(&prof_out_dir);
                    let _ = fs::create_dir_all(&prof_out_dir);
                }
            }

            "PerfRecord" => {
                let out_file = if let Some(pos) = args.iter().position(|arg| arg == "--output") {
                    args.remove(pos);
                    let out_file = args.remove(pos);
                    let out_file = out_file.to_str().unwrap().to_string();
                    out_file
                } else {
                    "perf.data".to_string()
                };
                cmd = Command::new("perf");
                let has_perf = cmd.output().is_ok();
                assert!(has_perf);
                cmd.arg("record")
                    .arg("-a")
                    .arg("--output")
                    .arg(out_file)
                    .arg("--freq=3989")
                    .arg("--event=cycles:u,instructions:u")
                    .arg(&elf)
                    .args(&args);
            }
            _ => panic!(),
        }

        let start = Instant::now();

        let _status = cmd.status().expect("failed to spawn");
        let dur = start.elapsed();
        print_memory();
        print_time(dur);
    };
}

#[cfg(unix)]
fn raise_process_priority() {
    unsafe {
        // Try to reduce jitter in wall time by increasing our priority to the
        // maximum
        for i in (1..21).rev() {
            let r = libc::setpriority(libc::PRIO_PROCESS as _, libc::getpid() as libc::id_t, -i);
            if r == 0 {
                break;
            }
        }
    }
}

fn print_memory() {
    use std::mem;

    unsafe {
        let mut usage = mem::zeroed();
        let r = libc::getrusage(libc::RUSAGE_CHILDREN, &mut usage);
        if r == 0 {
            // for explanation of all the semicolons, see `print_time` below
            println!("{};;max-rss;3;100.00", usage.ru_maxrss);
        }
    }
}

fn print_time(dur: Duration) {
    // Format output the same as `perf stat` in CSV mode, explained at
    // http://man7.org/linux/man-pages/man1/perf-stat.1.html#CSV_FORMAT
    //
    // tl;dr; it's:
    //
    //      $value ; $unit ; $name ; $runtime ; $pct
    println!(
        "{}.{:09};;wall-time;4;100.00",
        dur.as_secs(),
        dur.subsec_nanos()
    );
}
