use std::path::PathBuf;
/// External SMT solver configuration.
#[derive(Clone, Debug)]
pub struct Smt {
    /// Solver executable path.
    pub executable: PathBuf,
}
