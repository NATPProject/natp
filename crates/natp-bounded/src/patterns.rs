//! Representatives of the equality classes left unnamed by a proposition.

use natp_core::{ElemExpr, Element, Prop, Value};
use std::collections::BTreeSet;

/// Literal values are fixed; fresh values stand for distinct unnamed classes.
pub(super) struct EqualityPatterns {
    constants: Vec<i32>,
    fresh: Vec<i32>,
}

impl EqualityPatterns {
    pub(super) fn new(claim: &Prop, cells: usize) -> Self {
        let mut constants = BTreeSet::new();
        collect_constants(claim, &mut constants);

        // Syntax limits cap the excluded literals below the number of i32 values,
        // leaving enough representatives for every permitted input cell.
        let fresh = (i32::MIN..=i32::MAX)
            .filter(|value| !constants.contains(value))
            .take(cells)
            .collect();

        Self {
            constants: constants.into_iter().collect(),
            fresh,
        }
    }

    /// Restricted-growth enumeration: reuse any known class or introduce one.
    pub(super) fn candidate_values(
        &self,
        element: Element,
        used: usize,
    ) -> impl Iterator<Item = (Value, usize)> + '_ {
        // Search consumes one candidate at a time and may stop on the first
        // refutation. Avoid allocating a candidate list at every tree node.
        let mut index = 0;

        std::iter::from_fn(move || {
            let candidate = match element {
                Element::Bool if index < 2 => Some((Value::Bool(index == 1), used)),
                Element::Bool => None,
                Element::I32 if index < self.constants.len() => {
                    Some((Value::I32(self.constants[index]), used))
                }
                Element::I32 => {
                    let fresh_index = index - self.constants.len();

                    self.fresh
                        .get(fresh_index)
                        .filter(|_| fresh_index <= used)
                        .map(|&value| (Value::I32(value), used.max(fresh_index + 1)))
                }
            };

            index += usize::from(candidate.is_some());

            candidate
        })
    }
}

fn collect_constants(proposition: &Prop, constants: &mut BTreeSet<i32>) {
    match proposition {
        Prop::ElemEq(left, right) => {
            collect_element(left, constants);
            collect_element(right, constants);
        }
        Prop::Contains(_, element) => collect_element(element, constants),
        Prop::All(_, predicate) | Prop::Not(predicate) => collect_constants(predicate, constants),
        Prop::And(left, right) | Prop::Or(left, right) | Prop::Implies(left, right) => {
            collect_constants(left, constants);
            collect_constants(right, constants);
        }
        _ => {}
    }
}

fn collect_element(element: &ElemExpr, constants: &mut BTreeSet<i32>) {
    if let ElemExpr::Constant(Value::I32(value)) = element {
        constants.insert(*value);
    }
}
