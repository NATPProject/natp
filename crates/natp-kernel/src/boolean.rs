//! Propositional deduction trees.

use serde::{Deserialize, Serialize};

/// A classical deduction that can close early only on a true partial valuation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum BooleanProof {
    /// The proposition evaluates to true under the current partial assignment.
    True,
    /// Exhaust both assignments to the next atom in the certificate's atom table.
    Cases {
        /// Must equal the current depth and be in range.
        atom: usize,
        /// Deduction under the false assignment.
        on_false: Box<BooleanProof>,
        /// Deduction under the true assignment.
        on_true: Box<BooleanProof>,
    },
}
