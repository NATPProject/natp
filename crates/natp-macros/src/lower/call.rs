use natp_diagnostics::diagnostic;
use syn::{Expr, ExprCall};

#[derive(Clone, Copy)]
pub(crate) enum Operation {
    Empty,
    Reverse,
    Length,
    Append,
    IsEmpty,
    Contains,
    All,
    Implies,
}

impl Operation {
    const ALL: [Self; 8] = [
        Self::Empty,
        Self::Reverse,
        Self::Length,
        Self::Append,
        Self::IsEmpty,
        Self::Contains,
        Self::All,
        Self::Implies,
    ];

    pub(super) fn spellings(self) -> &'static [&'static str] {
        match self {
            Self::Empty => &["empty"],
            Self::Reverse => &["reverse"],
            Self::Length => &["length"],
            Self::Append => &["append"],
            Self::IsEmpty => &["is_empty"],
            Self::Contains => &["contains"],
            Self::All => &["all"],
            Self::Implies => &["implies", Self::IMPLICATION_REWRITE],
        }
    }

    pub(crate) const IMPLICATION_REWRITE: &'static str = "__natp_implies";

    fn named(name: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|operation| operation.spellings().contains(&name))
    }

    fn signature(self) -> (usize, &'static str) {
        match self {
            Self::Empty => (0, "empty()"),
            Self::Reverse => (1, "reverse(xs)"),
            Self::Length => (1, "length(xs)"),
            Self::Append => (2, "append(xs, ys)"),
            Self::IsEmpty => (1, "is_empty(xs)"),
            Self::Contains => (2, "contains(xs, x)"),
            Self::All => (2, "all(xs, |e| predicate)"),
            Self::Implies => (2, "implies(a, b)"),
        }
    }
}

/// A recognised DSL call with a validated name and arity.
/// Its operands are lowered separately.
pub(super) struct DslCall<'a> {
    expression: &'a ExprCall,
    operation: Operation,
    arguments: Vec<&'a Expr>,
}

impl<'a> DslCall<'a> {
    pub(super) fn operation(&self) -> Operation {
        self.operation
    }

    pub(super) fn arguments(&self) -> &[&'a Expr] {
        &self.arguments
    }

    pub(super) fn parse(expression: &'a ExprCall) -> syn::Result<Self> {
        let name = operation_name(expression)?;
        let Some(operation) = Operation::named(&name) else {
            return Err(syn::Error::new_spanned(
                &expression.func,
                diagnostic!(
                    UnknownOperation,
                    "unknown symbolic operation '{name}'; supported calls: empty, reverse, append, length, \
                     is_empty, contains, all, implies; runtime functions are not translated",
                ),
            ));
        };

        let (arity, signature) = operation.signature();

        if expression.args.len() != arity {
            return Err(syn::Error::new_spanned(
                expression,
                diagnostic!(
                    CallArity,
                    "{name} expects {arity} arguments, found {}; use {signature}",
                    expression.args.len()
                ),
            ));
        }

        Ok(Self {
            expression,
            operation,
            arguments: expression.args.iter().collect(),
        })
    }

    pub(super) fn expected_proposition(&self) -> syn::Error {
        let (_, signature) = self.operation.signature();

        syn::Error::new_spanned(
            self.expression,
            diagnostic!(
                ExpectedProposition,
                "{signature} produces a value, but a proposition is required here; compare it with == or !=",
            ),
        )
    }
}

fn operation_name(call: &ExprCall) -> syn::Result<String> {
    let Expr::Path(path) = call.func.as_ref() else {
        return Err(syn::Error::new_spanned(
            &call.func,
            diagnostic!(
                UnknownOperation,
                "expected a named DSL operation such as contains(xs, x); indirect function calls are not supported",
            ),
        ));
    };

    path.path
        .get_ident()
        .map(ToString::to_string)
        .ok_or_else(|| {
            syn::Error::new_spanned(
                &path.path,
                diagnostic!(
                    UnknownOperation,
                    "qualified runtime calls are not translated; use a bare DSL name such as reverse(xs)",
                ),
            )
        })
}
