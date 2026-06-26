use std::{path::PathBuf, time::Duration};
/// External SMT solver configuration.
#[derive(Clone, Debug)]
pub struct Smt {
    /// Solver executable path.
    pub executable: PathBuf,
    /// Maximum time for one obligation.
    pub timeout: Duration,
}
