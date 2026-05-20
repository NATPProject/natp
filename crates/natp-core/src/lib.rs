//! Theorem representation, evaluation, validation, and backend interface.

mod catalogue;
mod error;
mod eval;
pub mod limits;
mod logic;
mod theorem;
mod validate;

pub use catalogue::{DiagnosticField, SyntaxEdge, TraceRule};
pub use error::{Error, ErrorKind};
pub use logic::*;
pub use natp_diagnostics::{Code, Diagnostic, diagnostic};
pub use theorem::Theorem;
