//! Bounded JSON input.

use crate::{Error, ErrorKind};
use serde::de::DeserializeOwned;
use std::{fs::File, io::Read, path::Path};

/// Decode JSON within the byte limit, adding the file path to errors.
///
/// # Errors
/// Returns [`ErrorKind::ResourceLimit`] for oversized input, or preserves the
/// underlying I/O or JSON error. Semantic validation remains the caller's job.
pub fn load_json<T: DeserializeOwned>(path: &Path, limit: usize) -> Result<T, Error> {
    let read = || -> Result<T, Error> {
        let file = File::open(path)?;
        let mut bytes = Vec::new();
        file.take((limit as u64).saturating_add(1))
            .read_to_end(&mut bytes)?;

        if bytes.len() > limit {
            return Err(Error::with_kind(
                ErrorKind::ResourceLimit,
                format!("JSON input exceeds {limit} bytes"),
            ));
        }

        let value = serde_json::from_slice(&bytes)?;

        Ok(value)
    };

    read().map_err(|error| error.context(format!("read {}", path.display())))
}
