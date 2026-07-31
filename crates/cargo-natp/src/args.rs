use std::{convert::identity, ffi::OsString, path::PathBuf};

use natp::{Code, Error};

pub(crate) const HELP: &str = "\
Rust-native theorem system

Usage: cargo natp check [--trace] <theorem.json>...
       cargo natp certify <theorem.json> <certificate.json>
       cargo natp check-certificate <theorem.json> <certificate.json>
       cargo natp smt <theorem.json> [z3-executable]
       cargo natp emit-smt <theorem.json> <new-directory>

Outcomes distinguish kernel Proven, bounded Verified, and SmtValidated.
Exit codes: 0 established/exported, 1 refuted/inconclusive/invalid certificate, 2 error.
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

enum CommandLimit {
    SuggestionDistance,
}

impl CommandLimit {
    fn value(self) -> usize {
        match self {
            Self::SuggestionDistance => 2,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CommandName {
    Help,
    Check,
    Certify,
    CheckCertificate,
    Smt,
    EmitSmt,
}

impl CommandName {
    const ACTIONS: [Self; 5] = [
        Self::Check,
        Self::Certify,
        Self::CheckCertificate,
        Self::Smt,
        Self::EmitSmt,
    ];

    fn spelling(self) -> &'static str {
        match self {
            Self::Help => "help",
            Self::Check => "check",
            Self::Certify => "certify",
            Self::CheckCertificate => "check-certificate",
            Self::Smt => "smt",
            Self::EmitSmt => "emit-smt",
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
            Self::Check => "at least one theorem path",
            Self::Certify | Self::CheckCertificate => {
                "two paths: <theorem.json> <certificate.json>"
            }
            Self::Smt => "one or two paths: <theorem.json> [z3-executable]",
            Self::EmitSmt => "two paths: <theorem.json> <new-directory>",
            Self::Help => "no paths",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Flag {
    Help,
    ShortHelp,
    Trace,
    LiteralPaths,
}

impl Flag {
    const ALL: [Self; 4] = [Self::Help, Self::ShortHelp, Self::Trace, Self::LiteralPaths];

    fn spelling(self) -> &'static str {
        match self {
            Self::Help => "--help",
            Self::ShortHelp => "-h",
            Self::Trace => "--trace",
            Self::LiteralPaths => "--",
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
    Show,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ArgumentMode {
    Options,
    LiteralPaths,
}

pub(crate) enum Command {
    Help,
    Check {
        theorems: Vec<PathBuf>,
        trace: TraceMode,
    },
    Certify {
        theorem: PathBuf,
        certificate: PathBuf,
    },
    CheckCertificate {
        theorem: PathBuf,
        certificate: PathBuf,
    },
    Smt {
        theorem: PathBuf,
        executable: Option<PathBuf>,
    },
    EmitSmt {
        theorem: PathBuf,
        directory: PathBuf,
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
                .with_help(
                    "use check, certify, check-certificate, smt, or emit-smt; file paths may be OS-native",
                )
        })?;

        let Some(command) = CommandName::parse(name) else {
            let suggestion = CommandName::ACTIONS
                .into_iter()
                .map(CommandName::spelling)
                .find(|candidate| {
                    edit_distance(name, candidate) <= CommandLimit::SuggestionDistance.value()
                });
            let help = match suggestion {
                Some(candidate) => format!("did you mean 'cargo natp {candidate}'? Run cargo natp --help for usage"),
                None => "use check, certify, check-certificate, smt, or emit-smt; run cargo natp --help for usage".into(),
            };

            return Err(Error::new(format!("unknown command {name:?}"))
                .with_code(Code::Command)
                .with_help(help));
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
        let mut trace = TraceMode::Hide;
        let mut mode = ArgumentMode::Options;

        for argument in arguments {
            let flag = if mode == ArgumentMode::LiteralPaths {
                None
            } else {
                argument.to_str().and_then(Flag::parse)
            };

            if flag == Some(Flag::LiteralPaths) {
                mode = ArgumentMode::LiteralPaths;
                continue;
            }

            if flag == Some(Flag::Trace) && command == CommandName::Check {
                if trace == TraceMode::Show {
                    return Err(Error::new("--trace was specified more than once")
                        .with_code(Code::Option)
                        .with_help("pass --trace once to show kernel derivations"));
                }

                trace = TraceMode::Show;
                continue;
            }

            if mode == ArgumentMode::Options && argument.to_string_lossy().starts_with('-') {
                return Err(Error::new(format!("unknown option {argument:?} for '{name}'"))
                    .with_code(Code::Option)
                    .with_help(if command == CommandName::Check {
                        "the only check option is --trace; use -- before a filename beginning with '-'"
                    } else {
                        "this command takes positional paths; use -- before a filename beginning with '-'"
                    }));
            }

            paths.push(PathBuf::from(argument));
        }

        match (command, paths.as_slice()) {
            (CommandName::Check, [_, ..]) => Ok(Self::Check {
                theorems: paths,
                trace,
            }),
            (CommandName::Certify, [theorem, certificate]) => Ok(Self::Certify {
                theorem: theorem.clone(),
                certificate: certificate.clone(),
            }),
            (CommandName::CheckCertificate, [theorem, certificate]) => Ok(Self::CheckCertificate {
                theorem: theorem.clone(),
                certificate: certificate.clone(),
            }),
            (CommandName::Smt, [theorem]) => Ok(Self::Smt {
                theorem: theorem.clone(),
                executable: None,
            }),
            (CommandName::Smt, [theorem, executable]) => Ok(Self::Smt {
                theorem: theorem.clone(),
                executable: Some(executable.clone()),
            }),
            (CommandName::EmitSmt, [theorem, directory]) => Ok(Self::EmitSmt {
                theorem: theorem.clone(),
                directory: directory.clone(),
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

fn edit_distance(left: &str, right: &str) -> usize {
    let mut previous: Vec<_> = (0..=right.chars().count()).collect();

    for (i, a) in left.chars().enumerate() {
        let mut current = vec![i + 1];

        for (j, b) in right.chars().enumerate() {
            current.push(
                (previous[j + 1] + 1)
                    .min(current[j] + 1)
                    .min(previous[j] + usize::from(a != b)),
            );
        }
        previous = current;
    }

    previous.last().copied().map_or(left.len(), identity)
}
