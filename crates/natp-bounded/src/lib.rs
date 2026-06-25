//! Finite-length verification by equality-pattern enumeration.
//!
//! General propositions enumerate equality patterns, including aliases.
//! Resource exhaustion is inconclusive; successful finite checks return Verified.

mod budget;
mod configuration;
mod input_variable;
mod inputs;
mod patterns;
mod policy;
mod runner;
mod search;
mod stop;
mod symbolic;

pub use configuration::Bounded;
