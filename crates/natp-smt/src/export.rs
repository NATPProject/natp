//! Persist SMT queries for export and solver execution.

use crate::{Obligation, induction, vocabulary::OutputFile};
use natp_core::{Error, Theorem};
use std::path::{Path, PathBuf};

pub(super) fn write_obligation(
    obligation: &Obligation,
    directory: &Path,
) -> Result<PathBuf, Error> {
    let path = directory.join(OutputFile::Obligation.name(&obligation.name));
    std::fs::write(&path, &obligation.source).map_err(|error| {
        Error::from(error).context(format!("writing SMT obligation {}", path.display()))
    })?;

    Ok(path)
}

/// Export to a new directory; this creates obligations without proving them.
///
/// # Errors
/// Returns validation or filesystem errors, including an already-existing
/// output directory. Failed exports may leave a partially populated directory.
pub fn write_obligations(theorem: &Theorem, path: impl AsRef<Path>) -> Result<(), Error> {
    let plan = induction(theorem)?;
    let path = path.as_ref();
    std::fs::create_dir(path).map_err(|error| {
        Error::from(error).context(format!("creating SMT export directory {}", path.display()))
    })?;

    for obligation in plan.obligations {
        write_obligation(&obligation, path)?;
    }

    Ok(())
}
