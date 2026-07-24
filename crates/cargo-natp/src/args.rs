use natp::Code;
use natp::Error;
use std::{ffi::OsString, path::PathBuf};

pub(crate) const HELP: &str = "\
Rust-native theorem system

Usage: cargo natp check <theorem.json>

Outcomes distinguish kernel Proven from bounded Verified.
Exit codes: 0 established, 1 refuted/inconclusive, 2 error.
";

enum Invocation {
    CargoSubcommand,
}

impl Invocation {
    fn spelling(self) -> &'static str {
        match self {
            Self::CargoSubcommand => "natp",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CommandName {
    Help,
    Check,
}

impl CommandName {
    const ACTIONS: [Self; 1] = [Self::Check];

    fn spelling(self) -> &'static str {
        match self {
            Self::Help => "help",
            Self::Check => "check",
        }
    }

    fn parse(name: &str) -> Option<Self> {
        if Flag::parse(name).is_some_and(Flag::is_help) || name == Self::Help.spelling() {
            Some(Self::Help)
        } else {
            Self::ACTIONS
                .into_iter()
                .find(|command| command.spelling() == name)
        }
    }

    fn expected_paths(self) -> &'static str {
        match self {
            Self::Check => "one theorem path",
            Self::Help => "no paths",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Flag {
    Help,
    ShortHelp,
}

impl Flag {
    const ALL: [Self; 2] = [Self::Help, Self::ShortHelp];

    fn spelling(self) -> &'static str {
        match self {
            Self::Help => "--help",
            Self::ShortHelp => "-h",
        }
    }

    fn parse(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|flag| flag.spelling() == name)
    }

    fn is_help(self) -> bool {
        matches!(self, Self::Help | Self::ShortHelp)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TraceMode {
    Hide,
}

pub(crate) enum Command {
    Help,
    Check {
        theorems: Vec<PathBuf>,
        trace: TraceMode,
    },
}

impl Command {
    pub(crate) fn parse(mut arguments: Vec<OsString>) -> Result<Self, Error> {
        if arguments
            .first()
            .is_some_and(|argument| argument == Invocation::CargoSubcommand.spelling())
        {
            arguments.remove(0);
        }

        let Some((name, arguments)) = arguments.split_first() else {
            return Ok(Self::Help);
        };

        let name = name.to_str().ok_or_else(|| {
            Error::new("command name is not valid UTF-8")
                .with_code(Code::Command)
                .with_help("use check; file paths may be OS-native")
        })?;

        let Some(command) = CommandName::parse(name) else {
            return Err(Error::new(format!("unknown command {name:?}"))
                .with_code(Code::Command)
                .with_help("run cargo natp --help for usage"));
        };

        if command == CommandName::Help {
            return Ok(Self::Help);
        }

        if arguments.len() == 1
            && arguments[0]
                .to_str()
                .and_then(Flag::parse)
                .is_some_and(Flag::is_help)
        {
            return Ok(Self::Help);
        }

        let mut paths = Vec::new();
        let trace = TraceMode::Hide;

        for argument in arguments {
            if argument.to_string_lossy().starts_with('-') {
                return Err(
                    Error::new(format!("unknown option {argument:?} for '{name}'"))
                        .with_code(Code::Option)
                        .with_help(if command == CommandName::Check {
                            "run cargo natp check --help for usage"
                        } else {
                            "this command takes positional paths"
                        }),
                );
            }

            paths.push(PathBuf::from(argument));
        }

        match (command, paths.as_slice()) {
            (CommandName::Check, [_]) => Ok(Self::Check {
                theorems: paths,
                trace,
            }),
            _ => {
                let expected = command.expected_paths();

                Err(Error::new(format!(
                    "'{name}' expects {expected}; received {} positional arguments",
                    paths.len()
                ))
                .with_code(Code::Arguments)
                .with_help(format!("run cargo natp {name} --help for usage")))
            }
        }
    }
}
