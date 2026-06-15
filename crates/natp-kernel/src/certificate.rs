//! Independent certificate checking.

use crate::{Atom, BooleanProof, Formula, boolean, normalise};
use natp_core::{Code, DiagnosticField};
use natp_core::{Error, Scope, Theorem, Trace};
use serde::{Deserialize, Serialize};

/// A proposed proof for a theorem. Call [`check`] before trusting loaded evidence.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Certificate {
    /// Exact theorem, including scope and name, bound to this evidence.
    pub theorem: Theorem,
    /// Canonical proposition recomputed by the trusted normaliser.
    pub normalized: Formula,
    /// Normalisation trace; must match recomputation.
    pub normalization: Trace,
    /// Sorted, unique propositional atoms used by the Boolean deduction.
    pub atoms: Vec<Atom>,
    /// Classical case tree checked independently of proof search.
    pub deduction: BooleanProof,
}

pub(super) fn validate(theorem: &Theorem) -> Result<(), Error> {
    theorem.validate()?;

    if theorem.scope != Scope::Unbounded {
        return Err(Error::new(
            "kernel proofs require Unbounded scope; bounded claims use verification",
        ));
    }

    Ok(())
}

/// Recompute normalisation and check the supplied deduction without proof search.
///
/// # Errors
/// Returns validation errors for malformed or bounded expected theorems.
/// Mismatched or invalid certificate evidence returns `Ok(false)`.
pub fn check(theorem: &Theorem, certificate: &Certificate) -> Result<bool, Error> {
    validate(theorem)?;

    Ok(check_evidence(theorem, certificate).is_ok())
}

/// Recheck a certificate and explain the first rejected evidence component.
///
/// # Errors
/// Returns theorem validation errors or certificate diagnostics naming
/// the mismatched field or invalid deduction branch. Acceptance is identical to [`check`].
pub fn check_detailed(theorem: &Theorem, certificate: &Certificate) -> Result<(), Error> {
    validate(theorem)?;

    check_evidence(theorem, certificate)
}

fn check_evidence(theorem: &Theorem, certificate: &Certificate) -> Result<(), Error> {
    let rejected = |code, path, message| {
        Error::new(message).with_code(code).at(path).with_help(
            "regenerate the certificate for the exact expected theorem; \
                 never edit evidence to force acceptance",
        )
    };

    if certificate.theorem != *theorem {
        let field = if certificate.theorem.schema_version != theorem.schema_version {
            DiagnosticField::SchemaVersion
        } else if certificate.theorem.name != theorem.name {
            DiagnosticField::Name
        } else if certificate.theorem.variables != theorem.variables {
            DiagnosticField::Variables
        } else if certificate.theorem.scope != theorem.scope {
            DiagnosticField::Scope
        } else {
            DiagnosticField::Claim
        };

        return Err(rejected(
            Code::CertificateTheorem,
            DiagnosticField::Theorem.child(field),
            "certificate is bound to a different theorem",
        ));
    }

    let (normal_form, normalisation_trace) = normalise::normalise(&theorem.claim);

    if normal_form != certificate.normalized {
        return Err(rejected(
            Code::CertificateNormalForm,
            DiagnosticField::NormalForm.as_str().into(),
            "certificate normal form differs from trusted normalisation",
        ));
    }

    if normalisation_trace != certificate.normalization {
        return Err(rejected(
            Code::CertificateTrace,
            DiagnosticField::Normalisation.as_str().into(),
            "certificate normalisation trace differs from the recomputed rule trace",
        ));
    }

    let atoms = boolean::atoms(&normal_form);

    if atoms.len() > boolean::MAX_ATOMS || atoms != certificate.atoms {
        return Err(rejected(
            Code::CertificateAtoms,
            DiagnosticField::Atoms.as_str().into(),
            "certificate atom table is not the exact sorted canonical table, or exceeds the atom limit",
        ));
    }

    boolean::diagnose(
        &normal_form,
        &atoms,
        &certificate.deduction,
        &mut Vec::new(),
        DiagnosticField::Deduction.as_str(),
    )
}
