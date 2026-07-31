//! Portable rule applications and deterministic tree rendering.

use std::fmt::Write;

use serde::{Deserialize, Serialize};

/// A rule application and its premises.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Trace {
    /// Stable rule identifier from the kernel rule catalogue.
    pub rule: String,
    /// Description of the rule input; normalisation traces use Debug formatting.
    pub before: String,
    /// Description of the rule result; normalisation traces use Debug formatting.
    pub after: String,
    /// Rule applications supporting this step, in deterministic order.
    pub premises: Vec<Trace>,
}

impl Trace {
    /// Render the trace as an indented tree.
    pub fn render(&self) -> String {
        fn render_trace(trace: &Trace, depth: usize, output: &mut String) {
            let _ = writeln!(
                output,
                "{}{}: {} -> {}",
                "  ".repeat(depth),
                trace.rule,
                trace.before,
                trace.after
            );

            for premise in &trace.premises {
                render_trace(premise, depth + 1, output);
            }
        }

        let mut output = String::new();
        render_trace(self, 0, &mut output);

        output
    }
}
