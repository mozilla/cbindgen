/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use proc_macro2::TokenStream;
use std::collections::HashSet;
use syn::fold::Fold;
use syn::parse::{Parse, ParseStream, Parser, Result as ParseResult};

// $(#[$outer:meta])*
// ($($vis:tt)*) $BitFlags:ident: $T:ty {
//     $(
//         $(#[$inner:ident $($args:tt)*])*
//         const $Flag:ident = $value:expr;
//     )+
// }
#[derive(Debug)]
pub struct Bitflags {
    attrs: Vec<syn::Attribute>,
    vis: syn::Visibility,
    #[allow(dead_code)]
    struct_token: Token![struct],
    name: syn::Ident,
    #[allow(dead_code)]
    colon_token: Token![:],
    repr: syn::Type,
    flags: Flags,
}

impl Bitflags {
    pub fn expand(&self) -> (syn::ItemStruct, syn::ItemImpl) {
        let Bitflags {
            ref attrs,
            ref vis,
            ref name,
            ref repr,
            ref flags,
            ..
        } = *self;

        let struct_ = parse_quote! {
            /// cbindgen:internal-derive-bitflags=true
            #(#attrs)*
            #vis struct #name {
                bits: #repr,
            }
        };

        let consts = flags.expand(name, repr);
        let impl_ = parse_quote! {
            impl #name {
                #consts
            }
        };

        (struct_, impl_)
    }
}

impl Parse for Bitflags {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        Ok(Self {
            attrs: input.call(syn::Attribute::parse_outer)?,
            vis: input.parse()?,
            struct_token: input.parse()?,
            name: input.parse()?,
            colon_token: input.parse()?,
            repr: input.parse()?,
            flags: input.parse()?,
        })
    }
}

// $(#[$inner:ident $($args:tt)*])*
// const $Flag:ident = $value:expr;
#[derive(Debug)]
struct Flag {
    attrs: Vec<syn::Attribute>,
    #[allow(dead_code)]
    const_token: Token![const],
    name: syn::Ident,
    #[allow(dead_code)]
    equals_token: Token![=],
    value: syn::Expr,
    #[allow(dead_code)]
    semicolon_token: Token![;],
}

struct FlagValueFold<'a> {
    struct_name: &'a syn::Ident,
    flag_names: &'a HashSet<String>,
}

impl<'a> FlagValueFold<'a> {
    fn is_self(&self, ident: &syn::Ident) -> bool {
        ident == self.struct_name || ident == "Self"
    }
}

impl<'a> Fold for FlagValueFold<'a> {
    fn fold_expr(&mut self, node: syn::Expr) -> syn::Expr {
        // bitflags 2 doesn't expose `bits` publically anymore, and the documented way to
        // combine flags is using the `bits` method, e.g.
        // ```
        // bitflags! {
        //     struct Flags: u8 {
        //         const A = 1;
        //         const B = 1 << 1;
        //         const AB = Flags::A.bits() | Flags::B.bits();
        //     }
        // }
        // ```
        // As we're transforming the struct definition into `struct StructName { bits: T }`
        // as far as our bindings generation is concerned, `bits` is available as a field,
        // so by replacing `StructName::FLAG.bits()` with `StructName::FLAG.bits`, we make
        // e.g. `Flags::AB` available in the generated bindings.
        match node {
            syn::Expr::MethodCall(syn::ExprMethodCall {
                attrs,
                receiver,
                dot_token,
                method,
                args,
                ..
            }) if method == "bits"
                && args.is_empty()
                && matches!(&*receiver,
                syn::Expr::Path(syn::ExprPath { path, .. })
                    if path.segments.len() == 2
                        && self.is_self(&path.segments.first().unwrap().ident)
                        && self
                            .flag_names
                            .contains(&path.segments.last().unwrap().ident.to_string())) =>
            {
                return syn::Expr::Field(syn::ExprField {
                    attrs,
                    base: receiver,
                    dot_token,
                    member: syn::Member::Named(method),
                });
            }
            _ => {}
        }
        syn::fold::fold_expr(self, node)
    }
}

impl Flag {
    fn expand(
        &self,
        struct_name: &syn::Ident,
        repr: &syn::Type,
        flag_names: &HashSet<String>,
    ) -> TokenStream {
        let Flag {
            ref attrs,
            ref name,
            ref value,
            ..
        } = *self;
        let folded_value = FlagValueFold {
            struct_name,
            flag_names,
        }
        .fold_expr(value.clone());
        quote! {
            #(#attrs)*
            pub const #name : #struct_name = #struct_name { bits: (#folded_value) as #repr };
        }
    }
}

impl Parse for Flag {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        Ok(Self {
            attrs: input.call(syn::Attribute::parse_outer)?,
            const_token: input.parse()?,
            name: input.parse()?,
            equals_token: input.parse()?,
            value: input.parse()?,
            semicolon_token: input.parse()?,
        })
    }
}

#[derive(Debug)]
struct Flags(Vec<Flag>);

impl Parse for Flags {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let content;
        let _ = braced!(content in input);
        let mut flags = vec![];
        while !content.is_empty() {
            flags.push(content.parse()?);
        }
        Ok(Flags(flags))
    }
}

impl Flags {
    fn expand(&self, struct_name: &syn::Ident, repr: &syn::Type) -> TokenStream {
        let mut ts = quote! {};
        let flag_names = self
            .0
            .iter()
            .map(|flag| flag.name.to_string())
            .collect::<HashSet<_>>();
        for flag in &self.0 {
            ts.extend(flag.expand(struct_name, repr, &flag_names));
        }
        ts
    }
}

pub fn parse(tokens: TokenStream) -> ParseResult<Bitflags> {
    let parser = Bitflags::parse;
    parser.parse2(tokens)
}
