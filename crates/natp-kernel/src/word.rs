//! Canonical sequence words and orientation-preserving normalisation.

use crate::trace::normalisation as trace;
use natp_core::{Expr, Trace, TraceRule};
use serde::{Deserialize, Serialize};

/// A sequence variable, possibly reversed, in a canonical concatenation word.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Letter {
    /// Zero-based sequence quantifier ID.
    pub variable: usize,
    /// Whether this occurrence denotes the variable's reversal.
    pub reversed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Orientation {
    Forward,
    Reversed,
}

impl Orientation {
    const fn reversed(self) -> bool {
        match self {
            Self::Forward => false,
            Self::Reversed => true,
        }
    }

    pub(super) const fn flipped(self) -> Self {
        match self {
            Self::Forward => Self::Reversed,
            Self::Reversed => Self::Forward,
        }
    }
}

pub(super) fn normalise_sequence(
    expression: &Expr,
    orientation: Orientation,
) -> (Vec<Letter>, Trace) {
    let (output, rule, premises) = match expression {
        Expr::Empty(_) => (vec![], TraceRule::SequenceEmpty, vec![]),
        Expr::Var(id) => (
            vec![Letter {
                variable: *id,
                reversed: orientation.reversed(),
            }],
            TraceRule::SequenceVariable,
            vec![],
        ),
        Expr::Reverse(x) => {
            let (output, predicate) = normalise_sequence(x, orientation.flipped());

            (output, TraceRule::SequenceReverse, vec![predicate])
        }
        Expr::Append(left, right) => {
            let (first, second) = match orientation {
                Orientation::Forward => (left, right),
                Orientation::Reversed => (right, left),
            };

            let (mut left, first_trace) = normalise_sequence(first, orientation);
            let (right, second_trace) = normalise_sequence(second, orientation);
            left.extend(right);

            (
                left,
                TraceRule::SequenceAppend,
                vec![first_trace, second_trace],
            )
        }
    };

    let normalisation_trace = trace(
        rule,
        (expression, orientation.reversed()),
        &output,
        premises,
    );

    (output, normalisation_trace)
}

pub(super) fn reverse_word(normalise_sequence: &[Letter]) -> Vec<Letter> {
    normalise_sequence
        .iter()
        .rev()
        .map(|letter| Letter {
            variable: letter.variable,
            reversed: !letter.reversed,
        })
        .collect()
}
