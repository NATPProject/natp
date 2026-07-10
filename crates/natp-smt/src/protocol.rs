//! Solver response classification.
use natp_core::process;
use std::time::Duration;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SolverAnswer<'a> {
    Unsatisfiable,
    Unexpected(&'a str),
}
impl<'a> SolverAnswer<'a> {
    fn parse(text: &'a str) -> Self {
        match text.trim() {
            "unsat" => Self::Unsatisfiable,
            text => Self::Unexpected(text),
        }
    }
}
pub(super) enum Failure<'a> {
    Timeout,
    Exit(Option<i32>),
    Stderr,
    UnexpectedAnswer(&'a str),
}
pub(super) fn failure(output: &process::Output) -> Option<Failure<'_>> {
    if output.timed_out {
        return Some(Failure::Timeout);
    }
    if !output.success {
        return Some(Failure::Exit(output.exit_code));
    }
    if !output.stderr.trim().is_empty() {
        return Some(Failure::Stderr);
    }
    match SolverAnswer::parse(&output.stdout) {
        SolverAnswer::Unsatisfiable => None,
        SolverAnswer::Unexpected(answer) => Some(Failure::UnexpectedAnswer(answer)),
    }
}
impl Failure<'_> {
    pub(super) fn describe(&self, timeout: Duration) -> String {
        match self {
            Self::Timeout => format!(
                "solver timeout after {timeout:?}; increase Smt::timeout or simplify the obligation"
            ),
            Self::Exit(exit_code) => format!(
                "solver exited unsuccessfully (exit code {exit_code:?}); inspect stderr in the report evidence"
            ),
            Self::Stderr => "solver wrote to stderr; inspect the evidence and resolve the diagnostic before trusting its answer".into(),
            Self::UnexpectedAnswer(answer) => format!("solver result was not accepted; stdout: {answer:?}"),
        }
    }
}
