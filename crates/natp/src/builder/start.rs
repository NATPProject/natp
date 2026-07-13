use super::Quantified;
use crate::SymbolicDomain;

enum BuilderSetting {
    DefaultName,
}

impl BuilderSetting {
    fn value(self) -> &'static str {
        match self {
            Self::DefaultName => "anonymous theorem",
        }
    }
}

/// The starting point for a named, universally quantified theorem.
#[derive(Clone, Debug)]
#[must_use = "configure quantifiers and attach a claim"]
pub struct Builder {
    name: String,
}

/// Start building an unbounded theorem.
pub fn theorem() -> Builder {
    Builder {
        name: BuilderSetting::DefaultName.value().into(),
    }
}

impl Builder {
    /// Introduce the first universal symbolic variable of domain `D`, initially unbounded.
    pub fn forall<D: SymbolicDomain>(self) -> Quantified<(D,)> {
        Quantified::first(self.name, D::DOMAIN)
    }
}
