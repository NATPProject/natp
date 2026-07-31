//! Sequence-length coefficient accumulation for validated natural expressions.

use std::collections::BTreeMap;

use natp_core::{Expr, NatExpr};
use serde::{Deserialize, Serialize};

use crate::Letter;

/// A natural constant plus nonnegative multiples of input sequence lengths.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Linear {
    /// Constant term; validated syntax bounds prevent overflow.
    pub constant: u128,
    /// Nonzero coefficients keyed by sequence quantifier ID.
    pub lengths: BTreeMap<usize, u64>,
}

impl Linear {
    pub(super) fn from_word(word: &[Letter]) -> Self {
        let mut result = Self::default();

        for letter in word {
            *result.lengths.entry(letter.variable).or_default() += 1;
        }

        result
    }

    fn add_sequence_length(&mut self, expression: &Expr) {
        // Length ignores orientation and concatenation order. Accumulate directly
        // instead of allocating a canonical word and a discarded sequence trace.
        match expression {
            Expr::Empty(_) => {}
            Expr::Var(id) => *self.lengths.entry(*id).or_default() += 1,
            Expr::Reverse(sequence) => self.add_sequence_length(sequence),
            Expr::Append(left, right) => {
                self.add_sequence_length(left);
                self.add_sequence_length(right);
            }
        }
    }

    fn add_natural(&mut self, expression: &NatExpr) {
        // Validation bounds the entire tree by the shared syntax-node limit. Each literal is u64,
        // so their sum fits u128; every coefficient is <= the syntax-node count.
        match expression {
            NatExpr::Constant(value) => self.constant += u128::from(*value),
            NatExpr::Length(sequence) => self.add_sequence_length(sequence),
            NatExpr::Add(left, right) => {
                self.add_natural(left);
                self.add_natural(right);
            }
        }
    }
}

pub(super) fn normalise_length(expression: &NatExpr) -> Linear {
    let mut result = Linear::default();
    result.add_natural(expression);

    result
}
