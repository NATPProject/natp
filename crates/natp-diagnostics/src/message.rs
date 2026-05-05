use crate::Code;
use std::fmt;

/// A diagnostic code and message.
#[derive(Clone, Debug, PartialEq, Eq)]
#[must_use]
pub struct Diagnostic {
    code: Code,
    message: String,
}

impl Diagnostic {
    /// Construct a diagnostic.
    pub fn new(code: Code, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
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
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "[{}] {}", self.code, self.message)
    }
}
