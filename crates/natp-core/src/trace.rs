//! Portable rule applications and deterministic tree rendering.

use serde::{Deserialize, Serialize};

/// A rule application and its premises.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Trace {
    /// Stable rule identifier from the kernel rule catalogue.
    pub rule: String,
    /// Description of the rule input; normalisation traces use Debug formatting.
    pub before: String,
    /// Description of the rule result; normalisation traces use Debug formatting.
    pub after: String,
    /// Rule applications supporting this step, in deterministic order.
    pub premises: Vec<Trace>,
}
