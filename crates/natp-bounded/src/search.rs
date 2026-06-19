//! Enumerate sequence lengths and equality patterns within the work and case budgets.

use crate::policy::{Exhaustion, ResourcePolicy};
use crate::{Bounded, budget::SearchBudget, inputs::InputLayout, stop::SearchStop};
use natp_core::{Assignment, Domain, Element, ErrorKind, Theorem, Value};

pub(super) struct BoundedSearch<'a> {
    theorem: &'a Theorem,
    budget: SearchBudget,
}

impl<'a> BoundedSearch<'a> {
    pub(super) fn new(theorem: &'a Theorem, limits: &Bounded) -> Self {
        Self {
            theorem,
            budget: SearchBudget::new(limits),
        }
    }

    pub(super) fn verify(&mut self, max_length: usize) -> Result<(), SearchStop> {
        if self
            .theorem
            .variables
            .iter()
            .any(|domain| domain.element() != Element::Bool)
        {
            return Err(SearchStop::Exhausted(Exhaustion::Evaluation(
                "integer equality patterns are not supported".into(),
            )));
        }
        self.visit_lengths(max_length, &mut Vec::new())
    }

    pub(super) fn completed_cases(&self) -> usize {
        self.budget.completed_cases()
    }

    pub(super) fn remaining_work(&self) -> usize {
        self.budget.remaining_work()
    }

    fn visit_lengths(
        &mut self,
        max_length: usize,
        lengths: &mut Vec<usize>,
    ) -> Result<(), SearchStop> {
        self.budget.spend(1)?;
        let Some(domain) = self.theorem.variables.get(lengths.len()) else {
            return self.check_lengths(lengths);
        };

        let range = match domain {
            Domain::Scalar(_) => 1..=1,
            Domain::Sequence(_) => 0..=max_length,
        };

        for length in range {
            lengths.push(length);
            let result = self.visit_lengths(max_length, lengths);
            lengths.pop();
            result?;
        }

        Ok(())
    }

    fn check_lengths(&mut self, lengths: &[usize]) -> Result<(), SearchStop> {
        let inputs = InputLayout::new(&self.theorem.variables, lengths);

        if inputs.len() > ResourcePolicy::PatternCells.limit() {
            return Err(SearchStop::Exhausted(Exhaustion::PatternCells));
        }

        self.visit_patterns(&inputs, 0, &mut inputs.zeroes())
    }

    fn visit_patterns(
        &mut self,
        inputs: &InputLayout,
        cell: usize,
        values: &mut [Value],
    ) -> Result<(), SearchStop> {
        self.budget.spend(1)?;

        if cell == inputs.len() {
            return self.check_assignment(inputs.bind(values));
        }

        for value in [false, true] {
            values[cell] = Value::Bool(value);
            self.visit_patterns(inputs, cell + 1, values)?;
        }

        Ok(())
    }

    fn check_assignment(&mut self, inputs: Vec<Assignment>) -> Result<(), SearchStop> {
        self.budget.record_case()?;

        match self.theorem.evaluate(&inputs, self.budget.fuel()) {
            Ok(true) => Ok(()),
            Ok(false) => Err(SearchStop::Refuted(inputs)),
            Err(error) if error.kind() == ErrorKind::ResourceLimit => Err(SearchStop::Exhausted(
                Exhaustion::Evaluation(error.to_string()),
            )),
            Err(error) => Err(SearchStop::Failed(error)),
        }
    }
}
