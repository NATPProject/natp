use crate::policy::Exhaustion;
use natp_core::{Assignment, Error};

pub(super) enum SearchStop {
    Refuted(Vec<Assignment>),
    Exhausted(Exhaustion),
    Failed(Error),
}
