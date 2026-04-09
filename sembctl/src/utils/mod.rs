use std::io::{BufRead, BufReader};
use std::process::{Command, ExitStatus, Stdio};

use eyre::ensure;
use tracing::{debug, instrument};

pub(crate) fn command_ok(command: Command) -> bool {
    spawn_cmd(command).is_ok()
}

#[instrument]
pub(crate) fn spawn_cmd(mut command: Command) -> eyre::Result<ExitStatus> {
    let mut cmd = command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;

    if let Some(out) = cmd.stdout.take() {
        for line in BufReader::new(out).lines() {
            let line = line?;

            debug!(output = line.replace("\n", "\\n"));
        }
    }

    let status = cmd.wait()?;

    ensure!(status.success(), "command exited with error");

    Ok(status)
}
