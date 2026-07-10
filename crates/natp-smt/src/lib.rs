//! SMT query data and solver configuration.

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
/// Accept an unambiguous successful unsat response.
pub fn is_unsat(output: &process::Output) -> bool {
    protocol::failure(output).is_none()
}
