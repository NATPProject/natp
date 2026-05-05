use crate::Code;

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
