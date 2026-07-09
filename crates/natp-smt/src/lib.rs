//! SMT-LIB obligations and external solver execution.

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
/// Accept successful, unambiguous unsat output only.
pub fn is_unsat(output: &process::Output) -> bool {
    protocol::is_unsat(output)
}
