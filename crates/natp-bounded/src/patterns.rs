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
            .take(cells.min(1))
            .collect();

        Self {
            constants: constants.into_iter().collect(),
            fresh,
        }
    }

    pub(super) fn candidate_values(&self, element: Element, used: usize) -> Vec<(Value, usize)> {
        match element {
            Element::Bool => vec![(Value::Bool(false), used), (Value::Bool(true), used)],
            Element::I32 => {
                let mut candidates: Vec<_> = self
                    .constants
                    .iter()
                    .map(|&value| (Value::I32(value), used))
                    .collect();
                for (index, &value) in self.fresh.iter().enumerate().take(used + 1) {
                    candidates.push((Value::I32(value), used.max(index + 1)));
                }
                candidates
            }
        }
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
