use natp_core::{Domain, Element, Value};
mod element;
mod natural;
mod sequence;

pub use element::{Elem, literal};
pub use natural::Nat;
pub use sequence::{Seq, append, contains, is_empty, length, reverse};

mod sealed {
    pub trait Element {}
    impl Element for i32 {}

    impl Element for bool {}
    pub trait Domain {}
}

/// An element sort whose values can be represented exactly in the logic.
pub trait SymbolicElement: sealed::Element {
    /// The IR sort corresponding to this Rust type.
    const ELEMENT: Element;

    /// Convert a literal to an IR value.
    fn value(self) -> Value;
}

impl SymbolicElement for i32 {
    const ELEMENT: Element = Element::I32;

    fn value(self) -> Value {
        Value::I32(self)
    }
}

impl SymbolicElement for bool {
    const ELEMENT: Element = Element::Bool;

    fn value(self) -> Value {
        Value::Bool(self)
    }
}

/// A sealed domain accepted by the theorem builder's universal quantifiers.
pub trait SymbolicDomain: sealed::Domain {
    /// The quantifier domain represented by this symbolic type.
    const DOMAIN: Domain;
    #[doc(hidden)]
    fn variable(id: usize) -> Self;
}
