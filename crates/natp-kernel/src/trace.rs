//! Stable trace adapters for canonical normalisation and Boolean deductions.

use natp_core::{Trace, TraceRule};

pub(super) fn normalisation(
    rule: TraceRule,
    before: impl std::fmt::Debug,
    after: impl std::fmt::Debug,
    premises: Vec<Trace>,
) -> Trace {
    Trace {
        rule: rule.as_str().into(),
        before: format!("{before:?}"),
        after: format!("{after:?}"),
        premises,
    }
}
