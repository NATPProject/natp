//! Theorem representation, evaluation, validation, and backend interface.

mod catalogue;
mod error;
pub mod limits;
mod logic;

pub use catalogue::DiagnosticField;
pub use error::{Error, ErrorKind};
pub use logic::*;
pub use natp_diagnostics::{Code, Diagnostic, diagnostic};
