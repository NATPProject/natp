//! Portable theorem representation and validated persistence.

use crate::{Domain, Error, Prop, Scope, limits, storage};
use serde::{Deserialize, Serialize};
use std::path::Path;

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

impl Theorem {
    /// Validate and serialise a theorem.
    ///
    /// # Errors
    /// Returns validation, size-limit, serialisation, or atomic-write errors.
    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        self.validate()
            .map_err(|error| error.context(format!("save {}", path.as_ref().display())))?;

        storage::save_json(self, path.as_ref(), limits::MAX_THEOREM_BYTES)
    }
}
