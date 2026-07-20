use super::{Elem, Nat, SymbolicDomain, SymbolicElement, sealed};
use natp_core::{Domain, Expr, NatExpr, Prop};
use std::marker::PhantomData;

/// A symbolic finite list parameterised by element sort `T`.
/// Its operations build expressions; values are supplied during evaluation.
#[derive(Debug)]
pub struct Seq<T> {
    expression: Expr,
    marker: PhantomData<T>,
}

impl<T> Clone for Seq<T> {
    fn clone(&self) -> Self {
        Self {
            expression: self.expression.clone(),
            marker: PhantomData,
        }
    }
}

impl<T: SymbolicElement> Seq<T> {
    fn new(expression: Expr) -> Self {
        Self {
            expression,
            marker: PhantomData,
        }
    }

    /// Construct an empty sequence with this element sort.
    pub fn empty() -> Self {
        Self::new(Expr::Empty(T::ELEMENT))
    }

    /// Build a symbolic reversal; no concrete values are evaluated.
    pub fn reversed(self) -> Self {
        Self::new(Expr::Reverse(Box::new(self.expression)))
    }

    /// Concatenate two symbolic sequences of the same sort.
    pub fn append(self, rhs: Self) -> Self {
        Self::new(Expr::Append(
            Box::new(self.expression),
            Box::new(rhs.expression),
        ))
    }

    /// Construct extensional sequence equality.
    pub fn equals(self, rhs: Self) -> Prop {
        Prop::SeqEq(self.expression, rhs.expression)
    }

    /// Construct a sequence-length expression.
    pub fn length(self) -> Nat {
        Nat::from_expression(NatExpr::Length(self.expression))
    }

    /// Construct the proposition that this sequence has length zero.
    pub fn is_empty(self) -> Prop {
        Prop::IsEmpty(self.expression)
    }

    /// Construct membership using only element equality.
    pub fn contains(self, value: impl Into<Elem<T>>) -> Prop {
        Prop::Contains(self.expression, value.into().into_expression())
    }
}

/// Function form of [`Seq::reversed`].
pub fn reverse<T: SymbolicElement>(sequence: Seq<T>) -> Seq<T> {
    sequence.reversed()
}

/// Function form of [`Seq::append`].
pub fn append<T: SymbolicElement>(sequence: Seq<T>, other: Seq<T>) -> Seq<T> {
    sequence.append(other)
}

/// Function form of [`Seq::length`].
pub fn length<T: SymbolicElement>(sequence: Seq<T>) -> Nat {
    sequence.length()
}

/// Function form of [`Seq::is_empty`].
pub fn is_empty<T: SymbolicElement>(sequence: Seq<T>) -> Prop {
    sequence.is_empty()
}

impl<T: SymbolicElement> sealed::Domain for Seq<T> {}

impl<T: SymbolicElement> SymbolicDomain for Seq<T> {
    const DOMAIN: Domain = Domain::Sequence(T::ELEMENT);
    fn variable(id: usize) -> Self {
        Self::new(Expr::Var(id))
    }
}
