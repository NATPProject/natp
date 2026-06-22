//! Value-independent bounded claims.

use natp_core::Prop;

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
