use crate::implication;
use natp_diagnostics::diagnostic;
use syn::{
    Expr, Ident, Token,
    parse::{Parse, ParseStream},
};

// Mirrors the IR's quantifier limit without coupling this proc macro to a backend.
enum MacroLimit {
    Variables,
}

impl MacroLimit {
    const fn value(self) -> usize {
        match self {
            Self::Variables => 8,
        }
    }
}

pub(super) struct ClaimInput {
    pub(super) variables: Vec<Ident>,
    pub(super) body: Expr,
}

impl Parse for ClaimInput {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        input.parse::<Token![|]>().map_err(|_| {
            input.error(diagnostic!(
                QuantifierSyntax,
                "expected quantified closure parameters: claim!(|xs| predicate)"
            ))
        })?;
        let mut variables: Vec<Ident> = Vec::new();

        loop {
            let variable: Ident = input.parse().map_err(|_| {
                input.error(diagnostic!(
                    QuantifierSyntax,
                    "expected a plain variable name; use |xs, ys| without types, mut, or patterns"
                ))
            })?;

            if let Some(previous) = variables.iter().find(|previous| **previous == variable) {
                let mut error = syn::Error::new_spanned(
                    &variable,
                    diagnostic!(
                        DuplicateQuantifier,
                        "duplicate quantified variable '{variable}'; give each quantifier a distinct name",
                    ),
                );
                error.combine(syn::Error::new_spanned(
                    previous,
                    "first declaration is here",
                ));

                return Err(error);
            }

            variables.push(variable);

            if input.peek(Token![|]) {
                break;
            }

            input.parse::<Token![,]>().map_err(|_| {
                input.error(diagnostic!(
                    QuantifierSyntax,
                    "expected ',' between variables or '|' before the predicate"
                ))
            })?;
        }

        input.parse::<Token![|]>()?;

        if variables.len() > MacroLimit::Variables.value() {
            return Err(input.error(diagnostic!(
                MacroQuantifierLimit,
                "at most eight quantified variables are supported; found {}",
                variables.len()
            )));
        }

        let body = syn::parse2(implication::rewrite(input.parse()?)).map_err(|error| {
            syn::Error::new(
                error.span(),
                diagnostic!(
                    ClaimSyntax,
                    "malformed claim expression: {error}; expected a proposition after the closing '|', \
                     such as xs == xs",
                ),
            )
        })?;

        Ok(Self { variables, body })
    }
}
