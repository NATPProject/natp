use super::{SymbolicDomain, SymbolicElement, sealed};
use natp_core::{Domain, ElemExpr, Prop};
use std::marker::PhantomData;

/// A typed symbolic scalar, either quantified or literal.
#[derive(Debug)]
pub struct Elem<T> {
    expression: ElemExpr,
    marker: PhantomData<T>,
}

impl<T> Clone for Elem<T> {
    fn clone(&self) -> Self {
        Self {
            expression: self.expression.clone(),
            marker: PhantomData,
        }
    }
}

impl<T: SymbolicElement> Elem<T> {
    pub(super) fn new(expression: ElemExpr) -> Self {
        Self {
            expression,
            marker: PhantomData,
        }
    }

    /// Construct equality with another symbolic element or a same-sort literal.
    pub fn equals(self, rhs: impl Into<Self>) -> Prop {
        Prop::ElemEq(self.expression, rhs.into().expression)
    }
}

impl<T: SymbolicElement> sealed::Domain for Elem<T> {}

impl<T: SymbolicElement> SymbolicDomain for Elem<T> {
    const DOMAIN: Domain = Domain::Scalar(T::ELEMENT);
    fn variable(id: usize) -> Self {
        Self::new(ElemExpr::Var(id))
    }
}
