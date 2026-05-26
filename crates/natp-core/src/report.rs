use crate::{Assignment, Error, ErrorKind, Scope, Theorem, Trace};
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

/// Stable report outcome category, without outcome-specific evidence payloads.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutcomeKind {
    /// Native unbounded proof.
    Proven,
    /// Finite verification.
    Verified,
    /// External SMT validation.
    SmtValidated,
    /// Concrete refutation.
    Refuted,
    /// No conclusion.
    Inconclusive,
}

impl OutcomeKind {
    /// All outcome categories in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Proven,
        Self::Verified,
        Self::SmtValidated,
        Self::Refuted,
        Self::Inconclusive,
    ];

    /// Stable label matching the serialised variant name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Proven => "Proven",
            Self::Verified => "Verified",
            Self::SmtValidated => "SmtValidated",
            Self::Refuted => "Refuted",
            Self::Inconclusive => "Inconclusive",
        }
    }
}

impl Outcome {
    /// Return the outcome category without its variant-specific data.
    pub const fn kind(&self) -> OutcomeKind {
        match self {
            Self::Proven => OutcomeKind::Proven,
            Self::Verified { .. } => OutcomeKind::Verified,
            Self::SmtValidated => OutcomeKind::SmtValidated,
            Self::Refuted { .. } => OutcomeKind::Refuted,
            Self::Inconclusive { .. } => OutcomeKind::Inconclusive,
        }
    }

    /// Stable outcome label for human-readable evaluation tables.
    pub const fn label(&self) -> &'static str {
        self.kind().as_str()
    }
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

impl Method {
    /// Every backend method, in declaration order.
    pub const ALL: [Self; 3] = [Self::Kernel, Self::NativeBounded, Self::SmtInduction];

    /// Stable label matching the serialised variant name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Kernel => "Kernel",
            Self::NativeBounded => "NativeBounded",
            Self::SmtInduction => "SmtInduction",
        }
    }
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ReportRequirement {
    KernelProof,
    BoundedVerification,
    SmtValidation,
}

impl ReportRequirement {
    const fn description(self) -> &'static str {
        match self {
            Self::KernelProof => "native kernel proof",
            Self::BoundedVerification => "bounded verification",
            Self::SmtValidation => "SMT validation",
        }
    }

    fn accepts(self, report: &Report) -> bool {
        match self {
            Self::KernelProof => report.is_proven(),
            Self::BoundedVerification => report.is_verified(),
            Self::SmtValidation => report.is_smt_validated(),
        }
    }
}

impl Report {
    /// Test for an unbounded native-kernel result with consistent method and scope.
    pub fn is_proven(&self) -> bool {
        self.outcome == Outcome::Proven
            && self.method == Method::Kernel
            && self.theorem.scope == Scope::Unbounded
    }

    /// Test for a finite result whose reported bound matches the theorem.
    pub fn is_verified(&self) -> bool {
        let (Outcome::Verified { max_length }, Scope::Bounded { max_len }) =
            (&self.outcome, self.theorem.scope)
        else {
            return false;
        };

        *max_length == max_len && self.method == Method::NativeBounded
    }

    /// Test for an unbounded result explicitly trusting the external SMT solver.
    pub fn is_smt_validated(&self) -> bool {
        self.outcome == Outcome::SmtValidated
            && self.method == Method::SmtInduction
            && self.theorem.scope == Scope::Unbounded
    }

    /// Accepts any successful method without promoting its outcome.
    pub fn is_established(&self) -> bool {
        self.is_proven() || self.is_verified() || self.is_smt_validated()
    }

    /// Require native proof without panicking.
    ///
    /// # Errors
    /// Returns [`ErrorKind::UnexpectedOutcome`] unless [`Self::is_proven`] holds.
    pub fn require_proven(&self) -> Result<(), Error> {
        self.require(ReportRequirement::KernelProof)
    }

    /// Require finite verification without panicking.
    ///
    /// # Errors
    /// Returns [`ErrorKind::UnexpectedOutcome`] unless [`Self::is_verified`] holds.
    pub fn require_verified(&self) -> Result<(), Error> {
        self.require(ReportRequirement::BoundedVerification)
    }

    /// Require SMT validation, returning an error for other outcomes.
    ///
    /// # Errors
    /// Returns [`ErrorKind::UnexpectedOutcome`] unless [`Self::is_smt_validated`] holds.
    pub fn require_smt_validated(&self) -> Result<(), Error> {
        self.require(ReportRequirement::SmtValidation)
    }

    fn require(&self, requirement: ReportRequirement) -> Result<(), Error> {
        if requirement.accepts(self) {
            Ok(())
        } else {
            Err(Error::with_kind(
                ErrorKind::UnexpectedOutcome,
                format!("expected {}: {self}", requirement.description()),
            ))
        }
    }

    /// Assert native proof in a test.
    ///
    /// # Panics
    /// Panics unless [`Self::is_proven`] holds. Applications can use [`Self::require_proven`].
    pub fn assert_proven(&self) {
        assert!(self.is_proven(), "{self}");
    }

    /// Alias for [`Self::assert_proven`], with the same panic contract.
    ///
    /// # Panics
    /// Panics unless this report represents a native kernel proof.
    pub fn assert_unbounded(&self) {
        self.assert_proven();
    }

    /// Assert a consistent finite verification result in a test.
    ///
    /// # Panics
    /// Panics unless [`Self::is_verified`] holds; see [`Self::require_verified`].
    pub fn assert_verified(&self) {
        assert!(self.is_verified(), "{self}");
    }

    /// Assert that the report represents SMT validation.
    ///
    /// # Panics
    /// Panics unless [`Self::is_smt_validated`] holds; see [`Self::require_smt_validated`].
    pub fn assert_smt_validated(&self) {
        assert!(self.is_smt_validated(), "{self}");
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
