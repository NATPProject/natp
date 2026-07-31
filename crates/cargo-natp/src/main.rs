//! Check theorem files and certificates, and run or export SMT queries.

mod args;
mod commands;

use std::{
    env,
    io::{self, Write},
    process::ExitCode,
};

#[derive(Clone, Copy)]
#[repr(u8)]
enum Status {
    Established = 0,
    Unestablished = 1,
    Error = 2,
}

impl From<Status> for ExitCode {
    fn from(status: Status) -> Self {
        Self::from(status as u8)
    }
}

fn main() -> ExitCode {
    let result = args::Command::parse(env::args_os().skip(1).collect()).and_then(commands::execute);

    match result {
        Ok(commands::CommandOutcome::Established) => Status::Established.into(),
        Ok(commands::CommandOutcome::Unestablished) => Status::Unestablished.into(),
        Err(error) => {
            // A closed diagnostic stream must not turn an ordinary CLI error into a panic.
            let _ = writeln!(io::stderr().lock(), "error: {error}");

            Status::Error.into()
        }
    }
}
