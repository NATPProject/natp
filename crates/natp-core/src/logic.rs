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
