//! Error categories, diagnostics, and source chains.

use std::{error, fmt, io};

use natp_diagnostics::{Code, Diagnostic};
use serde_json::error::Category;

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

/// An execution or input failure, separate from a logical [`crate::Outcome`].
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

    /// Associate a diagnostic with an error category.
    pub fn from_diagnostic(kind: ErrorKind, diagnostic: Diagnostic) -> Self {
        Self {
            kind,
            diagnostic,
            source: None,
        }
    }

    /// Runtime error category, preserved through context wrappers.
    pub const fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// Structured message shared with the compiler and CLI.
    pub fn diagnostic(&self) -> &Diagnostic {
        &self.diagnostic
    }

    /// Diagnostic code, preserved when adding context.
    pub const fn code(&self) -> Code {
        self.diagnostic.code()
    }

    /// Primary explanation without rendering labels.
    pub fn message(&self) -> &str {
        self.diagnostic.message()
    }

    /// Logical path or source position, when available.
    pub fn location(&self) -> Option<&str> {
        self.diagnostic.location()
    }

    /// Corrective action, when available.
    pub fn help(&self) -> Option<&str> {
        self.diagnostic.help()
    }

    /// Replace the diagnostic code.
    #[must_use]
    pub fn with_code(mut self, code: Code) -> Self {
        self.diagnostic = self.diagnostic.with_code(code);

        self
    }

    /// Attach the offending field or source position.
    #[must_use]
    pub fn at(mut self, location: impl Into<String>) -> Self {
        self.diagnostic = self.diagnostic.at(location);

        self
    }

    /// Attach a corrective action.
    #[must_use]
    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.diagnostic = self.diagnostic.with_help(help);

        self
    }

    /// Add operation context while retaining this error as the source.
    #[must_use]
    pub fn context(self, operation: impl fmt::Display) -> Self {
        Self {
            kind: self.kind,
            diagnostic: self.diagnostic.clone().context(operation),
            source: Some(Box::new(self)),
        }
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

impl From<io::Error> for Error {
    fn from(source: io::Error) -> Self {
        let mut error = Self::with_kind(ErrorKind::Io, source.to_string());
        error = match source.kind() {
            io::ErrorKind::NotFound => error.with_help(
                "check that the path exists; for an external solver, supply its executable path",
            ),
            io::ErrorKind::PermissionDenied => error.with_help(
                "check access permissions and whether another process has locked the file",
            ),
            io::ErrorKind::AlreadyExists => {
                error.with_help("choose a new output path or directory")
            }
            _ => error,
        };

        error.source = Some(Box::new(source));

        error
    }
}

impl From<serde_json::Error> for Error {
    fn from(source: serde_json::Error) -> Self {
        let mut error = Self::with_kind(ErrorKind::Serialization, source.to_string()).with_help(match source.classify() {
            Category::Eof => "complete the JSON document; check for missing values, quotes, and closing brackets",
            Category::Syntax => "correct the JSON syntax at the reported position; use double-quoted field names and no trailing commas",
            Category::Data => "match the expected field types and variant names",
            Category::Io => "check that the input can be read completely",
        });

        if source.line() > 0 {
            error = error.at(format!(
                "line {}, column {}",
                source.line(),
                source.column()
            ));
        }

        error.source = Some(Box::new(source));

        error
    }
}
