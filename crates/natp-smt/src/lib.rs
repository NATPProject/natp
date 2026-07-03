//! SMT-LIB obligations and external solver execution.

mod configuration;
mod induction;
mod obligation;
mod obligations;
mod prelude;
mod translate;
mod vocabulary;

pub use configuration::Smt;
pub use induction::Induction;
pub use obligation::Obligation;
pub use obligations::induction;
