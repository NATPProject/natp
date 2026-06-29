//! Generate direct scalar obligations.
use crate::vocabulary::{Binding, ObligationKind, Operator, SmtCommand, Sort};
use crate::{Induction, Obligation, prelude, translate::Translator};
use natp_core::{Error, Scope, Theorem};
/// Negate a validated, unbounded claim.
///
/// # Errors
/// Returns errors for invalid theorems or unsupported scopes, domains, or expressions.
pub fn induction(theorem: &Theorem) -> Result<Induction, Error> {
    theorem.validate()?;
    if theorem.scope != Scope::Unbounded {
        return Err(Error::new("SMT induction requires unbounded scope"));
    }
    let bindings: Vec<_> = (0..theorem.variables.len())
        .map(|id| Binding::Quantified(id).render())
        .collect();
    let mut prelude = prelude::generate();
    for (id, &domain) in theorem.variables.iter().enumerate() {
        prelude.push_str(&SmtCommand::DeclareConstant.render(&[
            Binding::Quantified(id).render(),
            Sort::domain(domain).render(),
        ]));
    }
    let claim = Translator::new(theorem).translate_proposition(&theorem.claim, &bindings)?;
    Ok(Induction {
        variable: None,
        obligations: vec![Obligation {
            name: ObligationKind::Direct.name().into(),
            source: format!(
                "{prelude}{}{}",
                SmtCommand::Assert.render(&[Operator::Not.apply(&[claim])]),
                SmtCommand::CheckSat.render(&[])
            ),
        }],
    })
}
