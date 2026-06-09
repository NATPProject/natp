//! Canonical sequence letters.

use serde::{Deserialize, Serialize};

/// A sequence variable, possibly reversed, in a canonical concatenation word.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Letter {
    /// Zero-based sequence quantifier ID.
    pub variable: usize,
    /// Whether this occurrence denotes the variable's reversal.
    pub reversed: bool,
}
