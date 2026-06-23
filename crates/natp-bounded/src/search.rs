//! Enumerate sequence lengths and equality patterns within the work and case budgets.

use crate::policy::{Exhaustion, ResourcePolicy};
use crate::{
    Bounded, budget::SearchBudget, inputs::InputLayout, patterns::EqualityPatterns,
    stop::SearchStop, symbolic,
};
use natp_core::{Assignment, Domain, Error, ErrorKind, Expr, Prop, Theorem, Value};

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
        let patterns =
            EqualityPatterns::new(&self.theorem.claim, ResourcePolicy::PatternCells.limit());

        self.visit_lengths(max_length, &patterns, &mut Vec::new())
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
        patterns: &EqualityPatterns,
        lengths: &mut Vec<usize>,
    ) -> Result<(), SearchStop> {
        self.budget.spend(1)?;
        let Some(domain) = self.theorem.variables.get(lengths.len()) else {
            return self.check_lengths(lengths, patterns);
        };

        let range = match domain {
            Domain::Scalar(_) => 1..=1,
            Domain::Sequence(_) => 0..=max_length,
        };

        for length in range {
            lengths.push(length);
            let result = self.visit_lengths(max_length, patterns, lengths);
            lengths.pop();
            result?;
        }

        Ok(())
    }

    fn check_lengths(
        &mut self,
        lengths: &[usize],
        patterns: &EqualityPatterns,
    ) -> Result<(), SearchStop> {
        let inputs = InputLayout::new(&self.theorem.variables, lengths);

        if let Prop::SeqEq(left, right) = &self.theorem.claim {
            if symbolic::supports(left) && symbolic::supports(right) {
                return self.check_equality(&inputs, left, right);
            }
        }

        if symbolic::is_value_independent(&self.theorem.claim) {
            return self.check_assignment(inputs.bind(&inputs.zeroes()));
        }

        if inputs.len() > ResourcePolicy::PatternCells.limit() {
            return Err(SearchStop::Exhausted(Exhaustion::PatternCells));
        }

        self.visit_patterns(&inputs, patterns, 0, 0, &mut inputs.zeroes())
    }

    fn check_equality(
        &mut self,
        inputs: &InputLayout,
        left: &Expr,
        right: &Expr,
    ) -> Result<(), SearchStop> {
        self.budget.ensure_case()?;
        let left = symbolic::positions(left, inputs, self.budget.fuel())?;
        let right = symbolic::positions(right, inputs, self.budget.fuel())?;

        if left == right {
            self.budget.record_case()?;

            return Ok(());
        }

        // A length mismatch needs no special values. Otherwise two distinct
        // values separate the input positions at the first differing output.
        let mut values = inputs.zeroes();

        if let Some((&different, _)) = left.iter().zip(&right).find(|(a, b)| a != b) {
            values[different] = match values[different] {
                Value::I32(_) => Value::I32(1),
                Value::Bool(_) => Value::Bool(true),
            };
        }

        match self.check_assignment(inputs.bind(&values)) {
            Ok(()) => Err(SearchStop::Failed(Error::with_kind(
                ErrorKind::Internal,
                "internal symbolic counterexample replay failed",
            ))),
            failure => failure,
        }
    }

    fn visit_patterns(
        &mut self,
        inputs: &InputLayout,
        patterns: &EqualityPatterns,
        cell: usize,
        used_classes: usize,
        values: &mut [Value],
    ) -> Result<(), SearchStop> {
        self.budget.spend(1)?;

        if cell == inputs.len() {
            return self.check_assignment(inputs.bind(values));
        }

        for (value, next_used) in patterns.candidate_values(inputs.element(cell), used_classes) {
            values[cell] = value;
            self.visit_patterns(inputs, patterns, cell + 1, next_used, values)?;
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
