//! Typed catalogues for stable trace identifiers and diagnostic paths.

/// A trusted kernel rule with a stable certificate wire identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TraceRule {
    /// Empty sequence identity.
    SequenceEmpty,
    /// Canonical sequence variable.
    SequenceVariable,
    /// Concatenation associativity and identity.
    SequenceAppend,
    /// Reversal involution and anti-homomorphism.
    SequenceReverse,
    /// Sequence equality symmetry, reversal, and length congruence.
    SequenceEquality,
    /// Concatenation is empty exactly when all parts are empty.
    SequenceEmptiness,
    /// Length homomorphism and natural cancellation.
    NaturalEquality,
    /// Scalar equality reflexivity, symmetry, and literals.
    ElementEquality,
    /// Membership through emptiness, concatenation, and reversal.
    Membership,
    /// Universal membership and vacuity.
    UniversalMembership,
    /// Constant Boolean evaluation.
    BooleanConstant,
    /// Classical negation.
    Negation,
    /// Boolean connectives and material implication.
    Connectives,
    /// Truth under a partial valuation.
    BooleanEvaluation,
    /// Exhaustive classical Boolean cases.
    ClassicalCases,
    /// Unbounded universal introduction.
    UniversalIntroduction,
}

impl TraceRule {
    /// All kernel rules in declaration order.
    pub const ALL: [Self; 16] = [
        Self::SequenceEmpty,
        Self::SequenceVariable,
        Self::SequenceAppend,
        Self::SequenceReverse,
        Self::SequenceEquality,
        Self::SequenceEmptiness,
        Self::NaturalEquality,
        Self::ElementEquality,
        Self::Membership,
        Self::UniversalMembership,
        Self::BooleanConstant,
        Self::Negation,
        Self::Connectives,
        Self::BooleanEvaluation,
        Self::ClassicalCases,
        Self::UniversalIntroduction,
    ];

    /// Stable identifier used by JSON certificates and trace rendering.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SequenceEmpty => "S1.Empty",
            Self::SequenceVariable => "S2.Variable",
            Self::SequenceAppend => "S3.AppendAssociativityIdentity",
            Self::SequenceReverse => "S4.ReverseInvolutionAntiHomomorphism",
            Self::SequenceEquality => "S5.EqualitySymmetryReverseInjectivityLengthCongruence",
            Self::SequenceEmptiness => "S6.AppendEmptyIffAllPartsEmpty",
            Self::NaturalEquality => "N1.LengthHomomorphismNaturalCancellation",
            Self::ElementEquality => "E1.ElementEqualityReflexivitySymmetryConstants",
            Self::Membership => "P1.MembershipEmptyAppendReverse",
            Self::UniversalMembership => "P2.AllEmptyAppendReverseVacuity",
            Self::BooleanConstant => "L1.BooleanConstant",
            Self::Negation => "L2.Negation",
            Self::Connectives => "L3.ConnectivesImplicationDefinition",
            Self::BooleanEvaluation => "L4.BooleanEvaluation",
            Self::ClassicalCases => "L5.ClassicalCases",
            Self::UniversalIntroduction => "K1.UniversalIntroduction",
        }
    }
}

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
