//! Validate bounded claims and produce verification reports.

use natp_core::{Backend, Error, Method, Outcome, Report, Scope, Theorem};

use crate::{Bounded, search::BoundedSearch, stop::SearchStop};

impl Backend for Bounded {
    fn prove(&self, theorem: &Theorem) -> Result<Report, Error> {
        theorem.validate()?;
        let Scope::Bounded { max_len } = theorem.scope else {
            return Err(Error::new(
                "native bounded verification requires finite scope",
            ));
        };

        let mut search = BoundedSearch::new(theorem, self);
        let outcome = match search.verify(max_len) {
            Ok(()) => Outcome::Verified {
                max_length: max_len,
            },
            Err(SearchStop::Refuted(inputs)) => Outcome::Refuted { inputs },
            Err(SearchStop::Exhausted(reason)) => Outcome::Inconclusive {
                reason: reason.reason(),
            },
            Err(SearchStop::Failed(error)) => return Err(error),
        };

        let evidence = format!(
            "{} symbolic length/equality-pattern cases examined; {} work units used; \
             bounds apply independently to every sequence",
            search.completed_cases(),
            self.max_work - search.remaining_work(),
        );

        Ok(Report {
            theorem: theorem.clone(),
            method: Method::NativeBounded,
            outcome,
            evidence,
            trace: None,
        })
    }
}
