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

        let mut decoder = serde_json::Deserializer::from_slice(&bytes);
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
