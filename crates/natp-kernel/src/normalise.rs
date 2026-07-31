//! Canonical propositions derived from validated theorem expressions.

use std::{cmp, mem};

use natp_core::{ElemExpr, Prop, Trace, TraceRule};
use serde::{Deserialize, Serialize};

use crate::linear::normalise_length;
use crate::trace::normalisation as trace;
use crate::word::{Orientation, normalise_sequence, reverse_word};
use crate::{Letter, Linear};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
/// An opaque proposition after trusted finite-list normalisation.
pub enum Atom {
    /// Equality of canonical words, modulo equality symmetry and reversal.
    SequenceEqual(Vec<Letter>, Vec<Letter>),
    /// Equality of reduced natural linear expressions.
    LengthEqual(Linear, Linear),
    /// Emptiness of a sequence variable.
    Empty(usize),
    /// Equality of canonical scalar terms.
    ElementEqual(ElemExpr, ElemExpr),
    /// Membership in a sequence variable.
    Contains(usize, ElemExpr),
    /// A predicate holding for every element of a sequence variable.
    All(usize, Box<Formula>),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
/// Canonical propositional syntax serialised in kernel certificates.
pub enum Formula {
    /// Constant truth value.
    Bool(bool),
    /// An opaque, independently assigned atom.
    Atom(Atom),
    /// Negation.
    Not(Box<Formula>),
    /// Sorted, deduplicated conjunction.
    And(Vec<Formula>),
    /// Sorted, deduplicated disjunction.
    Or(Vec<Formula>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Connective {
    Conjunction,
    Disjunction,
}

impl Connective {
    const fn identity(self) -> bool {
        match self {
            Self::Conjunction => true,
            Self::Disjunction => false,
        }
    }

    fn formula(self, items: Vec<Formula>) -> Formula {
        match self {
            Self::Conjunction => Formula::And(items),
            Self::Disjunction => Formula::Or(items),
        }
    }
}

fn negate(formula: Formula) -> Formula {
    match formula {
        Formula::Bool(right) => Formula::Bool(!right),
        Formula::Not(x) => *x,
        x => Formula::Not(Box::new(x)),
    }
}

fn combine_formulae(items: Vec<Formula>, connective: Connective) -> Formula {
    let mut output = Vec::new();

    for item in items {
        match item {
            Formula::Bool(right) if right != connective.identity() => return Formula::Bool(right),
            Formula::Bool(_) => {}
            Formula::And(xs) if connective == Connective::Conjunction => output.extend(xs),
            Formula::Or(xs) if connective == Connective::Disjunction => output.extend(xs),
            x => output.push(x),
        }
    }

    output.sort();
    output.dedup();

    match output.as_slice() {
        [] => Formula::Bool(connective.identity()),
        [single] => single.clone(),
        _ => connective.formula(output),
    }
}

fn word_is_empty(normalise_sequence: &[Letter]) -> Formula {
    combine_formulae(
        normalise_sequence
            .iter()
            .map(|letter| Formula::Atom(Atom::Empty(letter.variable)))
            .collect(),
        Connective::Conjunction,
    )
}

fn compare_lengths(mut left: Linear, mut right: Linear) -> Formula {
    // Cancel only shared nonnegative contributions. A sum of natural lengths
    // is zero exactly when every positively weighted input sequence is empty.
    let shared = left.constant.min(right.constant);
    left.constant -= shared;
    right.constant -= shared;

    for (id, value) in &mut left.lengths {
        if let Some(right_coefficient) = right.lengths.get_mut(id) {
            let shared_coefficient = (*value).min(*right_coefficient);
            *value -= shared_coefficient;
            *right_coefficient -= shared_coefficient;
        }
    }

    left.lengths.retain(|_, value| *value != 0);
    right.lengths.retain(|_, value| *value != 0);

    if left == right {
        return Formula::Bool(true);
    }

    if left.lengths.is_empty() && right.lengths.is_empty() {
        return Formula::Bool(false);
    }

    if left == Linear::default() || right == Linear::default() {
        let positive = if left == Linear::default() {
            right
        } else {
            left
        };

        if positive.constant > 0 {
            return Formula::Bool(false);
        }

        return combine_formulae(
            positive
                .lengths
                .keys()
                .map(|id| Formula::Atom(Atom::Empty(*id)))
                .collect(),
            Connective::Conjunction,
        );
    }

    if right < left {
        mem::swap(&mut left, &mut right);
    }

    Formula::Atom(Atom::LengthEqual(left, right))
}

/// Caller must validate the typed IR and resource limits first.
pub(super) fn normalise(prop: &Prop) -> (Formula, Trace) {
    let mut premises = Vec::new();
    let (formula, rule) = match prop {
        Prop::Bool(right) => (Formula::Bool(*right), TraceRule::BooleanConstant),
        Prop::SeqEq(left, right) => {
            let (left_word, left_trace) = normalise_sequence(left, Orientation::Forward);
            let (right_word, right_trace) = normalise_sequence(right, Orientation::Forward);

            premises.extend([left_trace, right_trace]);
            let formula = if left_word == right_word {
                Formula::Bool(true)
            } else if left_word.is_empty() {
                word_is_empty(&right_word)
            } else if right_word.is_empty() {
                word_is_empty(&left_word)
            } else {
                let first = (left_word.clone(), right_word.clone());
                let pairs = [
                    (right_word.clone(), left_word.clone()),
                    (reverse_word(&left_word), reverse_word(&right_word)),
                    (reverse_word(&right_word), reverse_word(&left_word)),
                ];
                let (canonical_left, canonical_right) = pairs.into_iter().fold(first, cmp::min);
                let equality = Formula::Atom(Atom::SequenceEqual(canonical_left, canonical_right));
                // Equality entails equal lengths; conjoining this consequence is
                // equivalence preserving and enables elementary congruence proofs.
                combine_formulae(
                    vec![
                        equality,
                        compare_lengths(
                            Linear::from_word(&left_word),
                            Linear::from_word(&right_word),
                        ),
                    ],
                    Connective::Conjunction,
                )
            };

            (formula, TraceRule::SequenceEquality)
        }
        Prop::NatEq(left, right) => (
            compare_lengths(normalise_length(left), normalise_length(right)),
            TraceRule::NaturalEquality,
        ),
        Prop::ElemEq(left, right) => {
            let formula = if left == right {
                Formula::Bool(true)
            } else if let (ElemExpr::Constant(left), ElemExpr::Constant(right)) = (left, right) {
                Formula::Bool(left == right)
            } else {
                let (left, right) = if left < right {
                    (left, right)
                } else {
                    (right, left)
                };

                Formula::Atom(Atom::ElementEqual(left.clone(), right.clone()))
            };

            (formula, TraceRule::ElementEquality)
        }
        Prop::IsEmpty(x) => {
            let (sequence_word, normalisation_trace) = normalise_sequence(x, Orientation::Forward);
            premises.push(normalisation_trace);

            (word_is_empty(&sequence_word), TraceRule::SequenceEmptiness)
        }
        Prop::Contains(x, e) => {
            let (sequence_word, normalisation_trace) = normalise_sequence(x, Orientation::Forward);
            premises.push(normalisation_trace);

            (
                combine_formulae(
                    sequence_word
                        .iter()
                        .map(|letter| Formula::Atom(Atom::Contains(letter.variable, e.clone())))
                        .collect(),
                    Connective::Disjunction,
                ),
                TraceRule::Membership,
            )
        }
        Prop::All(x, predicate) => {
            let (sequence_word, normalisation_trace) = normalise_sequence(x, Orientation::Forward);
            let (predicate, predicate_trace) = normalise(predicate);

            premises.extend([normalisation_trace, predicate_trace]);
            let formula = match predicate {
                Formula::Bool(true) => Formula::Bool(true),
                Formula::Bool(false) => word_is_empty(&sequence_word),
                predicate => combine_formulae(
                    sequence_word
                        .iter()
                        .map(|letter| {
                            Formula::Atom(Atom::All(letter.variable, Box::new(predicate.clone())))
                        })
                        .collect(),
                    Connective::Conjunction,
                ),
            };

            (formula, TraceRule::UniversalMembership)
        }
        Prop::Not(predicate) => {
            let (predicate, normalisation_trace) = normalise(predicate);
            premises.push(normalisation_trace);

            (negate(predicate), TraceRule::Negation)
        }
        Prop::And(left, right) | Prop::Or(left, right) | Prop::Implies(left, right) => {
            let (left, left_trace) = normalise(left);
            let (right, right_trace) = normalise(right);

            premises.extend([left_trace, right_trace]);
            let formula = match prop {
                Prop::And(..) => combine_formulae(vec![left, right], Connective::Conjunction),
                Prop::Or(..) => combine_formulae(vec![left, right], Connective::Disjunction),
                _ => combine_formulae(vec![negate(left), right], Connective::Disjunction),
            };

            (formula, TraceRule::Connectives)
        }
    };

    let normalisation_trace = trace(rule, prop, &formula, premises);

    (formula, normalisation_trace)
}
