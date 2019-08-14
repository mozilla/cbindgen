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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Repr {
    pub style: ReprStyle,
    pub ty: Option<ReprType>,
}

impl Repr {
    pub const C: Self = Repr {
        style: ReprStyle::C,
        ty: None,
    };

    pub const TRANSPARENT: Self = Repr {
        style: ReprStyle::Transparent,
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
                    Some(path.segments.first().unwrap().ident.to_string())
                }
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
                "transparent" => {
                    repr.style = ReprStyle::Transparent;
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
