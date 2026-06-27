//! SMT query prelude.
use crate::vocabulary::{Logic, SmtCommand};
pub(super) fn generate() -> String {
    SmtCommand::SetLogic.render(&[Logic::All.token().into()])
}
