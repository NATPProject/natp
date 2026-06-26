use crate::vocabulary::{RuntimePolicy, SolverSetting};
use std::{path::PathBuf, time::Duration};

/// Solver executable and per-obligation timeout.
#[derive(Clone, Debug)]
pub struct Smt {
    /// Z3, or a solver accepting the same `-smt2 <path>` invocation.
    pub executable: PathBuf,
    /// Maximum wall-clock time for each obligation independently.
    pub timeout: Duration,
}

impl Default for Smt {
    fn default() -> Self {
        Self {
            executable: SolverSetting::Executable.text().into(),
            timeout: RuntimePolicy::timeout(),
        }
    }
}
