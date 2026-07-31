//! Lower right-associative => before parsing ordinary Rust expressions.
//! Groups and argument boundaries are respected; implication has lowest precedence.

use proc_macro2::{Group, Ident, TokenStream, TokenTree};
use quote::quote;

use crate::lower::Operation;

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
    if items.iter().any(|token| punctuation(token, ',')) {
        return rewrite_arguments(items);
    }

    // An all-predicate may use an unbraced implication as its closure body.
    if items.first().is_some_and(|token| punctuation(token, '|')) {
        if let Some(end) = items
            .iter()
            .enumerate()
            .skip(1)
            .find_map(|(index, token)| punctuation(token, '|').then_some(index))
        {
            let mut prefix: TokenStream = items[..=end].iter().cloned().collect();
            prefix.extend(rewrite_level(&items[end + 1..]));

            return prefix;
        }
    }

    if let Some(index) = items
        .windows(2)
        .position(|pair| punctuation(&pair[0], '=') && punctuation(&pair[1], '>'))
    {
        let left: TokenStream = items[..index].iter().cloned().collect();
        let right = rewrite_level(&items[index + 2..]);

        let operation = Ident::new(Operation::IMPLICATION_REWRITE, items[index].span());

        return quote!(#operation(#left, #right));
    }

    items.iter().cloned().collect()
}

fn rewrite_arguments(items: &[TokenTree]) -> TokenStream {
    let mut output = TokenStream::new();
    let mut start = 0;

    for (index, token) in items.iter().enumerate() {
        if punctuation(token, ',') {
            output.extend(rewrite_level(&items[start..index]));
            output.extend([token.clone()]);
            start = index + 1;
        }
    }

    output.extend(rewrite_level(&items[start..]));

    output
}

fn punctuation(token: &TokenTree, expected: char) -> bool {
    matches!(token, TokenTree::Punct(punctuation) if punctuation.as_char() == expected)
}
