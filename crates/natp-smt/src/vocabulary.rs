//! SMT symbol spellings and solver policy.

use natp_core::{Domain, Element};
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Sort {
    Integer,
    Boolean,
    BitVector32,
    List(Element),
}
impl Sort {
    pub(super) fn element(element: Element) -> Self {
        match element {
            Element::I32 => Self::BitVector32,
            Element::Bool => Self::Boolean,
        }
    }

    pub(super) fn domain(domain: Domain) -> Self {
        match domain {
            Domain::Scalar(element) => Self::element(element),
            Domain::Sequence(element) => Self::List(element),
        }
    }

    pub(super) fn render(self) -> String {
        match self {
            Self::Integer => "Int".into(),
            Self::Boolean => "Bool".into(),
            Self::BitVector32 => format!("(_ BitVec {})", RuntimePolicy::ElementBits.number()),
            Self::List(element) => format!("List{}", Self::suffix(element)),
        }
    }

    pub(super) fn suffix(element: Element) -> &'static str {
        match element {
            Element::I32 => "I",
            Element::Bool => "B",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Operator {
    Equal,
    Add,
    Not,
    Or,
    IfThenElse,
}
impl Operator {
    pub(super) fn token(self) -> &'static str {
        match self {
            Self::Equal => "=",
            Self::Add => "+",
            Self::Not => "not",
            Self::Or => "or",
            Self::IfThenElse => "ite",
        }
    }

    pub(super) fn apply(self, operands: &[String]) -> String {
        format!("({} {})", self.token(), operands.join(" "))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SmtCommand {
    SetLogic,
    DeclareDatatype,
    DeclareConstant,
    DefineRecursiveFunction,
    Assert,
    CheckSat,
}
impl SmtCommand {
    pub(super) fn token(self) -> &'static str {
        match self {
            Self::SetLogic => "set-logic",
            Self::DeclareDatatype => "declare-datatype",
            Self::DeclareConstant => "declare-const",
            Self::DefineRecursiveFunction => "define-fun-rec",
            Self::Assert => "assert",
            Self::CheckSat => "check-sat",
        }
    }

    pub(super) fn render(self, operands: &[String]) -> String {
        if operands.is_empty() {
            format!("({})\n", self.token())
        } else {
            format!("({} {})\n", self.token(), operands.join(" "))
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Logic {
    All,
}
impl Logic {
    pub(super) fn token(self) -> &'static str {
        match self {
            Self::All => "ALL",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ListSymbol {
    Empty,
    Cons,
    Head,
    Tail,
    Append,
    Reverse,
    Length,
    Membership,
}
impl ListSymbol {
    pub(super) fn render(self, element: Element) -> String {
        let name = match self {
            Self::Empty => "nil",
            Self::Cons => "cons",
            Self::Head => "head",
            Self::Tail => "tail",
            Self::Append => "app",
            Self::Reverse => "rev",
            Self::Length => "len",
            Self::Membership => "mem",
        };

        format!("{name}{}", Sort::suffix(element))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Binding {
    Quantified(usize),
    ListLeft,
    ListRight,
    Member,
}
impl Binding {
    pub(super) fn render(self) -> String {
        match self {
            Self::Quantified(id) => format!("v{id}"),
            Self::ListLeft => "xs".into(),
            Self::ListRight => "ys".into(),
            Self::Member => "x".into(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ObligationKind {
    Direct,
}
impl ObligationKind {
    pub(super) fn name(self) -> &'static str {
        match self {
            Self::Direct => "direct",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SolverSetting {
    Executable,
}
impl SolverSetting {
    pub(super) fn text(self) -> &'static str {
        match self {
            Self::Executable => "z3",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum RuntimePolicy {
    TimeoutSeconds,
    ElementBits,
}
impl RuntimePolicy {
    pub(super) fn number(self) -> usize {
        match self {
            Self::TimeoutSeconds => 10,
            Self::ElementBits => 32,
        }
    }

    pub(super) fn timeout() -> Duration {
        Duration::from_secs(Self::TimeoutSeconds.number() as u64)
    }
}
