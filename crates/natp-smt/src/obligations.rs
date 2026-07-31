//! Generate base, step, and scalar obligations.

use natp_core::{Domain, Error, Scope, Theorem};

use crate::vocabulary::{Binding, ListSymbol, ObligationKind, Operator, SmtCommand, Sort};
use crate::{Induction, Obligation, prelude, translate::Translator};

/// Choose the first sequence for induction, or a direct query for scalar-only claims.
///
/// # Errors
/// Returns validation errors for malformed or bounded theorems.
pub fn induction(theorem: &Theorem) -> Result<Induction, Error> {
    theorem.validate()?;

    if theorem.scope != Scope::Unbounded {
        return Err(Error::new("SMT induction requires unbounded scope"));
    }

    let variable = theorem
        .variables
        .iter()
        .position(|domain| matches!(domain, Domain::Sequence(_)));
    let bindings: Vec<_> = (0..theorem.variables.len())
        .map(|id| Binding::Quantified(id).render())
        .collect();
    let prelude = declare_parameters(theorem, variable);
    let obligations = match variable {
        Some(variable) => sequence_cases(theorem, &bindings, &prelude, variable),
        None => {
            let claim = Translator::new(theorem).translate_proposition(&theorem.claim, &bindings);

            vec![Obligation {
                name: ObligationKind::Direct.name().into(),
                source: format!(
                    "{prelude}{}{}",
                    SmtCommand::Assert.render(&[Operator::Not.apply(&[claim])]),
                    SmtCommand::CheckSat.render(&[])
                ),
            }]
        }
    };

    Ok(Induction {
        variable,
        obligations,
    })
}

/// The induction variable is replaced by nil, t, or cons(h,t), never a free vN.
fn declare_parameters(theorem: &Theorem, induction_variable: Option<usize>) -> String {
    let mut prelude = prelude::generate();

    for (id, &domain) in theorem.variables.iter().enumerate() {
        if Some(id) != induction_variable {
            prelude.push_str(&SmtCommand::DeclareConstant.render(&[
                Binding::Quantified(id).render(),
                Sort::domain(domain).render(),
            ]));
        }
    }

    prelude
}

fn sequence_cases(
    theorem: &Theorem,
    bindings: &[String],
    prelude: &str,
    variable: usize,
) -> Vec<Obligation> {
    let translator = Translator::new(theorem);
    let element = theorem.variables[variable].element();
    let head = Binding::Head.render();
    let tail = Binding::Tail.render();

    let mut substituted = bindings.to_vec();
    substituted[variable] = ListSymbol::Empty.render(element);
    let base = translator.translate_proposition(&theorem.claim, &substituted);

    substituted[variable] = tail.clone();
    let hypothesis = translator.translate_proposition(&theorem.claim, &substituted);

    substituted[variable] = ListSymbol::Cons.apply(element, &[head.clone(), tail.clone()]);
    let conclusion = translator.translate_proposition(&theorem.claim, &substituted);

    vec![
        Obligation {
            name: ObligationKind::Base.name().into(),
            source: format!(
                "{prelude}; Negated base case; all other variables arbitrary\n{}{}",
                SmtCommand::Assert.render(&[Operator::Not.apply(&[base])]),
                SmtCommand::CheckSat.render(&[]),
            ),
        },
        Obligation {
            name: ObligationKind::Step.name().into(),
            source: format!(
                "{prelude}{}{}; Induction hypothesis at t, with arbitrary parameters\n{}{}{}",
                SmtCommand::DeclareConstant.render(&[head, Sort::element(element).render()]),
                SmtCommand::DeclareConstant.render(&[tail, Sort::List(element).render()]),
                SmtCommand::Assert.render(&[hypothesis]),
                SmtCommand::Assert.render(&[Operator::Not.apply(&[conclusion])]),
                SmtCommand::CheckSat.render(&[]),
            ),
        },
    ]
}
