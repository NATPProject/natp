//! Lower right-associative => before parsing ordinary Rust expressions.
//! Groups and argument boundaries are respected; implication has lowest precedence.

use crate::lower::Operation;
use proc_macro2::{Group, TokenStream, TokenTree};
use quote::quote;

pub(super) fn rewrite(tokens: TokenStream) -> TokenStream {
    let items: Vec<_> = tokens
        .into_iter()
        .map(|token| match token {
            TokenTree::Group(group) => {
                let mut rewritten = Group::new(group.delimiter(), rewrite(group.stream()));
                rewritten.set_span(group.span());

                TokenTree::Group(rewritten)
            }
            token => token,
        })
        .collect();

    rewrite_level(&items)
}

/// Nested groups have already been rewritten, so revisiting an argument or
/// implication tail does not repeatedly walk the same group tree.
fn rewrite_level(items: &[TokenTree]) -> TokenStream {
    if items.iter().any(|token| punctuation(token, ','))
        || items.first().is_some_and(|token| punctuation(token, '|'))
    {
        return items.iter().cloned().collect();
    }
    if let Some(index) = items
        .windows(2)
        .position(|pair| punctuation(&pair[0], '=') && punctuation(&pair[1], '>'))
    {
        let left: TokenStream = items[..index].iter().cloned().collect();
        let right = rewrite_level(&items[index + 2..]);

        let operation =
            proc_macro2::Ident::new(Operation::IMPLICATION_REWRITE, items[index].span());

        return quote!(#operation(#left, #right));
    }

    items.iter().cloned().collect()
}

fn punctuation(token: &TokenTree, expected: char) -> bool {
    matches!(token, TokenTree::Punct(punctuation) if punctuation.as_char() == expected)
}
