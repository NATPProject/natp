//! Solver defaults.

use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SolverSetting {
    Executable,
}
impl SolverSetting {
    pub(super) fn text(self) -> &'static str {
        match self {
            Self::Executable => "z3",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum RuntimePolicy {
    TimeoutSeconds,
}
impl RuntimePolicy {
    pub(super) fn number(self) -> usize {
        match self {
            Self::TimeoutSeconds => 10,
        }
    }

    pub(super) fn timeout() -> Duration {
        Duration::from_secs(Self::TimeoutSeconds.number() as u64)
    }
}
