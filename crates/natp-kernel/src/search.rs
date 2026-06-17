//! Untrusted proof search. Its output must pass `certificate::check`.

use crate::{Atom, BooleanProof, Certificate, Formula, boolean, certificate, normalise};
use natp_core::{Error, Theorem};

/// Propose a certificate for an unbounded theorem, or report an incomplete search.
///
/// # Errors
/// Returns validation errors for malformed or bounded theorems. Incomplete
/// search returns `Ok(None)` and makes no claim about the theorem's truth.
pub fn certify(theorem: &Theorem) -> Result<Option<Certificate>, Error> {
    certificate::validate(theorem)?;
    let (normal_form, normalisation_trace) = normalise::normalise(&theorem.claim);
    let atoms = boolean::atoms(&normal_form);

    if atoms.len() > boolean::MAX_ATOMS {
        return Ok(None);
    }

    let Some(deduction) = search(&normal_form, &atoms, &mut Vec::new()) else {
        return Ok(None);
    };

    Ok(Some(Certificate {
        theorem: theorem.clone(),
        normalized: normal_form,
        normalization: normalisation_trace,
        atoms,
        deduction,
    }))
}

fn search(formula: &Formula, atoms: &[Atom], values: &mut Vec<bool>) -> Option<BooleanProof> {
    match boolean::evaluate(formula, atoms, values) {
        Some(true) => return Some(BooleanProof::True),
        Some(false) => return None,
        None => {}
    }

    let atom = values.len();

    if atom >= atoms.len() {
        return None;
    }

    let on_false = search_branch(formula, atoms, values, false)?;
    let on_true = search_branch(formula, atoms, values, true)?;

    Some(BooleanProof::Cases {
        atom,
        on_false: Box::new(on_false),
        on_true: Box::new(on_true),
    })
}

fn search_branch(
    formula: &Formula,
    atoms: &[Atom],
    values: &mut Vec<bool>,
    assumption: bool,
) -> Option<BooleanProof> {
    values.push(assumption);
    let proof = search(formula, atoms, values);
    values.pop();

    proof
}
