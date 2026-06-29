//! Inspectable induction plan; every obligation must be checked independently.

use crate::Obligation;

/// SMT queries with an optional induction variable.
#[derive(Clone, Debug)]
pub struct Induction {
    /// The quantified variable selected for induction, if any.
    pub variable: Option<usize>,
    /// Every query must succeed independently before accepting SMT validation.
    pub obligations: Vec<Obligation>,
}
