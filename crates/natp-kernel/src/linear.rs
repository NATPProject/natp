//! Canonical sequence-length coefficients.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A natural constant plus nonnegative multiples of input sequence lengths.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Linear {
    /// Constant term; validated syntax bounds prevent overflow.
    pub constant: u128,
    /// Nonzero coefficients keyed by sequence quantifier ID.
    pub lengths: BTreeMap<usize, u64>,
}
