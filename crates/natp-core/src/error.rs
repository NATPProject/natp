//! Error categories, diagnostics, and source chains.

use natp_diagnostics::{Code, Diagnostic};
use std::{error, fmt};

/// Error categories for callers that do not parse diagnostic messages.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorKind {
    /// A theorem, assignment, or command violates the accepted input contract.
    InvalidInput,
    /// A configured size, syntax, or execution budget was exceeded.
    ResourceLimit,
    /// A file or external process could not be accessed.
    Io,
    /// A JSON representation could not be encoded or decoded.
    Serialization,
    /// Internally generated evidence failed an independent check.
    Internal,
    /// A report did not meet the caller's requested outcome.
    UnexpectedOutcome,
}

impl ErrorKind {
    /// Every currently defined runtime error category.
    pub const ALL: [Self; 6] = [
        Self::InvalidInput,
        Self::ResourceLimit,
        Self::Io,
        Self::Serialization,
        Self::Internal,
        Self::UnexpectedOutcome,
    ];

    /// Default diagnostic code for this runtime category.
    pub const fn default_code(self) -> Code {
        match self {
            Self::InvalidInput => Code::InvalidInput,
            Self::ResourceLimit => Code::ResourceLimit,
            Self::Io => Code::Io,
            Self::Serialization => Code::Serialization,
            Self::Internal => Code::Internal,
            Self::UnexpectedOutcome => Code::UnexpectedOutcome,
        }
    }
}

/// An input or execution failure.
///
/// Typed diagnostic data is shared with macros and the CLI; the runtime also
/// retains an underlying cause through [`std::error::Error::source`].
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    diagnostic: Diagnostic,
    source: Option<Box<dyn error::Error + Send + Sync>>,
}

impl Error {
    /// Construct an invalid-input error.
    pub fn new(message: impl Into<String>) -> Self {
        Self::with_kind(ErrorKind::InvalidInput, message)
    }

    /// Construct an error in the given category.
    pub fn with_kind(kind: ErrorKind, message: impl Into<String>) -> Self {
        let code = kind.default_code();

        Self {
            kind,
            diagnostic: Diagnostic::new(code, message),
            source: None,
        }
    }

    /// Runtime error category, preserved through context wrappers.
    pub const fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// Primary explanation without rendering labels.
    pub fn message(&self) -> &str {
        self.diagnostic.message()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.source
            .as_deref()
            .map(|source| source as &dyn error::Error)
    }
}
