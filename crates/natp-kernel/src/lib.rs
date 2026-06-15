//! Sequence normalisation and independently checked proof certificates.

mod boolean;
mod certificate;
mod linear;
mod normalise;
mod trace;
mod word;

pub use boolean::BooleanProof;
pub use certificate::{Certificate, check, check_detailed};
pub use linear::Linear;
pub use normalise::{Atom, Formula};
pub use word::Letter;
