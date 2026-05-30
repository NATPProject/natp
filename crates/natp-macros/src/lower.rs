use natp_diagnostics::diagnostic;
use proc_macro2::TokenStream as Tokens;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{BinOp, Expr, ExprBinary, Ident, Lit, Stmt, UnOp};

pub(super) fn lower_proposition(
    root: &Tokens,
    variables: &[Ident],
    expression: &Expr,
) -> syn::Result<Tokens> {
    LoweringContext { root, variables }.lower_proposition(expression)
}

struct LoweringContext<'a> {
    root: &'a Tokens,
    variables: &'a [Ident],
}

impl LoweringContext<'_> {
    fn lower_proposition(&self, expression: &Expr) -> syn::Result<Tokens> {
        let expression = ungroup_expression(expression)?;
        let root = self.root;

        match expression {
            Expr::Lit(literal) if matches!(literal.lit, Lit::Bool(_)) => {
                Ok(quote!(#root::Prop::Bool(#literal)))
            }
            Expr::Unary(unary) if matches!(unary.op, UnOp::Not(_)) => {
                let operand = self.lower_proposition(&unary.expr)?;

                Ok(quote!(!(#operand)))
            }
            Expr::Binary(binary) => self.lower_binary(binary),
            _ => Err(unsupported_expression(expression)),
        }
    }

    fn lower_binary(&self, binary: &ExprBinary) -> syn::Result<Tokens> {
        match binary.op {
            BinOp::Eq(_) | BinOp::Ne(_) => self.lower_equality(binary),
            BinOp::And(_) => {
                let left = self.lower_proposition(&binary.left)?;
                let right = self.lower_proposition(&binary.right)?;
                Ok(quote!((#left).and(#right)))
            }
            _ => Err(syn::Error::new_spanned(
                binary.op,
                diagnostic!(
                    UnsupportedOperator,
                    "unsupported proposition operator; use ==, !=, &&, or !",
                ),
            )),
        }
    }

    fn lower_equality(&self, binary: &ExprBinary) -> syn::Result<Tokens> {
        let root = self.root;
        let (left, right) = if is_literal(&binary.left) && !is_literal(&binary.right) {
            (&binary.right, &binary.left)
        } else {
            (&binary.left, &binary.right)
        };

        let receiver = self.lower_term(left)?;
        let argument = self.lower_term(right)?;

        let equality = if is_literal(left) {
            quote_spanned!(binary.span()=> #root::literal(#receiver).equals(#argument))
        } else {
            quote_spanned!(binary.span()=> (#receiver).equals(#argument))
        };

        if matches!(binary.op, BinOp::Ne(_)) {
            Ok(quote!(!(#equality)))
        } else {
            Ok(equality)
        }
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
                         matching closure parameter, or use an integer/Boolean literal",
                    ),
                ))
            }
            Expr::Lit(literal) if matches!(literal.lit, Lit::Int(_) | Lit::Bool(_)) => {
                Ok(quote!(#literal))
            }

            Expr::Unary(unary)
                if matches!(unary.op, UnOp::Neg(_))
                    && matches!(unary.expr.as_ref(), Expr::Lit(_)) =>
            {
                Ok(quote!(#unary))
            }
            Expr::Binary(_) | Expr::Unary(_) => self.lower_proposition(expression),
            _ => Err(unsupported_expression(expression)),
        }
    }
}

fn ungroup_expression(expression: &Expr) -> syn::Result<&Expr> {
    match expression {
        Expr::Paren(parentheses) => ungroup_expression(&parentheses.expr),
        Expr::Block(block) if block.block.stmts.len() == 1 => match &block.block.stmts[0] {
            Stmt::Expr(expression, None) => ungroup_expression(expression),
            _ => Err(syn::Error::new_spanned(
                expression,
                diagnostic!(
                    ClaimBlock,
                    "claim blocks must contain a single expression without a trailing semicolon; move let \
                     bindings and runtime statements outside claim!",
                ),
            )),
        },
        Expr::Block(_) => Err(syn::Error::new_spanned(
            expression,
            diagnostic!(
                ClaimBlock,
                "claim blocks must contain exactly one expression; remove statements and the trailing semicolon",
            ),
        )),
        _ => Ok(expression),
    }
}

fn is_literal(expression: &Expr) -> bool {
    matches!(expression, Expr::Lit(_))
        || matches!(expression, Expr::Unary(unary) if matches!(unary.op, UnOp::Neg(_)))
}

fn unsupported_expression(expression: &Expr) -> syn::Error {
    let help = match expression {
        Expr::Index(_) => "indexing is not supported; use sequence equality",
        Expr::MethodCall(_) => {
            "method calls are not supported inside claim!; for Rust symbolic methods, pass a \
            normal closure to .claim(...) without claim!"
        }
        Expr::If(_) | Expr::Match(_) => {
            "runtime control flow is not supported; combine propositions with && or !"
        }
        Expr::Lit(_) => "expected a proposition, not a value; compare values with == or !=",
        _ => "use supported symbolic variables, literals, and sequence operations",
    };

    syn::Error::new_spanned(
        expression,
        diagnostic!(
            UnsupportedExpression,
            "unsupported symbolic expression; {help}"
        ),
    )
}
