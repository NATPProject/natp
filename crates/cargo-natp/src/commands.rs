use crate::args::{Command, HELP, TraceMode};
use natp::{Backend, Bounded, Error, Kernel, Report, Scope, Smt, Theorem, kernel};
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

enum CertificateStatus {
    Accepted,
    Rejected,
}

impl CertificateStatus {
    fn label(self) -> &'static str {
        match self {
            Self::Accepted => "ACCEPTED (unbounded)",
            Self::Rejected => "REJECTED",
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
        Command::CheckCertificate {
            theorem,
            certificate,
        } => check_certificate(&theorem, &certificate, output),
        Command::Smt {
            theorem,
            executable,
        } => {
            let theorem = Theorem::load(theorem)?;
            let mut solver = Smt::default();

            if let Some(executable) = executable {
                solver.executable = executable;
            }

            let report = solver.prove(&theorem)?;
            writeln!(output, "{report}\n{}", report.evidence)?;

            Ok(CommandOutcome::from_established(report.is_smt_validated()))
        }
        Command::EmitSmt { theorem, directory } => {
            natp::smt::write_obligations(&Theorem::load(theorem)?, directory)?;
            writeln!(
                output,
                "Exported obligations. No proof or validation has been performed."
            )?;

            Ok(CommandOutcome::Established)
        }
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

fn check_certificate(
    theorem_path: &Path,
    certificate_path: &Path,
    output: &mut impl Write,
) -> Result<CommandOutcome, Error> {
    let theorem = Theorem::load(theorem_path)?;
    let certificate = kernel::Certificate::load(certificate_path)?;
    // Keep rejected certificates at exit code 1 while showing why they failed.
    let diagnosis = kernel::check_detailed(&theorem, &certificate);
    let valid = diagnosis.is_ok();
    let status = if valid {
        CertificateStatus::Accepted
    } else {
        CertificateStatus::Rejected
    };

    writeln!(output, "Kernel certificate: {}", status.label())?;

    if let Err(error) = diagnosis {
        if !error.code().is_certificate_rejection() {
            return Err(error);
        }

        writeln!(output, "{error}")?;
    }

    Ok(CommandOutcome::from_established(valid))
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
