use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use regex::Regex;
use serde::Deserialize;
use std::{
    collections::{BTreeSet, HashSet},
    env, fs,
};
use syn::Type;
use syn::{parse::Parse, parse::ParseStream, parse_macro_input, Expr, Ident, LitStr, Meta, Token};

#[derive(Deserialize)]
struct Codes {
    #[serde(default)]
    ascii: Option<serde_json::Value>,
    #[serde(default)]
    hex: Option<serde_json::Value>,
    #[serde(default)]
    dec: Option<serde_json::Value>,
}

#[derive(Deserialize)]
struct CommandSpec {
    id: String,
    name: String,
    category: String,
    #[serde(default)]
    to_string: Option<String>,
    #[serde(default)]
    message: Option<String>,
    #[serde(default)]
    detailed_message: Option<String>,
    #[serde(default)]
    notes: Vec<String>,
    #[serde(default)]
    obsolete: bool,
    #[serde(default)]
    content: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct RootSpec {
    categories: Vec<String>,
    commands: Vec<CommandSpec>,
}

struct MacroArgs {
    spec_expr: Option<Expr>,   // SPEC = include_str!(...)
    spec_path: Option<LitStr>, // SPEC_PATH = "spec/commands.json"
    enum_name: Ident,
    derive: Option<LitStr>,
    discr_derive: Option<LitStr>,

    // NEW:
    category_enum_name: Option<Ident>,
    category_derive: Option<LitStr>,
}

impl Parse for MacroArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut spec_expr = None;
        let mut spec_path = None;
        let mut enum_name = None;
        let mut derive = None;
        let mut discr_derive = None;

        // NEW:
        let mut category_enum_name = None;
        let mut category_derive = None;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            match &*key.to_string() {
                "SPEC" => { spec_expr = Some(input.parse()?); }
                "SPEC_PATH" => { spec_path = Some(input.parse()?); }
                "enum_name" => { enum_name = Some(input.parse()?); }
                "derive" => { derive = Some(input.parse()?); }
                "strum_discriminants_derive" => { discr_derive = Some(input.parse()?); }

                // NEW:
                "category_enum_name" => { category_enum_name = Some(input.parse()?); }
                "category_derive" => { category_derive = Some(input.parse()?); }

                _ => return Err(syn::Error::new_spanned(
                    key,
                    "Unknown key: use SPEC, SPEC_PATH, enum_name, derive, strum_discriminants_derive, category_enum_name, category_derive"
                )),
            }
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        if spec_expr.is_none() && spec_path.is_none() {
            return Err(syn::Error::new(
                Span::call_site(),
                "Provide SPEC = include_str!(...) or SPEC_PATH = \"path/to/file.json\"",
            ));
        }

        Ok(MacroArgs {
            spec_expr,
            spec_path,
            enum_name: enum_name
                .ok_or_else(|| syn::Error::new(Span::call_site(), "enum_name is required"))?,
            derive,
            discr_derive,
            category_enum_name,
            category_derive,
        })
    }
}

fn variant_tokens_with_ident(
    c: &CommandSpec,
    variant_ident: &syn::Ident,
    vtypes: &Vec<syn::Type>,
) -> TokenStream2 {
    let message_val = c.message.as_deref().unwrap_or(&c.name);
    let detailed_val = c.detailed_message.as_deref().unwrap_or(message_val);
    let to_string_default = pascalize(&c.name);
    let to_string_val = c.to_string.as_deref().unwrap_or(&to_string_default);

    let message_lit = LitStr::new(message_val, Span::call_site());
    let detailed_lit = LitStr::new(detailed_val, Span::call_site());
    let to_string_lit = LitStr::new(to_string_val, Span::call_site());
    let id_lit = LitStr::new(&c.id, Span::call_site());
    let cat_prop = LitStr::new(&c.category, Span::call_site());

    let mut props_list = Vec::<TokenStream2>::new();
    props_list.push(quote!( id = #id_lit ));
    props_list.push(quote!( category = #cat_prop ));

    let notes_docs = c.notes.iter().map(|n| {
        let s = LitStr::new(&format!(" - {}", n), Span::call_site());
        quote!( #[doc = #s] )
    });

    let obsolete_prop = if c.obsolete {
        Some(quote!(obsolete = "true",))
    } else {
        None
    };

    if vtypes.is_empty() {
        // Unit variant
        quote! {
            #[strum(message = #message_lit)]
            #[strum(detailed_message = #detailed_lit)]
            #[strum(to_string = #to_string_lit)]
            #[strum(props( #obsolete_prop #(#props_list),* ))]
            #(#notes_docs)*
            #variant_ident,
        }
    } else {
        // Tuple variant with content types
        quote! {
            #[strum(message = #message_lit)]
            #[strum(detailed_message = #detailed_lit)]
            #[strum(to_string = #to_string_lit)]
            #[strum(props( #obsolete_prop #(#props_list),* ))]
            #(#notes_docs)*
            #variant_ident( #( #vtypes ),* ),
        }
    }
}

#[proc_macro]
pub fn escpos_commands(input: TokenStream) -> TokenStream {
    let MacroArgs {
        spec_expr,
        spec_path,
        enum_name,
        derive,
        discr_derive,
        category_enum_name,
        category_derive,
    } = parse_macro_input!(input as MacroArgs);

    // Load JSON either from inline string (SPEC) or from file (SPEC_PATH)
    let spec_string = if let Some(p) = spec_path {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
        let full = std::path::Path::new(&manifest_dir).join(p.value());
        match fs::read_to_string(&full) {
            Ok(s) => s,
            Err(e) => {
                return syn::Error::new(
                    p.span(),
                    format!("Failed to read {}: {}", full.display(), e),
                )
                .to_compile_error()
                .into();
            }
        }
    } else {
        let expr = spec_expr.unwrap();
        match literal_string_from_expr(&expr) {
            Some(s) => s,
            None => {
                return syn::Error::new_spanned(
                    &expr,
                    "SPEC must be a literal string (e.g., include_str!(\"...\")) — identifiers like `ESC_SPEC` are not expanded here. Use SPEC_PATH instead."
                )
                .to_compile_error()
                .into();
            }
        }
    };

    let parsed: RootSpec = match serde_json::from_str(&spec_string) {
        Ok(v) => v,
        Err(e) => {
            return syn::Error::new(
                Span::call_site(),
                format!("Failed to parse commands JSON: {e}"),
            )
            .to_compile_error()
            .into()
        }
    };

    // Validate categories
    let mut cat_set = BTreeSet::new();
    for c in &parsed.categories {
        if !cat_set.insert(c.clone()) {
            return syn::Error::new(
                Span::call_site(),
                format!("Duplicate category in spec: {c}"),
            )
            .to_compile_error()
            .into();
        }
    }
    for cmd in &parsed.commands {
        if !cat_set.contains(&cmd.category) {
            return syn::Error::new(
                Span::call_site(),
                format!(
                    "Command '{}' has unknown category '{}'",
                    cmd.name, cmd.category
                ),
            )
            .to_compile_error()
            .into();
        }
    }

    // ---------- Generate category enum ----------
    let category_enum_ident =
        category_enum_name.unwrap_or_else(|| Ident::new("CommandCategory", Span::call_site()));
    let category_variants = parsed.categories.iter().map(|c| {
        let var_ident = make_ident(c);

        let msg_lit = LitStr::new(c, Span::call_site());
        quote! {
            #[strum(message = #msg_lit)]
            #var_ident,
        }
    });

    // Derives for the category enum (overridable)
    let cat_derive_attr: TokenStream2 = if let Some(d) = category_derive {
        let meta: Meta =
            syn::parse_str(&format!("derive({})", d.value())).expect("invalid category_derive");
        quote!( #[#meta] )
    } else {
        // default
        let meta: Meta =
            syn::parse_str("derive(strum::EnumCount, strum::EnumIter, strum::EnumMessage)")
                .unwrap();
        quote!( #[#meta] )
    };

    let cat_enum = quote! {
        #cat_derive_attr
        pub enum #category_enum_ident {
            #(#category_variants)*
        }
    };

    // ---------- Generate Command enum ----------
    let derive_attr: Option<TokenStream2> = derive.as_ref().map(|d| {
        let meta: Meta =
            syn::parse_str(&format!("derive({})", d.value())).expect("invalid derive attribute");
        quote!( #[#meta] )
    });
    let discr_attr: Option<TokenStream2> = discr_derive.as_ref().map(|d| {
        let meta: Meta = syn::parse_str(&format!("strum_discriminants(derive({}))", d.value()))
            .expect("invalid strum_discriminants derive");
        quote!( #[#meta] )
    });

    let mut used_variants: HashSet<String> = HashSet::new();
    let mut variant_idents: Vec<syn::Ident> = Vec::with_capacity(parsed.commands.len());
    let mut category_idents: Vec<syn::Ident> = Vec::with_capacity(parsed.commands.len());
    let mut field_types: Vec<Vec<Type>> = Vec::with_capacity(parsed.commands.len()); // NEW

    for c in &parsed.commands {
        let base_pretty = c.to_string.as_deref().unwrap_or(&c.name);
        let vident = uniquify_variant_name(base_pretty, &c.id, &mut used_variants);
        variant_idents.push(vident);

        let cat_ident = make_ident(&c.category);
        category_idents.push(cat_ident);

        // Parse content strings into syn::Type, with nice errors
        let mut vtypes: Vec<Type> = Vec::new();
        if let Some(items) = &c.content {
            for (idx, tstr) in items.iter().enumerate() {
                match syn::parse_str::<Type>(tstr) {
                    Ok(ty) => vtypes.push(ty),
                    Err(e) => {
                        // Point to the bad content with a clear message
                        let msg = format!("Invalid content type for id '{}', index {}: {}\n  hint: use valid Rust types like `u8`, `Option<u16>`, `Vec<u8>`, etc.", c.id, idx, e);
                        return syn::Error::new(Span::call_site(), msg)
                            .to_compile_error()
                            .into();
                    }
                }
            }
        }
        field_types.push(vtypes);
    }

    let variants = parsed
        .commands
        .iter()
        .enumerate()
        .map(|(i, c)| variant_tokens_with_ident(c, &variant_idents[i], &field_types[i]));

    let cmd_enum = quote! {
        #[repr(C)]
        #derive_attr
        #discr_attr
        pub enum #enum_name {
            #(#variants)*
        }
    };

    // ---------- Generate Command::category() ----------
    let category_arms = parsed.commands.iter().enumerate().map(|(i, _c)| {
        let v = &variant_idents[i];
        let cat = &category_idents[i];
        if field_types[i].is_empty() {
            quote!( Self::#v => #category_enum_ident::#cat, )
        } else {
            quote!( Self::#v(..) => #category_enum_ident::#cat, )
        }
    });

    let category_impl = quote! {
        impl #enum_name {
            #[inline]
            pub const fn category(&self) -> #category_enum_ident {
                match self {
                    #(#category_arms)*
                }
            }
        }
    };

    // Stitch together
    quote!( #cat_enum #cmd_enum #category_impl ).into()
}

// ---------- helpers ----------
fn variant_tokens(c: &CommandSpec) -> TokenStream2 {
    let variant_ident = make_ident(c.to_string.as_deref().unwrap_or(&c.name));

    let message_val = c.message.as_deref().unwrap_or(&c.name);
    let detailed_val = c.detailed_message.as_deref().unwrap_or(message_val);

    let to_string_default = pascalize(&c.name);
    let to_string_val = c.to_string.as_deref().unwrap_or(&to_string_default);

    let message_lit = LitStr::new(message_val, Span::call_site());
    let detailed_lit = LitStr::new(detailed_val, Span::call_site());
    let to_string_lit = LitStr::new(to_string_val, Span::call_site());

    let mut props_list = props_kv_tokens(c);
    // also surface category as a strum prop
    props_list.push(quote_kv_prop_ident_val("category", &c.category));

    let notes_docs = c.notes.iter().map(|n| {
        let s = LitStr::new(&format!(" - {}", n), Span::call_site());
        quote!( #[doc = #s] )
    });

    let obsolete_prop = if c.obsolete {
        Some(quote!(obsolete = "true",))
    } else {
        None
    };

    quote! {
        #[strum(message = #message_lit)]
        #[strum(detailed_message = #detailed_lit)]
        #[strum(to_string = #to_string_lit)]
        #[strum(props( #obsolete_prop #(#props_list),* ))]
        #(#notes_docs)*
        #variant_ident,
    }
}

fn props_kv_tokens(c: &CommandSpec) -> Vec<TokenStream2> {
    let mut props = Vec::new();
    // if let Some(codes) = &c.codes {
    //     if let Some(v) = &codes.ascii {
    //         props.push(quote_kv_prop("ascii", v));
    //     }
    //     if let Some(v) = &codes.hex {
    //         props.push(quote_kv_prop("hex", v));
    //     }
    //     if let Some(v) = &codes.dec {
    //         props.push(quote_kv_prop("decimal", v));
    //     }
    // }
    props
}

fn quote_kv_prop(key: &str, v: &serde_json::Value) -> TokenStream2 {
    let key_ident = syn::Ident::new(key, Span::call_site());
    let s = match v {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Array(arr) => arr
            .iter()
            .filter_map(|x| x.as_str().map(str::to_string))
            .collect::<Vec<_>>()
            .join(" "),
        _ => format!("{v}"),
    };
    let lit = LitStr::new(&s, Span::call_site());
    quote!( #key_ident = #lit )
}

fn quote_kv_prop_ident_val(key: &str, val: &str) -> TokenStream2 {
    let key_ident = syn::Ident::new(key, Span::call_site());
    let lit = LitStr::new(val, Span::call_site());
    quote!( #key_ident = #lit )
}

fn literal_string_from_expr(expr: &syn::Expr) -> Option<String> {
    match expr {
        syn::Expr::Lit(l) => match &l.lit {
            syn::Lit::Str(s) => Some(s.value()),
            _ => None,
        },
        _ => {
            let ts = quote!(#expr);
            syn::parse2::<LitStr>(ts).ok().map(|ls| ls.value())
        }
    }
}

/// Very plain PascalCase from arbitrary text.
fn pascalize(s: &str) -> String {
    let mut out = String::new();
    for w in s.split(|c: char| !c.is_alphanumeric()) {
        if w.is_empty() {
            continue;
        }
        let mut it = w.chars();
        if let Some(f) = it.next() {
            out.push(f.to_ascii_uppercase());
            for r in it {
                out.push(r.to_ascii_lowercase());
            }
        }
    }
    out
}

fn is_ident_start(c: char) -> bool {
    c == '_' || c.is_ascii_alphabetic()
}

fn is_keyword(s: &str) -> bool {
    // Keep it simple; add more if you hit them.
    matches!(
        s,
        "as"|"break"|"const"|"continue"|"crate"|"else"|"enum"|"extern"|"false"|"fn"|
        "for"|"if"|"impl"|"in"|"let"|"loop"|"match"|"mod"|"move"|"mut"|"pub"|
        "ref"|"return"|"self"|"Self"|"static"|"struct"|"super"|"trait"|"true"|
        "type"|"unsafe"|"use"|"where"|"while"|
        // 2018+ reserved:
        "async"|"await"|"dyn"|"try"
    )
}

fn make_ident(raw: &str) -> syn::Ident {
    let mut name = pascalize(raw);
    if name.is_empty() {
        name.push_str("Cmd");
    }
    if !is_ident_start(name.chars().next().unwrap()) {
        name.insert(0, 'N');
    }
    if is_keyword(&name) {
        return syn::Ident::new_raw(&name, Span::call_site());
    }
    syn::Ident::new(&name, Span::call_site())
}

fn extract_fn_suffix_from_id(id: &str) -> Option<String> {
    // match "..._fn60" or "..._FN165" etc.
    static RE: once_cell::sync::Lazy<Regex> =
        once_cell::sync::Lazy::new(|| Regex::new(r"(?i)_fn(\d+)").unwrap());
    RE.captures(id).map(|c| format!("Fn{}", &c[1]))
}

fn compact_id_suffix(id: &str) -> String {
    // keep last 2 meaningful tokens from id as PascalCase, e.g.
    // "fs_lparen_cc_fn60" -> "CcFn60"
    let toks: Vec<_> = id
        .split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
        .filter(|t| !t.is_empty())
        .collect();
    let tail = toks.iter().rev().take(2).cloned().collect::<Vec<_>>();
    let mut parts = tail.into_iter().rev().collect::<Vec<_>>().join(" ");
    if parts.is_empty() {
        parts = id.to_string();
    }
    pascalize(&parts)
}

fn uniquify_variant_name(base_pretty: &str, id: &str, used: &mut HashSet<String>) -> syn::Ident {
    let final_name = base_pretty.is_empty().then_some(id).unwrap_or(base_pretty);

    let try_insert = |s: &str, used: &mut HashSet<String>| -> Option<String> {
        let mut s2 = s.to_string();
        if !is_ident_start(s2.chars().next().unwrap_or('_')) {
            s2.insert(0, 'N');
        }
        if is_keyword(&s2) {
            s2.insert(0, 'N');
        }
        if used.insert(s2.clone()) {
            Some(s2)
        } else {
            None
        }
    };

    if let Some(ok) = try_insert(&final_name, used) {
        return syn::Ident::new(&ok, Span::call_site());
    }

    // collision: add FnNN if we have it
    if let Some(sfx) = extract_fn_suffix_from_id(id) {
        if let Some(ok) = try_insert(&format!("{}{}", final_name, sfx), used) {
            return syn::Ident::new(&ok, Span::call_site());
        }
    }

    // still collision: add compact id suffix
    let cid = compact_id_suffix(id);
    if let Some(ok) = try_insert(&format!("{}{}", final_name, cid), used) {
        return syn::Ident::new(&ok, Span::call_site());
    }

    // last resort: numeric disambiguator
    let mut n = 2usize;
    loop {
        if let Some(ok) = try_insert(&format!("{}_{n}", final_name), used) {
            return syn::Ident::new(&ok, Span::call_site());
        }
        n += 1;
    }
}
