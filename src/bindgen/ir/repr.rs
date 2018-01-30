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

    pub fn load(attrs: &[syn::Attribute]) -> Repr {
        attrs
        .iter()
        .filter_map(|attr| match *attr {
            syn::Attribute {
                style: syn::AttrStyle::Outer,
                is_sugared_doc: false,
                value: syn::MetaItem::List(ref id, ref nested)
            } if id == "repr" => Some(nested),
            _ => None,
        })
        .flat_map(|nested| nested)
        .filter_map(|meta| match *meta {
            syn::NestedMetaItem::MetaItem(syn::MetaItem::Word(ref id)) => Some(id.as_ref()),
            _ => None,
        })
        .fold(Repr::default(), |mut acc, id| {
            if id == "C" {
                acc.style = ReprStyle::C;
            } else {
                acc.ty = Some(match id {
                    "u8" => ReprType::U8,
                    "u16" => ReprType::U16,
                    "u32" => ReprType::U32,
                    "usize" => ReprType::USize,
                    "i8" => ReprType::I8,
                    "i16" => ReprType::I16,
                    "i32" => ReprType::I32,
                    "isize" => ReprType::ISize,
                    _ => return acc
                });
            }
            acc
        })
    }
}
