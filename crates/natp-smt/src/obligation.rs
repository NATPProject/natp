//! Inspectable SMT query representation; construction does not establish proof.

/// An SMT query for a base case, induction step, or scalar-only claim.
#[derive(Clone, Debug)]
pub struct Obligation {
    /// Stable role name: `base`, `step`, or `direct`.
    pub name: String,
    /// SMT-LIB query ending in `check-sat`.
    pub source: String,
}
