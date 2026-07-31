use natp_core::{Assignment, Error};

use crate::policy::Exhaustion;

pub(super) enum SearchStop {
    Refuted(Vec<Assignment>),
    Exhausted(Exhaustion),
    Failed(Error),
}
