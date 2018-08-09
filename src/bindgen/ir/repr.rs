/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use syn;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ReprStyle {
    Rust,
    C,
}

impl Default for ReprStyle {
    fn default() -> Self {
        ReprStyle::Rust
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
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

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Repr {
    pub style: ReprStyle,
    pub ty: Option<ReprType>,
}

impl Repr {
    pub const C: Self = Repr {
        style: ReprStyle::C,
        ty: None,
    };

    pub const RUST: Self = Repr {
        style: ReprStyle::Rust,
        ty: None,
    };

    pub fn load(attrs: &[syn::Attribute]) -> Result<Repr, String> {
        let ids = attrs
            .iter()
            .filter_map(|attr| {
                if attr.is_sugared_doc || attr.style != syn::AttrStyle::Outer {
                    return None;
                }

                if let Some(syn::Meta::List(syn::MetaList { ident, nested, .. })) =
                    attr.interpret_meta()
                {
                    if ident == "repr" {
                        return Some(nested.into_iter().collect::<Vec<_>>());
                    } else if ident == "cfg_attr" {
                        // e.g.: #[cfg_attr(feature = "cffi", repr(C))]
                        // TODO: interpret firts part like `feature = "cffi"` and check out cfg
                        let v = nested.into_iter().filter_map(|attr| {
                            if let syn::NestedMeta::Meta(syn::Meta::List(syn::MetaList { ident, nested, .. })) = attr {
                                if ident == "repr" {
                                    return Some(nested.into_iter().collect::<Vec<_>>());
                                }
                            }
                            None
                        }).flat_map(|i| i).collect::<Vec<_>>();
                        if !v.is_empty() {
                            return Some(v);
                        }
                    }
                }
                None
            }).flat_map(|nested| nested)
            .filter_map(|meta| match meta {
                syn::NestedMeta::Meta(syn::Meta::Word(ident)) => Some(ident.to_string()),
                _ => None,
            });

        let mut repr = Repr::default();
        for id in ids {
            let new_ty = match id.as_ref() {
                "u8" => ReprType::U8,
                "u16" => ReprType::U16,
                "u32" => ReprType::U32,
                "usize" => ReprType::USize,
                "i8" => ReprType::I8,
                "i16" => ReprType::I16,
                "i32" => ReprType::I32,
                "isize" => ReprType::ISize,
                "C" => {
                    repr.style = ReprStyle::C;
                    continue;
                }
                _ => {
                    return Err(format!("Unsupported #[repr({})].", id));
                }
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
