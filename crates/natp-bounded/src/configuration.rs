use crate::policy::ResourcePolicy;

/// Resource limits for native bounded verification.
#[derive(Clone, Copy, Debug)]
pub struct Bounded {
    /// Maximum number of length/equality-pattern cases to examine.
    pub max_cases: usize,
    /// Maximum number of traversal and evaluation work units.
    pub max_work: usize,
}

impl Default for Bounded {
    fn default() -> Self {
        Self {
            max_cases: ResourcePolicy::Cases.limit(),
            max_work: ResourcePolicy::Work.limit(),
        }
    }
}
