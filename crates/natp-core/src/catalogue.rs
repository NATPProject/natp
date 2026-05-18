/// Named representation fields that may appear in validation diagnostics.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiagnosticField {
    /// The theorem's schema version.
    SchemaVersion,
    /// The theorem name.
    Name,
    /// Ordered quantifier declarations.
    Variables,
    /// The quantification scope.
    Scope,
    /// The per-sequence length bound.
    MaximumLength,
    /// The theorem proposition.
    Claim,
    /// A standalone sequence expression.
    Expression,
    /// Concrete inputs in quantifier order.
    Assignments,
    /// The contents of a concrete assignment.
    Value,
    /// The exact theorem stored in a certificate.
    Theorem,
    /// The canonical proposition stored in a certificate.
    NormalForm,
    /// The certificate normalisation trace.
    Normalisation,
    /// The canonical atom table.
    Atoms,
    /// The propositional deduction tree.
    Deduction,
    /// A Boolean case-split node.
    Cases,
    /// The atom-table index selected by a split.
    Atom,
    /// The branch assuming the selected atom is false.
    OnFalse,
    /// The branch assuming the selected atom is true.
    OnTrue,
}

impl DiagnosticField {
    /// Every diagnostic field in declaration order.
    pub const ALL: [Self; 18] = [
        Self::SchemaVersion,
        Self::Name,
        Self::Variables,
        Self::Scope,
        Self::MaximumLength,
        Self::Claim,
        Self::Expression,
        Self::Assignments,
        Self::Value,
        Self::Theorem,
        Self::NormalForm,
        Self::Normalisation,
        Self::Atoms,
        Self::Deduction,
        Self::Cases,
        Self::Atom,
        Self::OnFalse,
        Self::OnTrue,
    ];

    /// Stable spelling in diagnostic locations.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SchemaVersion => "schema_version",
            Self::Name => "name",
            Self::Variables => "variables",
            Self::Scope => "scope",
            Self::MaximumLength => "max_len",
            Self::Claim => "claim",
            Self::Expression => "expression",
            Self::Assignments => "assignments",
            Self::Value => "value",
            Self::Theorem => "theorem",
            Self::NormalForm => "normalized",
            Self::Normalisation => "normalization",
            Self::Atoms => "atoms",
            Self::Deduction => "deduction",
            Self::Cases => "Cases",
            Self::Atom => "atom",
            Self::OnFalse => "on_false",
            Self::OnTrue => "on_true",
        }
    }

    /// Qualify a child field using dot notation.
    pub fn child(self, child: Self) -> String {
        format!("{}.{}", self.as_str(), child.as_str())
    }

    /// Address an indexed value under this field.
    pub fn index(self, index: usize) -> String {
        format!("{}[{index}]", self.as_str())
    }
}

/// Typed edges used while traversing the diagnostic path of a syntax tree.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SyntaxEdge {
    /// The single argument of a unary syntax node.
    Unary,
    /// The first argument of a binary syntax node.
    First,
    /// The second argument of a binary syntax node.
    Second,
}

impl SyntaxEdge {
    /// Every syntax traversal edge.
    pub const ALL: [Self; 3] = [Self::Unary, Self::First, Self::Second];

    /// Stable diagnostic suffix for this syntax edge.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unary => ".args",
            Self::First => ".args[0]",
            Self::Second => ".args[1]",
        }
    }
}
