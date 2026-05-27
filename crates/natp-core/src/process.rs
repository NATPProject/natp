//! Bounded external-process execution with file-backed output to avoid pipe stalls.
//! Attempts to kill and reap the direct child on timeout or early failure.

mod child;
mod output;

use child::RunningChild;
pub use output::Output;

use crate::{Error, ErrorKind, limits::ResourceLimit};
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

const MAX_OUTPUT_BYTES: u64 = ResourceLimit::ProcessOutputBytes.value() as u64;

/// Timing settings for the direct-child process runner.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExecutionSetting {
    /// Sleep interval between polls, shortened to the remaining deadline.
    PollInterval,
}

impl ExecutionSetting {
    /// Every process timing setting.
    pub const ALL: [Self; 1] = [Self::PollInterval];

    /// Default duration associated with this execution setting.
    pub const fn duration(self) -> Duration {
        match self {
            Self::PollInterval => Duration::from_millis(10),
        }
    }
}

/// Run a directly spawned command with a deadline and bounded captured output.
///
/// Standard input is closed. A zero timeout never starts a child. Output limits
/// are enforced when reading captured output. Descendant processes are not managed.
///
/// # Errors
/// Returns an I/O error for spawn, wait, or UTF-8 decoding failure, or a resource
/// error if either output exceeds one MiB. Timed-out evidence has `timed_out` set.
pub fn run(command: &mut Command, timeout: Duration) -> Result<Output, Error> {
    let program = command.get_program().to_string_lossy().into_owned();

    run_with_output_limits(command, timeout)
        .map_err(|error| error.context(format!("execute {program}")))
}

fn run_with_output_limits(command: &mut Command, timeout: Duration) -> Result<Output, Error> {
    if timeout.is_zero() {
        return Ok(Output {
            success: false,
            exit_code: None,
            timed_out: true,
            stdout: String::new(),
            stderr: String::new(),
        });
    }

    let mut stdout = tempfile::tempfile()?;
    let mut stderr = tempfile::tempfile()?;
    let mut child = RunningChild::spawn(
        command
            .stdin(Stdio::null())
            .stdout(stdout.try_clone()?)
            .stderr(stderr.try_clone()?),
    )?;
    let start = Instant::now();

    let (status, timed_out) = loop {
        if let Some(status) = child.try_wait()? {
            break (status, false);
        }

        if start.elapsed() >= timeout {
            break (child.terminate()?, true);
        }

        thread::sleep(
            ExecutionSetting::PollInterval
                .duration()
                .min(timeout.saturating_sub(start.elapsed())),
        );
    };

    Ok(Output {
        success: status.success(),
        exit_code: status.code(),
        timed_out,
        stdout: read_output(&mut stdout)?,
        stderr: read_output(&mut stderr)?,
    })
}

fn output_limit_error() -> Error {
    Error::with_kind(
        ErrorKind::ResourceLimit,
        "backend output exceeded 1 MiB; refusing truncated evidence",
    )
}

fn read_output(file: &mut File) -> Result<String, Error> {
    file.seek(SeekFrom::Start(0))?;
    let mut bytes = Vec::new();
    file.take(MAX_OUTPUT_BYTES + 1).read_to_end(&mut bytes)?;

    if bytes.len() as u64 > MAX_OUTPUT_BYTES {
        return Err(output_limit_error());
    }

    String::from_utf8(bytes)
        .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error).into())
}
