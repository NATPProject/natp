use std::fmt;

use crate::Code;

/// A code, message, optional location, and help text.
#[derive(Clone, Debug, PartialEq, Eq)]
#[must_use]
pub struct Diagnostic {
    code: Code,
    message: String,
    location: Option<String>,
    help: Option<String>,
}

impl Diagnostic {
    /// Construct a diagnostic with no location or help text.
    pub fn new(code: Code, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            location: None,
            help: None,
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

    /// Corrective action, when available.
    pub fn help(&self) -> Option<&str> {
        self.help.as_deref()
    }

    /// Replace the code, retaining the message, location, and help text.
    pub fn with_code(mut self, code: Code) -> Self {
        self.code = code;

        self
    }

    /// Attach a field path or source location.
    pub fn at(mut self, location: impl Into<String>) -> Self {
        self.location = Some(location.into());

        self
    }

    /// Set the help text.
    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());

        self
    }

    /// Prefix the message with operation context, retaining the code, location, and help text.
    pub fn context(mut self, operation: impl fmt::Display) -> Self {
        self.message = format!("{operation}: {}", self.message);

        self
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "[{}] {}", self.code, self.message)?;

        if let Some(location) = &self.location {
            write!(formatter, "\n  at: {location}")?;
        }

        if let Some(help) = &self.help {
            write!(formatter, "\n  help: {help}")?;
        }

        Ok(())
    }
}
