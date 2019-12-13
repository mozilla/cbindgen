/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use syn;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ReprStyle {
    Rust,
    C,
    Transparent,
}

impl Default for ReprStyle {
    fn default() -> Self {
        ReprStyle::Rust
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ReprType {
    U8,
    U16,
    U32,
    USize,
    I8,
    I16,
    I32,
    ISize,
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
        let ids = attrs
            .iter()
            .filter_map(|attr| {
                if let syn::Meta::List(syn::MetaList { path, nested, .. }) =
                    attr.parse_meta().ok()?
                {
                    if path.is_ident("repr") {
                        return Some(nested.into_iter().collect::<Vec<_>>());
                    }
                }
                None
            })
            .flat_map(|nested| nested)
            .filter_map(|meta| match meta {
                syn::NestedMeta::Meta(syn::Meta::Path(path)) => {
                    Some((path.segments.first().unwrap().ident.to_string(), None))
                }
                syn::NestedMeta::Meta(syn::Meta::List(syn::MetaList { path, nested, .. })) => {
                    Some((
                        path.segments.first().unwrap().ident.to_string(),
                        Some(
                            nested
                                .iter()
                                .filter_map(|meta| match meta {
                                    // Only used for #[repr(align(...))].
                                    syn::NestedMeta::Lit(syn::Lit::Int(literal)) => {
                                        Some(literal.base10_digits().to_string())
                                    }
                                    // Only single levels of nesting supported at the moment.
                                    _ => None,
                                })
                                .collect::<Vec<_>>(),
                        ),
                    ))
                }
                _ => None,
            });

        let mut repr = Repr::default();
        for id in ids {
            let new_ty = match (id.0.as_ref(), id.1) {
                ("u8", None) => ReprType::U8,
                ("u16", None) => ReprType::U16,
                ("u32", None) => ReprType::U32,
                ("usize", None) => ReprType::USize,
                ("i8", None) => ReprType::I8,
                ("i16", None) => ReprType::I16,
                ("i32", None) => ReprType::I32,
                ("isize", None) => ReprType::ISize,
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
                    // to calculate the native alignment of types. See eqrion/cbindgen#433.
                    if args.is_some() {
                        return Err(format!(
                            "Not-yet-implemented #[repr(packed(...))] encountered."
                        ));
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
                ("align", Some(args)) => {
                    // #[repr(align(...))] only allows a single argument.
                    if args.len() != 1 {
                        return Err(format!(
                            "Unsupported #[repr(align({}))], align must have exactly one argument.",
                            args.join(", ")
                        ));
                    }
                    // Must be a positive integer.
                    let align = match args.first().unwrap().parse::<u64>() {
                        Ok(align) => align,
                        Err(_) => {
                            return Err(format!("Non-numeric #[repr(align({}))].", args.join(", ")))
                        }
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
                (path, args) => match args {
                    None => return Err(format!("Unsupported #[repr({})].", path)),
                    Some(args) => {
                        return Err(format!(
                            "Unsupported #[repr({}({}))].",
                            path,
                            args.join(", ")
                        ));
                    }
                },
            };
            if let Some(old_ty) = repr.ty {
                return Err(format!(
                    "Conflicting #[repr(...)] type hints {:?} and {:?}.",
                    old_ty, new_ty
                ));
            }
            repr.ty = Some(new_ty);
        }
        Ok(repr)
    }
}
