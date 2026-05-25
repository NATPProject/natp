use crate::{Assignment, Scope, Theorem, Trace};
use serde::{Deserialize, Serialize};
use std::fmt;

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
        /// Values in theorem quantifier order, suitable for [`Theorem::evaluate`].
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

/// Evidence returned by a backend. Inspect the outcome before using a claim.
///
/// Reports are descriptive, publicly constructible values, not authenticated
/// proof tokens. Use the kernel certificate checker at an external trust boundary.
#[derive(Clone, Debug, PartialEq, Eq)]
#[must_use = "inspect the outcome; bounded verification and SMT validation are not kernel proofs"]
pub struct Report {
    /// Exact theorem submitted to the backend.
    pub theorem: Theorem,
    /// The procedure that generated this report.
    pub method: Method,
    /// The conclusion and the scope it covers.
    pub outcome: Outcome,
    /// Procedure-specific diagnostics, counters, or solver responses.
    pub evidence: String,
    /// Optional native derivation trace; absent for non-kernel evidence.
    pub trace: Option<Trace>,
}

impl Report {
    /// Test for an unbounded native-kernel result with consistent method and scope.
    pub fn is_proven(&self) -> bool {
        self.outcome == Outcome::Proven
            && self.method == Method::Kernel
            && self.theorem.scope == Scope::Unbounded
    }
}

impl fmt::Display for Report {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let scope = match self.theorem.scope {
            Scope::Unbounded => "all finite lengths".to_owned(),
            Scope::Bounded { max_len } => format!("each sequence length 0..={max_len} only"),
        };
        write!(
            formatter,
            "{} ... {:?} [{}; {:?}]",
            self.theorem.name, self.outcome, scope, self.method
        )
    }
}
