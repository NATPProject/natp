//! One quantified variable's domain and contiguous input-cell slice.

use natp_core::{Assignment, Domain, Value};
use std::ops::Range;

pub(super) struct InputVariable {
    domain: Domain,
    cells: Range<usize>,
}

impl InputVariable {
    pub(super) fn new(domain: Domain, cells: Range<usize>) -> Self {
        Self { domain, cells }
    }

    pub(super) fn positions(&self) -> Range<usize> {
        self.cells.clone()
    }

    pub(super) fn bind(&self, values: &[Value]) -> Assignment {
        match self.domain {
            Domain::Scalar(_) => Assignment::Scalar(values[self.cells.start].clone()),
            Domain::Sequence(_) => Assignment::Sequence(values[self.cells.clone()].to_vec()),
        }
    }
}
