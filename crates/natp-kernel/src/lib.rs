//! Canonical sequence representations.

mod linear;
mod normalise;
mod word;

pub use linear::Linear;
pub use normalise::{Atom, Formula};
pub use word::Letter;
