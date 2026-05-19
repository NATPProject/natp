//! Portable theorem representation.

use crate::{Domain, Prop, Scope};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
/// Portable logical IR. Call [`Self::validate`] after direct construction or mutation.
/// All built-in backends validate before interpreting this representation.
pub struct Theorem {
    /// Wire-format version; must equal [`crate::limits::SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Human-readable, nonempty name bounded by [`crate::limits::MAX_NAME_BYTES`].
    pub name: String,
    /// Ordered quantifiers, addressed by zero-based variable IDs.
    pub variables: Vec<Domain>,
    /// Quantification scope; successful finite checks never become kernel proofs.
    pub scope: Scope,
    /// Symbolic proposition to establish for every permitted assignment.
    pub claim: Prop,
}
