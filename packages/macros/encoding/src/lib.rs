use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use std::collections::BTreeSet;
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Expr, ExprLit, Ident, Lit, Result, Token, Visibility,
};

struct TableInput {
    vis: Option<Visibility>, // allow optional `pub`, `pub(crate)` etc.
    ident: Ident,
    _eq: Token![=],
    chars: Vec<char>,
}

impl Parse for TableInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let vis: Option<Visibility> = if input.peek(Token![pub]) {
            Some(input.parse()?)
        } else {
            None
        };

        let ident: Ident = input.parse()?;
        let _eq: Token![=] = input.parse()?;

        let content;
        let _bracket = bracketed!(content in input);

        let elems: Punctuated<Expr, Token![,]> =
            content.parse_terminated(Expr::parse, Token![,])?;
        let mut chars = Vec::with_capacity(elems.len());

        for expr in elems {
            match expr {
                Expr::Lit(ExprLit {
                    lit: Lit::Char(ch), ..
                }) => {
                    chars.push(ch.value());
                }
                // Allow string literals of length 1 (nicer ergonomics if people slip)
                Expr::Lit(ExprLit {
                    lit: Lit::Str(s), ..
                }) => {
                    let s = s.value();
                    let mut iter = s.chars();
                    let first = iter.next();
                    if first.is_some() && iter.next().is_none() {
                        chars.push(first.unwrap());
                    } else {
                        return Err(syn::Error::new_spanned(
                            Lit::Str(syn::LitStr::new(&s, Span::call_site())),
                            "expected char literal or 1-char string",
                        ));
                    }
                }
                other => {
                    return Err(syn::Error::new_spanned(
                        other,
                        "expected a char literal like 'é' or '\\u{00A0}' (or a 1-char string)",
                    ));
                }
            }
        }

        Ok(TableInput {
            vis,
            ident,
            _eq,
            chars,
        })
    }
}

#[proc_macro]
pub fn char_table(input: TokenStream) -> TokenStream {
    let TableInput {
        vis, ident, chars, ..
    } = match syn::parse::<TableInput>(input) {
        Ok(x) => x,
        Err(e) => return e.into_compile_error().into(),
    };

    let n = chars.len();
    if n == 0 {
        return syn::Error::new(Span::call_site(), "table must not be empty")
            .into_compile_error()
            .into();
    }
    if n > 256 {
        return syn::Error::new(
            Span::call_site(),
            format!("table has {n} entries; max supported is 256 so indices fit in u8"),
        )
        .into_compile_error()
        .into();
    }

    // Check duplicates (by scalar value)
    let mut seen = BTreeSet::new();
    for &c in &chars {
        if !seen.insert(c) {
            return syn::Error::new(
                Span::call_site(),
                format!("duplicate character in table: {c:?}"),
            )
            .into_compile_error()
            .into();
        }
    }

    // Build forward [char; N]
    let forward_elems = chars.iter().map(|&c| quote!(#c));

    // Build reverse &[(char, u8); N], sorted by char
    let mut pairs: Vec<(char, u8)> = chars
        .iter()
        .enumerate()
        .map(|(i, &c)| (c, i as u8))
        .collect();
    pairs.sort_by_key(|(c, _)| *c);

    let rev_pairs = pairs.iter().map(|(c, i)| quote!((#c, #i)));

    // Names: X_TABLE and X_INV_TABLE
    let table_name = format_ident!("{}_TABLE", ident);
    let inv_name = format_ident!("{}_INV_TABLE", ident);

    // Pass through visibility if present; else default to private (module-scoped)
    let vis_tokens = vis.map(|v| quote!(#v)).unwrap_or_default();

    let expanded = quote! {
        #vis_tokens static #table_name: [char; #n] = [ #(#forward_elems),* ];

        // Sorted by char; good for binary_search
        #vis_tokens static #inv_name: &[(char, u8); #n] = &[
            #(#rev_pairs),*
        ];
    };

    expanded.into()
}
