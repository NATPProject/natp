use std::ops;

use natp_core::{NatExpr, Prop};

/// A mathematical natural-number expression; addition never means wrapping arithmetic.
#[derive(Clone, Debug)]
pub struct Nat(NatExpr);

impl Nat {
    pub(super) fn from_expression(expression: NatExpr) -> Self {
        Self(expression)
    }

    /// Construct symbolic equality between two natural-number expressions.
    pub fn equals(self, rhs: impl Into<Self>) -> Prop {
        Prop::NatEq(self.0, rhs.into().0)
    }
}

impl From<u64> for Nat {
    fn from(value: u64) -> Self {
        Self(NatExpr::Constant(value))
    }
}

impl<R: Into<Nat>> ops::Add<R> for Nat {
    type Output = Self;

    fn add(self, rhs: R) -> Self {
        Self(NatExpr::Add(Box::new(self.0), Box::new(rhs.into().0)))
    }
}
