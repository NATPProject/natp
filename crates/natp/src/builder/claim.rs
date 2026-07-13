use crate::{Backend, Error, Report, Theorem};

/// A constructed theorem ready for backend submission; construction establishes no proof.
#[derive(Clone, Debug)]
#[must_use = "building a claim does not establish its truth"]
pub struct Claim {
    theorem: Theorem,
}

impl Claim {
    pub(super) fn new(theorem: Theorem) -> Self {
        Self { theorem }
    }

    /// Borrow the theorem representation.
    pub fn theorem(&self) -> &Theorem {
        &self.theorem
    }

    /// Return the theorem representation, consuming the claim.
    pub fn into_theorem(self) -> Theorem {
        self.theorem
    }

    /// Submit the claim to the given backend.
    ///
    /// # Errors
    /// Returns validation, scope, or execution errors reported by the backend.
    pub fn prove_with(&self, backend: &impl Backend) -> Result<Report, Error> {
        backend.prove(&self.theorem)
    }
}
