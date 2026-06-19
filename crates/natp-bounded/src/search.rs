//! Boolean scalar assignments within the case and work limits.
use crate::{Bounded, policy::Exhaustion, stop::SearchStop};
use natp_core::{Assignment, Domain, Element, ErrorKind, Theorem, Value};

fn spend_work(work: &mut usize, amount: usize) -> Result<(), SearchStop> {
    *work = work
        .checked_sub(amount)
        .ok_or_else(|| SearchStop::Exhausted(Exhaustion::Work))?;

    Ok(())
}

struct SearchBudget {
    max_cases: usize,
    completed_cases: usize,
    work: usize,
}

impl SearchBudget {
    fn new(limits: &Bounded) -> Self {
        Self {
            max_cases: limits.max_cases,
            completed_cases: 0,
            work: limits.max_work,
        }
    }

    fn completed_cases(&self) -> usize {
        self.completed_cases
    }

    fn remaining_work(&self) -> usize {
        self.work
    }

    fn fuel(&mut self) -> &mut usize {
        &mut self.work
    }

    fn spend(&mut self, amount: usize) -> Result<(), SearchStop> {
        spend_work(&mut self.work, amount)
    }

    fn ensure_case(&self) -> Result<(), SearchStop> {
        if self.completed_cases >= self.max_cases {
            return Err(SearchStop::Exhausted(Exhaustion::Cases));
        }

        Ok(())
    }

    fn record_case(&mut self) -> Result<(), SearchStop> {
        self.ensure_case()?;
        self.completed_cases += 1;

        Ok(())
    }
}

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
        if self.theorem.variables.as_slice() != [Domain::Scalar(Element::Bool)] {
            return Err(SearchStop::Exhausted(Exhaustion::Evaluation(
                "verification requires one Boolean scalar variable".into(),
            )));
        }
        self.budget.spend(3)?;
        for value in [false, true] {
            self.budget.spend(1)?;
            self.check_assignment(vec![Assignment::Scalar(Value::Bool(value))])?;
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
