use natp_diagnostics::diagnostic;
use proc_macro2::TokenStream as Tokens;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{BinOp, Expr, ExprBinary, Ident};

pub(super) fn lower_proposition(
    _root: &Tokens,
    variables: &[Ident],
    expression: &Expr,
) -> syn::Result<Tokens> {
    LoweringContext { variables }.lower_proposition(expression)
}

struct LoweringContext<'a> {
    variables: &'a [Ident],
}

impl LoweringContext<'_> {
    fn lower_proposition(&self, expression: &Expr) -> syn::Result<Tokens> {
        let expression = ungroup_expression(expression)?;

        match expression {
            Expr::Binary(binary) => self.lower_binary(binary),
            _ => Err(unsupported_expression(expression)),
        }
    }

    fn lower_binary(&self, binary: &ExprBinary) -> syn::Result<Tokens> {
        match binary.op {
            BinOp::Eq(_) => self.lower_equality(binary),
            _ => Err(syn::Error::new_spanned(
                binary.op,
                diagnostic!(
                    UnsupportedOperator,
                    "unsupported proposition operator; use ==",
                ),
            )),
        }
    }

    fn lower_equality(&self, binary: &ExprBinary) -> syn::Result<Tokens> {
        let receiver = self.lower_term(&binary.left)?;
        let argument = self.lower_term(&binary.right)?;
        Ok(quote_spanned!(binary.span()=> (#receiver).equals(#argument)))
    }

    fn lower_term(&self, expression: &Expr) -> syn::Result<Tokens> {
        let expression = ungroup_expression(expression)?;

        match expression {
            Expr::Path(path) if self.variables.iter().any(|name| path.path.is_ident(name)) => {
                Ok(quote!(#path.clone()))
            }
            Expr::Path(path) => {
                let available = self
                    .variables
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");

                Err(syn::Error::new_spanned(
                    path,
                    diagnostic!(
                        UnknownSymbol,
                        "unknown symbolic variable; available variables: {available}; add a quantifier and \
                         matching closure parameter",
                    ),
                ))
            }
            Expr::Binary(_) | Expr::Unary(_) => self.lower_proposition(expression),
            _ => Err(unsupported_expression(expression)),
        }
    }
}

fn ungroup_expression(expression: &Expr) -> syn::Result<&Expr> {
    match expression {
        Expr::Paren(parentheses) => ungroup_expression(&parentheses.expr),
        _ => Ok(expression),
    }
}

fn unsupported_expression(expression: &Expr) -> syn::Error {
    let help = match expression {
        Expr::Index(_) => "indexing is not supported; use sequence equality",
        Expr::MethodCall(_) => {
            "method calls are not supported inside claim!; for Rust symbolic methods, pass a \
            normal closure to .claim(...) without claim!"
        }
        Expr::If(_) | Expr::Match(_) => {
            "runtime control flow is not supported; express the claim with =="
        }
        Expr::Lit(_) => "expected a proposition, not a value; compare values with ==",
        _ => "compare quantified variables with ==",
    };

    syn::Error::new_spanned(
        expression,
        diagnostic!(
            UnsupportedExpression,
            "unsupported symbolic expression; {help}"
        ),
    )
}
