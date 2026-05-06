use crate::Code;
use std::fmt;

/// A diagnostic code, message and optional location.
#[derive(Clone, Debug, PartialEq, Eq)]
#[must_use]
pub struct Diagnostic {
    code: Code,
    message: String,
    location: Option<String>,
}

impl Diagnostic {
    /// Construct a diagnostic without a location.
    pub fn new(code: Code, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            location: None,
        }
    }

    /// The diagnostic code.
    pub const fn code(&self) -> Code {
        self.code
    }

    /// Primary explanation, without rendering labels.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Field path or source position, when supplied by the caller.
    pub fn location(&self) -> Option<&str> {
        self.location.as_deref()
    }

    /// Attach a field path or source location.
    pub fn at(mut self, location: impl Into<String>) -> Self {
        self.location = Some(location.into());

        self
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "[{}] {}", self.code, self.message)?;

        if let Some(location) = &self.location {
            write!(formatter, "\n  at: {location}")?;
        }

        Ok(())
    }
}
