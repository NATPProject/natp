//! Canonical proposition representations.

use crate::{Letter, Linear};
use natp_core::ElemExpr;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
/// An opaque proposition after trusted finite-list normalisation.
pub enum Atom {
    /// Equality of canonical words, modulo equality symmetry and reversal.
    SequenceEqual(Vec<Letter>, Vec<Letter>),
    /// Equality of reduced natural linear expressions.
    LengthEqual(Linear, Linear),
    /// Emptiness of a sequence variable.
    Empty(usize),
    /// Equality of canonical scalar terms.
    ElementEqual(ElemExpr, ElemExpr),
    /// Membership in a sequence variable.
    Contains(usize, ElemExpr),
    /// A predicate holding for every element of a sequence variable.
    All(usize, Box<Formula>),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
/// Canonical propositional syntax serialised in kernel certificates.
pub enum Formula {
    /// Constant truth value.
    Bool(bool),
    /// An opaque, independently assigned atom.
    Atom(Atom),
    /// Negation.
    Not(Box<Formula>),
    /// Sorted, deduplicated conjunction.
    And(Vec<Formula>),
    /// Sorted, deduplicated disjunction.
    Or(Vec<Formula>),
}
