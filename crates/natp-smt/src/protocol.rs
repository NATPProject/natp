//! Accept only successful, unambiguous solver answers.
use natp_core::process;
pub(super) fn is_unsat(output: &process::Output) -> bool {
    output.success
        && !output.timed_out
        && output.stderr.trim().is_empty()
        && output.stdout.trim() == "unsat"
}
