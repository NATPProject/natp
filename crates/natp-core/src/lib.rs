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
