//! Case accounting and fuel ownership shared by traversal and symbolic evaluation.

use crate::{Bounded, policy::Exhaustion, stop::SearchStop};

pub(super) fn spend_work(work: &mut usize, amount: usize) -> Result<(), SearchStop> {
    *work = work
        .checked_sub(amount)
        .ok_or_else(|| SearchStop::Exhausted(Exhaustion::Work))?;

    Ok(())
}

pub(super) struct SearchBudget {
    max_cases: usize,
    completed_cases: usize,
    work: usize,
}

impl SearchBudget {
    pub(super) fn new(limits: &Bounded) -> Self {
        Self {
            max_cases: limits.max_cases,
            completed_cases: 0,
            work: limits.max_work,
        }
    }

    pub(super) fn completed_cases(&self) -> usize {
        self.completed_cases
    }

    pub(super) fn remaining_work(&self) -> usize {
        self.work
    }

    pub(super) fn fuel(&mut self) -> &mut usize {
        &mut self.work
    }

    pub(super) fn spend(&mut self, amount: usize) -> Result<(), SearchStop> {
        spend_work(&mut self.work, amount)
    }

    pub(super) fn ensure_case(&self) -> Result<(), SearchStop> {
        if self.completed_cases >= self.max_cases {
            return Err(SearchStop::Exhausted(Exhaustion::Cases));
        }

        Ok(())
    }

    pub(super) fn record_case(&mut self) -> Result<(), SearchStop> {
        self.ensure_case()?;
        self.completed_cases += 1;

        Ok(())
    }
}
