/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use syn::parse::Parse;

use crate::bindgen::ir::ty::{IntKind, PrimitiveType};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum ReprStyle {
    #[default]
    Rust,
    C,
    Transparent,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ReprType {
    kind: IntKind,
    signed: bool,
}

impl ReprType {
    pub(crate) fn to_primitive(self) -> PrimitiveType {
        PrimitiveType::Integer {
            kind: self.kind,
            signed: self.signed,
            zeroable: true,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ReprAlign {
    Packed,
    Align(u64),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Repr {
    pub style: ReprStyle,
    pub ty: Option<ReprType>,
    pub align: Option<ReprAlign>,
}

impl Repr {
    pub fn load(attrs: &[syn::Attribute]) -> Result<Repr, String> {
        let mut ids = Vec::new();

        // We want only the `repr` attributes
        let iter = attrs
            .iter()
            .filter(|attr| attr.path().is_ident("repr"))
            .filter_map(|attr| {
                // repr must be a meta list
                if let syn::Meta::List(meta_list) = &attr.meta {
                    Some(meta_list)
                } else {
                    None
                }
            });

        for meta_list in iter {
            meta_list
                .parse_nested_meta(|meta| {
                    match meta
                        .path
                        .get_ident()
                        .map(|ident| ident.to_string())
                        .as_deref()
                    {
                        Some(
                            int_kind @ ("u8" | "u16" | "u32" | "u64" | "usize" | "i8" | "i16"
                            | "i32" | "i64" | "isize"),
                        ) => ids.push((int_kind.to_string(), None)),
                        Some(repr @ ("C" | "transparent")) => ids.push((repr.to_owned(), None)),
                        Some(repr @ "align") => {
                            let content;
                            syn::parenthesized!(content in meta.input);
                            let lit: syn::LitInt = content.parse()?;
                            ids.push((repr.to_string(), Some(lit.base10_digits().to_string())));
                        }
                        Some(repr @ "packed") => {
                            if meta.input.is_empty() {
                                ids.push((repr.to_string(), None));
                            } else {
                                let content;
                                syn::parenthesized!(content in meta.input);
                                let lit: syn::LitInt = content.parse()?;
                                ids.push((repr.to_string(), Some(lit.base10_digits().to_string())));
                            }
                        }
                        Some(repr) => {
                            if meta.input.is_empty() {
                                ids.push((repr.to_string(), None));
                            } else {
                                let content;
                                syn::parenthesized!(content in meta.input);

                                let args: Vec<_> = content
                                    .parse_terminated(
                                        proc_macro2::TokenStream::parse,
                                        syn::Token![,],
                                    )?
                                    .into_iter()
                                    .map(|arg| arg.to_string())
                                    .collect();
                                ids.push((repr.to_string(), Some(args.join(","))))
                            }
                        }
                        None => return Err(meta.error("not a identifier")),
                    }

                    Ok(())
                })
                .map_err(|err| format!("{err}"))?;
        }

        let mut repr = Repr::default();
        for id in ids {
            let (int_kind, signed) = match (id.0.as_ref(), id.1) {
                ("u8", None) => (IntKind::B8, false),
                ("u16", None) => (IntKind::B16, false),
                ("u32", None) => (IntKind::B32, false),
                ("u64", None) => (IntKind::B64, false),
                ("usize", None) => (IntKind::Size, false),
                ("i8", None) => (IntKind::B8, true),
                ("i16", None) => (IntKind::B16, true),
                ("i32", None) => (IntKind::B32, true),
                ("i64", None) => (IntKind::B64, true),
                ("isize", None) => (IntKind::Size, true),
                ("C", None) => {
                    repr.style = ReprStyle::C;
                    continue;
                }
                ("transparent", None) => {
                    repr.style = ReprStyle::Transparent;
                    continue;
                }
                ("packed", args) => {
                    // #[repr(packed(n))] not supported because of some open questions about how
                    // to calculate the native alignment of types. See mozilla/cbindgen#433.
                    if args.is_some() {
                        return Err(
                            "Not-yet-implemented #[repr(packed(...))] encountered.".to_string()
                        );
                    }
                    let align = ReprAlign::Packed;
                    // Only permit a single alignment-setting repr.
                    if let Some(old_align) = repr.align {
                        return Err(format!(
                            "Conflicting #[repr(align(...))] type hints {:?} and {:?}.",
                            old_align, align
                        ));
                    }
                    repr.align = Some(align);
                    continue;
                }
                ("align", Some(arg)) => {
                    // Must be a positive integer.
                    let align = match arg.parse::<u64>() {
                        Ok(align) => align,
                        Err(_) => return Err(format!("Non-unsigned #[repr(align({}))].", arg)),
                    };
                    // Must be a power of 2.
                    if !align.is_power_of_two() || align == 0 {
                        return Err(format!("Invalid alignment to #[repr(align({}))].", align));
                    }
                    // Only permit a single alignment-setting repr.
                    if let Some(old_align) = repr.align {
                        return Err(format!(
                            "Conflicting #[repr(align(...))] type hints {:?} and {:?}.",
                            old_align,
                            ReprAlign::Align(align)
                        ));
                    }
                    repr.align = Some(ReprAlign::Align(align));
                    continue;
                }
                (path, arg) => match arg {
                    None => return Err(format!("Unsupported #[repr({})].", path)),
                    Some(arg) => {
                        return Err(format!("Unsupported #[repr({}({}))].", path, arg));
                    }
                },
            };
            let ty = ReprType {
                kind: int_kind,
                signed,
            };
            if let Some(old_ty) = repr.ty {
                return Err(format!(
                    "Conflicting #[repr(...)] type hints {:?} and {:?}.",
                    old_ty, ty
                ));
            }
            repr.ty = Some(ty);
        }
        Ok(repr)
    }
}
