//! Check sequence equality, length, and emptiness without enumerating element values.

use crate::{budget::spend_work, inputs::InputLayout, stop::SearchStop};
use natp_core::{Expr, Prop};

/// Interpret sequence terms as words of independent input positions.
/// This establishes arbitrary-value validity only for unconditional equality.
pub(super) fn positions(
    expression: &Expr,
    inputs: &InputLayout,
    work: &mut usize,
) -> Result<Vec<usize>, SearchStop> {
    spend_work(work, 1)?;

    match expression {
        Expr::Empty(_) => Ok(Vec::new()),
        Expr::Var(variable) => {
            let positions = inputs.positions(*variable);
            spend_work(work, positions.len())?;

            Ok(positions.collect())
        }
        Expr::Reverse(_) => Err(SearchStop::Failed(natp_core::Error::new(
            "sequence operation is not supported symbolically",
        ))),
        Expr::Append(_, _) => Err(SearchStop::Failed(natp_core::Error::new(
            "sequence operation is not supported symbolically",
        ))),
    }
}

/// Length and emptiness propositions depend only on the current length vector.
pub(super) fn is_value_independent(proposition: &Prop) -> bool {
    match proposition {
        Prop::Bool(_) | Prop::NatEq(..) | Prop::IsEmpty(_) => true,
        Prop::Not(predicate) => is_value_independent(predicate),
        Prop::And(left, right) | Prop::Or(left, right) | Prop::Implies(left, right) => {
            is_value_independent(left) && is_value_independent(right)
        }
        _ => false,
    }
}

pub(super) fn supports(expression: &Expr) -> bool {
    matches!(expression, Expr::Empty(_) | Expr::Var(_))
}
