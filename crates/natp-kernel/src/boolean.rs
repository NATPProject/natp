//! Trusted propositional evaluation and deduction checking.

use std::collections::BTreeSet;

use natp_core::{Code, DiagnosticField, Error, limits::ResourceLimit};
use serde::{Deserialize, Serialize};

use crate::normalise::{Atom, Formula};

pub(super) const MAX_ATOMS: usize = ResourceLimit::KernelAtoms.value();

/// A classical deduction that can close early only on a true partial valuation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum BooleanProof {
    /// The proposition evaluates to true under the current partial assignment.
    True,
    /// Exhaust both assignments to the next atom in the certificate's atom table.
    Cases {
        /// Must equal the current depth and be in range.
        atom: usize,
        /// Deduction under the false assignment.
        on_false: Box<BooleanProof>,
        /// Deduction under the true assignment.
        on_true: Box<BooleanProof>,
    },
}

pub(super) fn atoms(formula: &Formula) -> Vec<Atom> {
    fn gather(formula: &Formula, out: &mut BTreeSet<Atom>) {
        match formula {
            Formula::Atom(a) => {
                out.insert(a.clone());
            }
            Formula::Not(p) => gather(p, out),
            Formula::And(ps) | Formula::Or(ps) => {
                for p in ps {
                    gather(p, out);
                }
            }
            Formula::Bool(_) => {}
        }
    }

    let mut out = BTreeSet::new();
    gather(formula, &mut out);

    out.into_iter().collect()
}

pub(super) fn evaluate(formula: &Formula, atoms: &[Atom], values: &[bool]) -> Option<bool> {
    match formula {
        Formula::Bool(b) => Some(*b),
        Formula::Atom(a) => atoms
            .binary_search(a)
            .ok()
            .and_then(|i| values.get(i).copied()),
        Formula::Not(p) => evaluate(p, atoms, values).map(|v| !v),
        Formula::And(ps) | Formula::Or(ps) => {
            let conjunction = matches!(formula, Formula::And(_));
            let mut unknown = false;

            for p in ps {
                match evaluate(p, atoms, values) {
                    Some(v) if v != conjunction => return Some(v),
                    None => unknown = true,
                    _ => {}
                }
            }

            if unknown { None } else { Some(conjunction) }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum BooleanStatus {
    True,
    False,
    Undetermined,
}

impl BooleanStatus {
    fn from_evaluation(value: Option<bool>) -> Self {
        match value {
            Some(true) => Self::True,
            Some(false) => Self::False,
            None => Self::Undetermined,
        }
    }

    pub(super) const fn as_str(self) -> &'static str {
        match self {
            Self::True => "true",
            Self::False => "false",
            Self::Undetermined => "undetermined",
        }
    }
}

pub(super) fn diagnose(
    formula: &Formula,
    atoms: &[Atom],
    proof: &BooleanProof,
    values: &mut Vec<bool>,
    path: &str,
) -> Result<(), Error> {
    // `values` is always a prefix of the ordered atom table. Sequential splits
    // prevent repeated assumptions or skipped variables from hiding a branch.
    match proof {
        BooleanProof::True => match evaluate(formula, atoms, values) {
            Some(true) => Ok(()),
            status => Err(Error::new(format!(
                "True leaf is unjustified: proposition is {} after {} atom assignments",
                BooleanStatus::from_evaluation(status).as_str(),
                values.len(),
            ))
            .with_code(Code::CertificateDeduction)
            .at(path)
            .with_help(
                "every True leaf must follow from its branch assumptions; regenerate the certificate",
            )),
        },
        BooleanProof::Cases {
            atom,
            on_false,
            on_true,
        } => {
            if *atom != values.len() || *atom >= atoms.len() {
                return Err(Error::new(format!(
                    "invalid split atom {atom}; expected the next atom {} in a table of {}",
                    values.len(),
                    atoms.len(),
                ))
                .with_code(Code::CertificateDeduction)
                .at(format!("{path}.{}", DiagnosticField::Cases.child(DiagnosticField::Atom)))
                .with_help(
                    "splits must assign atoms in order without skipping or repeating one; regenerate the certificate",
                ));
            }

            values.push(false);
            let left = diagnose(
                formula,
                atoms,
                on_false,
                values,
                &format!("{path}.{}", DiagnosticField::Cases.child(DiagnosticField::OnFalse)),
            );

            values[*atom] = true;
            let right = if left.is_ok() {
                diagnose(
                    formula,
                    atoms,
                    on_true,
                    values,
                    &format!("{path}.{}", DiagnosticField::Cases.child(DiagnosticField::OnTrue)),
                )
            } else {
                Ok(())
            };

            values.pop();

            left.and(right)
        }
    }
}
