//! Typed theorem expressions and concrete values.

use serde::{Deserialize, Serialize};

/// An equality-only element sort supported by every backend.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Element {
    /// All signed 32-bit values, with no arithmetic or ordering operations.
    I32,
    /// The two Boolean values.
    Bool,
}

/// A concrete element used for literals and counterexample replay.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(
    tag = "type",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum Value {
    /// A signed integer literal.
    I32(i32),
    /// A Boolean literal.
    Bool(bool),
}

impl Value {
    /// Return this value's logical sort.
    pub fn element(&self) -> Element {
        match self {
            Self::I32(_) => Element::I32,
            Self::Bool(_) => Element::Bool,
        }
    }
}

/// The domain of a single universally quantified variable.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    content = "element",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum Domain {
    /// All finite sequences over the given element sort.
    Sequence(Element),
    /// A single element of the given sort.
    Scalar(Element),
}

/// IDs index the theorem's ordered quantifier list; no implicit free variables.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(
    tag = "op",
    content = "args",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum Expr {
    /// An empty sequence of an explicit element sort.
    Empty(Element),
    /// A sequence variable indexed in the theorem's ordered quantifiers.
    Var(usize),
    /// Concatenation in left-to-right order.
    Append(Box<Expr>, Box<Expr>),
    /// Reversal of a finite sequence.
    Reverse(Box<Expr>),
}

/// A scalar term; bound elements are meaningful only inside one `all` predicate.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(
    tag = "op",
    content = "args",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum ElemExpr {
    /// A scalar variable indexed in the theorem's ordered quantifiers.
    Var(usize),
    /// A fixed literal, shared across all assignments.
    Constant(Value),
    /// The element bound by the enclosing all. Nested all is rejected.
    Bound,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(
    tag = "op",
    content = "args",
    rename_all = "snake_case",
    deny_unknown_fields
)]
/// A natural-number expression evaluated without wrapping arithmetic.
pub enum NatExpr {
    /// The mathematical length of a sequence.
    Length(Expr),
    /// A nonnegative integer literal.
    Constant(u64),
    /// Mathematical addition, without machine-integer wrapping.
    Add(Box<NatExpr>, Box<NatExpr>),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(
    tag = "op",
    content = "args",
    rename_all = "snake_case",
    deny_unknown_fields
)]
/// A proposition in the restricted, typed finite-sequence language.
#[must_use = "a symbolic proposition must be attached to a theorem"]
pub enum Prop {
    /// A constant proposition.
    Bool(bool),
    /// Extensional equality of two sequences with the same sort.
    SeqEq(Expr, Expr),
    /// Equality of same-sort elements.
    ElemEq(ElemExpr, ElemExpr),
    /// Equality of natural-number expressions.
    NatEq(NatExpr, NatExpr),
    /// Whether a sequence has zero length.
    IsEmpty(Expr),
    /// Whether an element occurs in a sequence.
    Contains(Expr, ElemExpr),
    /// Universal membership predicate; true on an empty sequence.
    All(Expr, Box<Prop>),
    /// Classical negation.
    Not(Box<Prop>),
    /// Conjunction.
    And(Box<Prop>, Box<Prop>),
    /// Disjunction.
    Or(Box<Prop>, Box<Prop>),
    /// Material implication.
    Implies(Box<Prop>, Box<Prop>),
}
