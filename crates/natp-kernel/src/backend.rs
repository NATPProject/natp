use crate::{boolean, certify, check};
use natp_core::{Backend, Error, Method, Outcome, Report, Theorem};

/// Native backend: trusted normalisation followed by independent certificate checking.
#[derive(Clone, Copy, Debug, Default)]
pub struct Kernel;
impl Backend for Kernel {
    fn prove(&self, theorem: &Theorem) -> Result<Report, Error> {
        if let Some(certificate) = certify(theorem)? {
            if !check(theorem, &certificate)? {
                return Err(Error::with_kind(
                    natp_core::ErrorKind::Internal,
                    "internal certificate check failed",
                ));
            }

            Ok(Report {
                theorem: theorem.clone(),
                method: Method::Kernel,
                outcome: Outcome::Proven,
                evidence: "rechecked normalisation and propositional deduction certificate".into(),
                trace: None,
            })
        } else {
            Ok(Report {
                theorem: theorem.clone(),
                method: Method::Kernel,
                outcome: Outcome::Inconclusive {
                    reason: format!(
                        "no derivation in the kernel fragment (or more than {} independent atoms)",
                        boolean::MAX_ATOMS
                    ),
                },
                evidence: "no certificate; no bounded result was promoted".into(),
                trace: None,
            })
        }
    }
}
