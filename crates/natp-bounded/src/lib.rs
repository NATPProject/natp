//! Finite-length verification with resource limits.

mod budget;
mod configuration;
mod input_variable;
mod inputs;
mod patterns;
mod policy;
mod runner;
mod search;
mod stop;

pub use configuration::Bounded;
