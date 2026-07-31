//! Restricted claim syntax lowered to NATP symbolic expressions.

mod implication;
mod lower;
mod syntax;

use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::{Span, TokenStream as Tokens};
use quote::quote;
use syn::{Error, Ident, Result, parse_macro_input};

use syntax::ClaimInput;

#[derive(Clone, Copy)]
enum PackageName {
    Natp,
}

impl PackageName {
    fn as_str(self) -> &'static str {
        match self {
            Self::Natp => "natp",
        }
    }
}

/// Translate a universally quantified DSL proposition into a typed symbolic closure.
#[proc_macro]
pub fn claim(input: TokenStream) -> TokenStream {
    match expand(parse_macro_input!(input as ClaimInput)) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.into_compile_error().into(),
    }
}

fn expand(input: ClaimInput) -> Result<Tokens> {
    let root = natp_crate()?;
    let body = lower::lower_proposition(&root, &input.variables, &input.body)?;
    let variables = input.variables;

    Ok(quote! {
        |#(#variables),*| {
            #(let _ = &#variables;)*
            #body
        }
    })
}

fn natp_crate() -> Result<Tokens> {
    let name = crate_name(PackageName::Natp.as_str())
        .map_err(|error| Error::new(Span::call_site(), error.to_string()))?;
    let identifier = match name {
        FoundCrate::Itself => Ident::new(PackageName::Natp.as_str(), Span::call_site()),
        FoundCrate::Name(name) => Ident::new(&name, Span::call_site()),
    };

    Ok(quote!(::#identifier))
}
