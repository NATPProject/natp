//! Boolean scalar assignments within the case and work limits.
use crate::{Bounded, budget::SearchBudget, policy::Exhaustion, stop::SearchStop};
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
    pub(super) fn completed_cases(&self) -> usize {
        self.budget.completed_cases()
    }
    pub(super) fn remaining_work(&self) -> usize {
        self.budget.remaining_work()
    }
    pub(super) fn verify(&mut self, _max_length: usize) -> Result<(), SearchStop> {
        if self
            .theorem
            .variables
            .iter()
            .any(|domain| *domain != Domain::Scalar(Element::Bool))
        {
            return Err(SearchStop::Exhausted(Exhaustion::Evaluation(
                "verification requires Boolean scalar variables".into(),
            )));
        }
        self.budget.spend(self.theorem.variables.len() + 1)?;
        self.visit_values(
            0,
            &mut vec![Value::Bool(false); self.theorem.variables.len()],
        )
    }
    fn visit_values(&mut self, cell: usize, values: &mut [Value]) -> Result<(), SearchStop> {
        self.budget.spend(1)?;
        if cell == values.len() {
            return self.check_assignment(values.iter().cloned().map(Assignment::Scalar).collect());
        }
        for value in [false, true] {
            values[cell] = Value::Bool(value);
            self.visit_values(cell + 1, values)?;
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
