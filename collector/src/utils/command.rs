use std::process::{self, Command, Stdio};

pub fn run_command_with_output(cmd: &mut Command) -> anyhow::Result<process::Output> {
    use crate::utils::read2;
    use anyhow::Context;
    let mut child = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("failed to spawn process for cmd: {:?}", cmd))?;

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut stdout_writer = std::io::LineWriter::new(std::io::stdout());
    let mut stderr_writer = std::io::LineWriter::new(std::io::stderr());
    read2::read2(
        child.stdout.take().unwrap(),
        child.stderr.take().unwrap(),
        &mut |is_stdout, buffer, _is_done| {
            // Send output if trace logging is enabled
            if log::log_enabled!(target: "raw_cargo_messages", log::Level::Trace) {
                use std::io::Write;
                if is_stdout {
                    stdout_writer.write_all(&buffer[stdout.len()..]).unwrap();
                } else {
                    stderr_writer.write_all(&buffer[stderr.len()..]).unwrap();
                }
            }
            if is_stdout {
                stdout = buffer.clone();
            } else {
                stderr = buffer.clone();
            }
        },
    )?;

    let status = child
        .wait()
        .with_context(|| "failed to wait on child process")?;

    Ok(process::Output {
        status,
        stdout,
        stderr,
    })
}

pub fn command_output(cmd: &mut Command) -> anyhow::Result<process::Output> {
    let output = run_command_with_output(cmd)?;

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "expected success, got {}\n\nstderr={}\n\n stdout={}\n",
            output.status,
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout)
        ));
    }

    Ok(output)
}

pub fn command_discard_output(cmd: &mut Command) -> anyhow::Result<()> {
    cmd.stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?
        .wait()?;

    Ok(())
}
