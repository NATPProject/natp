//! Theorem representation, evaluation, validation, and backend interface.

mod error;
pub mod limits;
mod logic;

pub use error::{Error, ErrorKind};
pub use logic::*;
pub use natp_diagnostics::{Code, Diagnostic, diagnostic};
