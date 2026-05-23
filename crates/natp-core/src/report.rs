use crate::Assignment;
use serde::{Deserialize, Serialize};
/// A backend result, including its evidence category and quantification scope.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Outcome {
    /// An unbounded theorem checked by the native trusted kernel only.
    Proven,
    /// Verification within a finite sequence-length bound.
    Verified {
        /// Inclusive bound checked independently for each sequence variable.
        max_length: usize,
    },
    /// External solver induction, not a kernel proof.
    SmtValidated,
    /// A concrete assignment falsifies the claim under its declared scope.
    Refuted {
        /// Values in theorem quantifier order, suitable for [`crate::Theorem::evaluate`].
        inputs: Vec<Assignment>,
    },
    /// No conclusion was established by the selected procedure.
    Inconclusive {
        /// A diagnostic describing incompleteness, solver uncertainty, or exhaustion.
        reason: String,
    },
}

/// The procedure and trust boundary responsible for a report.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Method {
    /// Native normalisation and independently checked Boolean deduction.
    Kernel,
    /// Exhaustive verification within finite sequence bounds.
    NativeBounded,
    /// External SMT queries, possibly using sequence induction.
    SmtInduction,
}
