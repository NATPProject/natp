//! Shared diagnostic codes and messages.

mod code;
mod message;

pub use code::Code;
pub use message::Diagnostic;

/// Build a typed diagnostic using standard formatting arguments.
///
/// The first argument is a [`Code`] variant name.
#[macro_export]
macro_rules! diagnostic {
    ($code:ident, $($arguments:tt)*) => {
        $crate::Diagnostic::new($crate::Code::$code, format!($($arguments)*))
    };
}
