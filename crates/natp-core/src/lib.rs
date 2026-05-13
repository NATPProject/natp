//! Theorem representation, evaluation, validation, and backend interface.

mod error;
mod logic;

pub use error::ErrorKind;
pub use logic::*;
pub use natp_diagnostics::{Code, Diagnostic, diagnostic};
