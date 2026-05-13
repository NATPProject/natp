use natp_diagnostics::Code;

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
