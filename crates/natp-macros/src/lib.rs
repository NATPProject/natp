//! Restricted claim syntax lowered to NATP symbolic expressions.

mod lower;
mod syntax;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as Tokens;
use quote::quote;
use syn::parse_macro_input;
use syntax::ClaimInput;

/// Translate a universally quantified DSL proposition into a typed symbolic closure.
/// This constructs expressions; proof checking remains the backend's responsibility.
#[proc_macro]
pub fn claim(input: TokenStream) -> TokenStream {
    match expand(parse_macro_input!(input as ClaimInput)) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.into_compile_error().into(),
    }
}

fn expand(input: ClaimInput) -> syn::Result<Tokens> {
    let root = quote!(::natp);
    let body = lower::lower_proposition(&root, &input.variables, &input.body)?;
    let variables = input.variables;

    Ok(quote! {
        |#(#variables),*| {
            #(let _ = &#variables;)*
            #body
        }
    })
}
