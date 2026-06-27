//! Inspectable induction plan; every obligation must be checked independently.

use crate::Obligation;

/// Induction on one sequence with all other quantified inputs left arbitrary.
#[derive(Clone, Debug)]
pub struct Induction {
    /// The quantified variable selected for induction, or None for a scalar-only claim.
    pub variable: Option<usize>,
    /// Every query must succeed independently before accepting SMT validation.
    pub obligations: Vec<Obligation>,
}
