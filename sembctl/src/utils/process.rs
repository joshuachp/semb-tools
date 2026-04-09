fn spawn(mut command: std::process::Command) -> eyre::Result<ExitStatus> {
    let mut cmd = command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;

    if let Some(out) = cmd.stdout.take() {
        for line in BufReader::new(out).lines() {
            let line = line?;

            trace!(output = line.replace("\n", "\\n"));
        }
    }

    let status = cmd.wait()?;

    ensure!(status.success(), "command exited with error");

    Ok(status)
}
