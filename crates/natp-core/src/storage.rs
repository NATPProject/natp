//! Bounded JSON input and atomic replacement of persisted evidence.

use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use serde::{Serialize, de::DeserializeOwned};
use serde_json::Deserializer;
use tempfile::NamedTempFile;

use crate::{Error, ErrorKind};

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

        let mut decoder = Deserializer::from_slice(&bytes);
        let value = serde_path_to_error::deserialize(&mut decoder).map_err(|error| {
            let field = error.path().to_string();
            let cause = error.into_inner();
            let position = format!("{field} (line {}, column {})", cause.line(), cause.column());

            Error::from(cause).at(position)
        })?;

        // Deserialising one value does not reject additional input; require end-of-file.
        decoder.end()?;

        Ok(value)
    };

    read().map_err(|error| error.context(format!("read {}", path.display())))
}

/// Encode JSON and atomically replace a destination after enforcing its byte limit.
///
/// # Errors
/// Returns an I/O, serialisation, or resource-limit error. The parent directory
/// must already exist. Existing destination contents survive a failed write.
pub fn save_json<T: Serialize>(value: &T, path: &Path, limit: usize) -> Result<(), Error> {
    let write = || -> Result<(), Error> {
        let bytes = serde_json::to_vec_pretty(value)?;

        if bytes.len() > limit {
            return Err(Error::with_kind(
                ErrorKind::ResourceLimit,
                format!("JSON output exceeds {limit} bytes"),
            ));
        }

        let parent = match path.parent() {
            Some(parent) if !parent.as_os_str().is_empty() => parent,
            _ => Path::new("."),
        };

        let mut temporary = NamedTempFile::new_in(parent)?;
        temporary.write_all(&bytes)?;
        temporary.as_file().sync_all()?;

        temporary
            .persist(path)
            .map_err(|error| Error::from(error.error))?;

        Ok(())
    };

    write().map_err(|error| error.context(format!("write {}", path.display())))
}
