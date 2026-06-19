//! Bounded backend resource defaults and exhaustion categories.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ResourcePolicy {
    Cases,
    Work,
}
impl ResourcePolicy {
    pub(super) const fn limit(self) -> usize {
        match self {
            Self::Cases => 100_000,
            Self::Work => 10_000_000,
        }
    }
}

pub(super) enum Exhaustion {
    Work,
    Cases,
    Evaluation(String),
}
impl Exhaustion {
    pub(super) fn reason(self) -> String {
        match self {
            Self::Work => "work budget exhausted".into(),
            Self::Cases => "equality-pattern case budget exhausted".into(),
            Self::Evaluation(reason) => reason,
        }
    }
}
