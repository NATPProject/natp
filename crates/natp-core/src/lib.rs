//! Theorem representation, evaluation, validation, and backend interface.

mod catalogue;
mod error;
mod eval;
pub mod limits;
mod logic;
mod report;
pub mod storage;
mod theorem;
mod trace;
mod validate;

pub use catalogue::{DiagnosticField, SyntaxEdge, TraceRule};
pub use error::{Error, ErrorKind};
pub use logic::*;
pub use natp_diagnostics::{Code, Diagnostic, diagnostic};
pub use report::*;
pub use theorem::Theorem;
pub use trace::Trace;

/// A backend that validates a theorem and returns its result and evidence.
pub trait Backend {
    /// Validate a theorem and return a report describing the result and evidence.
    ///
    /// # Errors
    /// Returns an error for invalid input or infrastructure failure. A valid claim
    /// that cannot be established returns an inconclusive report instead.
    fn prove(&self, theorem: &Theorem) -> Result<Report, Error>;
}
