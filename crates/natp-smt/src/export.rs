//! Persist SMT queries for export and solver execution.

use std::{
    fs,
    path::{Path, PathBuf},
};

use natp_core::{Error, Theorem};

use crate::{Obligation, induction, vocabulary::OutputFile};

pub(super) fn write_obligation(
    obligation: &Obligation,
    directory: &Path,
) -> Result<PathBuf, Error> {
    let path = directory.join(OutputFile::Obligation.name(&obligation.name));
    fs::write(&path, &obligation.source).map_err(|error| {
        Error::from(error).context(format!("writing SMT obligation {}", path.display()))
    })?;

    Ok(path)
}

/// Write SMT queries to a new directory.
///
/// # Errors
/// Returns validation or filesystem errors, including an already-existing
/// output directory. Failed exports may leave a partially populated directory.
pub fn write_obligations(theorem: &Theorem, path: impl AsRef<Path>) -> Result<(), Error> {
    let plan = induction(theorem)?;
    let path = path.as_ref();
    fs::create_dir(path).map_err(|error| {
        Error::from(error).context(format!("creating SMT export directory {}", path.display()))
    })?;

    for obligation in plan.obligations {
        write_obligation(&obligation, path)?;
    }

    theorem.save(path.join(OutputFile::Theorem.name("")))?;

    Ok(())
}
