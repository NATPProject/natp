/// Limits for native bounded verification.
#[derive(Clone, Copy, Debug)]
pub struct Bounded {
    /// Maximum assignments to examine.
    pub max_cases: usize,
    /// Maximum traversal and evaluation work units.
    pub max_work: usize,
}
