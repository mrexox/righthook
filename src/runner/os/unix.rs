use super::output::Output;
use crate::Result;
use std::io::Write;
use std::process::{Command, Stdio};
use std::str::from_utf8;

pub fn execute(cmd: &str, stdin_data: &str) -> Result<(bool, Output)> {
    let stdin = if stdin_data.is_empty() {
        Stdio::null()
    } else {
        Stdio::piped()
    };

    let mut child = Command::new("sh")
        .stdin(stdin)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .arg("-c")
        .arg(cmd)
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        // Write the input string to the child's stdin
        stdin.write_all(stdin_data.as_bytes())?;
    }

    let output = child.wait_with_output()?;

    Ok((
        output.status.success(),
        Output {
            stdout: from_utf8(&output.stdout)?.to_string(),
            stderr: from_utf8(&output.stderr)?.to_string(),
        },
    ))
}
