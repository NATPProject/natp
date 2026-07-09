//! Run SMT obligations and report solver evidence.

use crate::{Smt, export::write_obligation, induction, protocol, vocabulary::SolverSetting};
use natp_core::{Backend, Error, Method, Outcome, Report, Theorem, process};
use std::process::Command;

impl Backend for Smt {
    fn prove(&self, theorem: &Theorem) -> Result<Report, Error> {
        let goals = induction(theorem)?;
        let temp = tempfile::tempdir()?;
        let mut evidence = String::new();

        for goal in goals.obligations {
            let path = write_obligation(&goal, temp.path())?;
            let output = process::run(
                Command::new(&self.executable)
                    .arg(SolverSetting::SmtInputFlag.text())
                    .arg(path),
                self.timeout,
            )?;
            evidence.push_str(&output.stdout);

            if !protocol::is_unsat(&output) {
                return Ok(Report {
                    theorem: theorem.clone(),
                    method: Method::SmtInduction,
                    outcome: Outcome::Inconclusive {
                        reason: format!(
                            "{} obligation did not return a successful unsat answer",
                            goal.name
                        ),
                    },
                    evidence,
                    trace: None,
                });
            }
        }

        Ok(Report {
            theorem: theorem.clone(),
            method: Method::SmtInduction,
            outcome: Outcome::SmtValidated,
            evidence: format!("external solver trusted; not a kernel-checked proof\n{evidence}"),
            trace: None,
        })
    }
}
