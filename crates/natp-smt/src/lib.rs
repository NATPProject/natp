//! SMT-LIB translation and structural induction obligations.
//! Successful solver answers are SmtValidated, never native kernel Proven.

mod configuration;
mod export;
mod induction;
mod obligation;
mod obligations;
mod prelude;
mod protocol;
mod runner;
mod translate;
mod vocabulary;

pub use configuration::Smt;
pub use export::write_obligations;
pub use induction::Induction;
pub use obligation::Obligation;
pub use obligations::induction;

use natp_core::process;

/// Return true for `unsat` with a successful exit, no timeout, and empty stderr.
pub fn is_unsat(output: &process::Output) -> bool {
    protocol::failure(output).is_none()
}
