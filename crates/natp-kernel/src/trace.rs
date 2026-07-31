//! Stable trace adapters for canonical normalisation and Boolean deductions.

use std::fmt::Debug;

use natp_core::{Trace, TraceRule};

use crate::{BooleanProof, boolean};

pub(super) fn normalisation(
    rule: TraceRule,
    before: impl Debug,
    after: impl Debug,
    premises: Vec<Trace>,
) -> Trace {
    Trace {
        rule: rule.as_str().into(),
        before: format!("{before:?}"),
        after: format!("{after:?}"),
        premises,
    }
}

pub(super) fn deduction(proof: &BooleanProof) -> Trace {
    match proof {
        BooleanProof::True => Trace {
            rule: TraceRule::BooleanEvaluation.as_str().into(),
            before: "partial valuation".into(),
            after: boolean::BooleanStatus::True.as_str().into(),
            premises: vec![],
        },
        BooleanProof::Cases {
            atom,
            on_false,
            on_true,
        } => Trace {
            rule: TraceRule::ClassicalCases.as_str().into(),
            before: format!("atom {atom}: false / true"),
            after: "both branches true".into(),
            premises: vec![deduction(on_false), deduction(on_true)],
        },
    }
}
