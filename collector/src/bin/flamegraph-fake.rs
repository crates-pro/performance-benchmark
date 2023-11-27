use std::{env, process::Command};

fn main() {
    let mut arg_os = env::args_os();
    let name = arg_os.next().unwrap().into_string().unwrap();

    let args = arg_os.collect::<Vec<_>>();

    let mut cmd = Command::new("flamegraph");
    let has_flamegraph = cmd.output().is_ok();
    assert!(has_flamegraph);

    raise_process_priority();

    println!("name: {}", name);
    println!("args: {:?}", args);

    cmd.arg("--flamechart").args(args);

    let status = cmd.status().expect("failed to spawn");
    assert!(
        status.success(),
        "command did not complete successfully: {:?}",
        cmd
    );
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
