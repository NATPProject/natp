use crate::args::{Command, HELP, TraceMode};
use natp::{Backend, Bounded, Error, Kernel, Report, Scope, Theorem, kernel};
use std::{
    io::{self, Write},
    path::Path,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CommandOutcome {
    Established,
    Unestablished,
}

impl CommandOutcome {
    fn from_established(established: bool) -> Self {
        if established {
            Self::Established
        } else {
            Self::Unestablished
        }
    }

    fn combine(self, other: Self) -> Self {
        match (self, other) {
            (Self::Established, Self::Established) => Self::Established,
            _ => Self::Unestablished,
        }
    }
}

pub(crate) fn execute(command: Command) -> Result<CommandOutcome, Error> {
    execute_to(command, &mut io::stdout().lock())
}

pub(crate) fn execute_to(
    command: Command,
    output: &mut impl Write,
) -> Result<CommandOutcome, Error> {
    match command {
        Command::Help => {
            write!(output, "{HELP}")?;

            Ok(CommandOutcome::Established)
        }
        Command::Check { theorems, trace } => {
            let mut outcome = CommandOutcome::Established;

            for path in theorems {
                let theorem = Theorem::load(path)?;
                let report = match theorem.scope {
                    Scope::Unbounded => Kernel.prove(&theorem)?,
                    Scope::Bounded { .. } => Bounded::default().prove(&theorem)?,
                };

                print_report(&report, trace, output)?;
                outcome =
                    outcome.combine(CommandOutcome::from_established(report.is_established()));
            }

            Ok(outcome)
        }

        Command::Certify {
            theorem,
            certificate,
        } => certify(&theorem, &certificate, output),
    }
}

fn certify(
    theorem_path: &Path,
    certificate_path: &Path,
    output: &mut impl Write,
) -> Result<CommandOutcome, Error> {
    let theorem = Theorem::load(theorem_path)?;
    let Some(certificate) = kernel::certify(&theorem)? else {
        writeln!(output, "Inconclusive: no kernel certificate")?;

        return Ok(CommandOutcome::Unestablished);
    };

    certificate.save(certificate_path)?;
    writeln!(
        output,
        "Saved checked kernel certificate for {}",
        theorem.name
    )?;

    Ok(CommandOutcome::Established)
}

fn print_report(
    report: &Report,
    trace_mode: TraceMode,
    output: &mut impl Write,
) -> Result<(), Error> {
    writeln!(output, "{report}")?;

    if !report.is_established() {
        writeln!(output, "  {}", report.evidence)?;
    }

    if trace_mode == TraceMode::Show {
        match &report.trace {
            Some(trace) => write!(output, "{}", trace.render())?,
            None => writeln!(output, "  No kernel proof trace.")?,
        }
    }

    Ok(())
}
