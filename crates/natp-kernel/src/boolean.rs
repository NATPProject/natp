//! Propositional deduction trees.

use serde::{Deserialize, Serialize};

/// A classical deduction that can close early only on a true partial valuation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum BooleanProof {
    /// The proposition evaluates to true under the current partial assignment.
    True,
}
