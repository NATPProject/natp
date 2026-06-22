//! Claims determined by a length vector.
use natp_core::Prop;
pub(super) fn is_value_independent(proposition: &Prop) -> bool {
    matches!(
        proposition,
        Prop::Bool(_) | Prop::NatEq(..) | Prop::IsEmpty(_)
    )
}
