macro_rules! catalogue {
    ($($name:ident = $number:literal => $description:literal),+ $(,)?) => {
        /// Stable diagnostic identities. Match variants instead of parsing display text.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        #[repr(u16)]
        pub enum Code {
            $(#[doc = $description] $name = $number,)+
        }
        impl Code {
            /// All registered diagnostics, in catalogue order.
            pub const ALL: &'static [Self] = &[$(Self::$name,)+];
            /// Short explanation of this diagnostic class.
            pub const fn description(self) -> &'static str {
                match self { $(Self::$name => $description,)+ }
            }
        }
    };
}

catalogue! {
    InvalidInput = 1000 => "Input violates a public API contract.",
    SchemaVersion = 1001 => "Unsupported theorem schema version.",
    TheoremName = 1002 => "The theorem name is empty or too long.",
    QuantifierCount = 1003 => "Unsupported number of quantified variables.",
    LengthBound = 1004 => "Sequence length bound exceeds the supported limit.",
    UnknownVariable = 1005 => "A variable ID does not name a declared quantifier.",
    VariableDomain = 1006 => "A scalar was used as a sequence, or vice versa.",
    ElementSort = 1007 => "Element sorts do not agree.",
    UnboundElement = 1008 => "A bound element escaped its all predicate.",
    NestedBinder = 1009 => "Nested all predicates are unsupported.",
    AssignmentCount = 1010 => "Assignment count differs from quantifier count.",
    AssignmentSort = 1011 => "An assignment has the wrong domain or element sort.",
    AssignmentBound = 1012 => "An assignment is outside the theorem's length scope.",
    CertificateTheorem = 1101 => "Certificate is bound to a different theorem.",
    CertificateNormalForm = 1102 => "Certificate normal form differs from recomputation.",
    CertificateTrace = 1103 => "Certificate normalisation trace differs from recomputation.",
    CertificateAtoms = 1104 => "Certificate atom table is not canonical.",
    CertificateDeduction = 1105 => "A certificate deduction branch is invalid.",
    ResourceLimit = 2000 => "A configured resource limit was exceeded.",
    SyntaxBudget = 2001 => "Syntax exceeds the depth or node budget.",
    EvaluationBudget = 2002 => "Concrete evaluation exhausted its work budget.",
    Io = 3000 => "An input/output operation failed.",
    Serialization = 4000 => "A serialised representation is malformed.",
    Internal = 5000 => "An internal invariant or independent check failed.",
    UnexpectedOutcome = 6000 => "The report does not satisfy the requested outcome.",
    Command = 7001 => "Unknown or invalid command name.",
    Option = 7002 => "Unknown, misplaced, or repeated command-line option.",
    Arguments = 7003 => "Command received the wrong number of positional arguments.",
    UnsupportedExpression = 8001 => "Expression is outside the symbolic DSL.",
    UnsupportedOperator = 8002 => "Operator is outside the symbolic DSL.",
    UnknownSymbol = 8003 => "Identifier is not a quantified or bound symbol.",
    ClaimBlock = 8004 => "Claim block must contain one expression.",
    NestedAll = 8005 => "Nested all closures are unsupported.",
    AllClosure = 8006 => "An all predicate needs one plain closure parameter.",
    ShadowedBinder = 8007 => "An all binder shadows a quantified name or uses a pattern.",
    CallArity = 8008 => "A DSL operation received the wrong number of arguments.",
    ExpectedProposition = 8009 => "A value was used where a proposition is required.",
    UnknownOperation = 8010 => "Call is not a recognised named DSL operation.",
    QuantifierSyntax = 8011 => "Quantified closure parameters are malformed.",
    DuplicateQuantifier = 8012 => "Quantified closure parameters repeat a name.",
    MacroQuantifierLimit = 8013 => "Macro exceeds the supported quantifier count.",
    ClaimSyntax = 8014 => "Claim body cannot be parsed as a symbolic expression.",
}
