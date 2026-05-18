//! Shared resource limits.

/// Resource limits for theorem validation, certificate checking, and process execution.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResourceLimit {
    /// UTF-8 bytes in a theorem name.
    NameBytes,
    /// Universally quantified variables.
    Variables,
    /// Per-sequence length in finite verification.
    BoundedLength,
    /// Recursive syntax depth.
    SyntaxDepth,
    /// Total syntax nodes.
    SyntaxNodes,
    /// Bytes in persisted theorem JSON.
    TheoremBytes,
    /// Bytes in persisted certificate JSON.
    CertificateBytes,
    /// Independent Boolean atoms in kernel evidence.
    KernelAtoms,
    /// Bytes captured independently from each process output stream.
    ProcessOutputBytes,
}

impl ResourceLimit {
    /// All resource-limit keys in declaration order.
    pub const ALL: [Self; 9] = [
        Self::NameBytes,
        Self::Variables,
        Self::BoundedLength,
        Self::SyntaxDepth,
        Self::SyntaxNodes,
        Self::TheoremBytes,
        Self::CertificateBytes,
        Self::KernelAtoms,
        Self::ProcessOutputBytes,
    ];

    /// Default budget for this resource; units are documented on its variant.
    pub const fn value(self) -> usize {
        match self {
            Self::NameBytes => 200,
            Self::Variables => 8,
            Self::BoundedLength => 4096,
            Self::SyntaxDepth => 48,
            Self::SyntaxNodes => 2048,
            Self::TheoremBytes | Self::ProcessOutputBytes => 1024 * 1024,
            Self::CertificateBytes => 8 * 1024 * 1024,
            Self::KernelAtoms => 12,
        }
    }
}

/// Supported theorem wire formats. Version changes require an explicit migration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SchemaVersion {
    /// Current typed sequence, scalar, and natural representation.
    V2,
}

impl SchemaVersion {
    /// Every supported schema version.
    pub const ALL: [Self; 1] = [Self::V2];

    /// Stable numeric wire identifier.
    pub const fn number(self) -> u32 {
        match self {
            Self::V2 => 2,
        }
    }
}

/// Version of the serialised theorem format.
pub const SCHEMA_VERSION: u32 = SchemaVersion::V2.number();
/// Maximum UTF-8 byte length of a nonempty theorem name.
pub const MAX_NAME_BYTES: usize = ResourceLimit::NameBytes.value();
/// Maximum number of universally quantified sequence or scalar variables.
pub const MAX_VARIABLES: usize = ResourceLimit::Variables.value();
/// Largest permitted bound for each sequence in finite verification.
pub const MAX_BOUNDED_LENGTH: usize = ResourceLimit::BoundedLength.value();
/// Maximum recursive syntax depth accepted at every backend boundary.
pub const MAX_SYNTAX_DEPTH: usize = ResourceLimit::SyntaxDepth.value();
/// Maximum total syntax nodes in one theorem.
pub const MAX_SYNTAX_NODES: usize = ResourceLimit::SyntaxNodes.value();
/// Maximum serialised theorem size, enforced before JSON decoding.
pub const MAX_THEOREM_BYTES: usize = ResourceLimit::TheoremBytes.value();
