//! Command timing and output.
mod output;
pub use output::Output;
use std::time::Duration;
/// Timing settings for the direct-child process runner.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExecutionSetting {
    /// Sleep interval between polls, shortened to the remaining deadline.
    PollInterval,
}

impl ExecutionSetting {
    /// Every process timing setting.
    pub const ALL: [Self; 1] = [Self::PollInterval];

    /// Default duration associated with this execution setting.
    pub const fn duration(self) -> Duration {
        match self {
            Self::PollInterval => Duration::from_millis(10),
        }
    }
}
