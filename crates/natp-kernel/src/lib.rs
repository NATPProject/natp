//! Canonical sequence representations.

mod boolean;
mod linear;
mod normalise;
mod word;

pub use boolean::BooleanProof;
pub use linear::Linear;
pub use normalise::{Atom, Formula};
pub use word::Letter;
