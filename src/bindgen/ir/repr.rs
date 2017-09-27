/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use syn;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Repr {
    None,
    C,
    U8,
    U16,
    U32,
    I8,
    I16,
    I32,
}

impl Repr {
    pub fn load(attrs: &Vec<syn::Attribute>) -> Repr {
        if Repr::has_attr(Repr::repr_c(), attrs) {
            Repr::C
        } else if Repr::has_attr(Repr::repr_u32(), attrs) {
            Repr::U32
        } else if Repr::has_attr(Repr::repr_u16(), attrs) {
            Repr::U16
        } else if Repr::has_attr(Repr::repr_u8(), attrs) {
            Repr::U8
        } else if Repr::has_attr(Repr::repr_i32(), attrs) {
            Repr::I32
        } else if Repr::has_attr(Repr::repr_i16(), attrs) {
            Repr::I16
        } else if Repr::has_attr(Repr::repr_i8(), attrs) {
            Repr::I8
        } else {
            Repr::None
        }
    }

    fn has_attr(meta: syn::MetaItem, attrs: &Vec<syn::Attribute>) -> bool {
        attrs.iter().any(|x| !x.is_sugared_doc && x.value == meta)
    }

    fn repr_c() -> syn::MetaItem {
        syn::MetaItem::List(syn::Ident::new("repr"),
                            vec![syn::NestedMetaItem::MetaItem(syn::MetaItem::Word(syn::Ident::new("C")))])
    }

    fn repr_u32() -> syn::MetaItem {
        syn::MetaItem::List(syn::Ident::new("repr"),
                            vec![syn::NestedMetaItem::MetaItem(syn::MetaItem::Word(syn::Ident::new("u32")))])
    }

    fn repr_u16() -> syn::MetaItem {
        syn::MetaItem::List(syn::Ident::new("repr"),
                            vec![syn::NestedMetaItem::MetaItem(syn::MetaItem::Word(syn::Ident::new("u16")))])
    }

    fn repr_u8() -> syn::MetaItem {
        syn::MetaItem::List(syn::Ident::new("repr"),
                            vec![syn::NestedMetaItem::MetaItem(syn::MetaItem::Word(syn::Ident::new("u8")))])
    }

    fn repr_i32() -> syn::MetaItem {
        syn::MetaItem::List(syn::Ident::new("repr"),
                            vec![syn::NestedMetaItem::MetaItem(syn::MetaItem::Word(syn::Ident::new("i32")))])
    }

    fn repr_i16() -> syn::MetaItem {
        syn::MetaItem::List(syn::Ident::new("repr"),
                            vec![syn::NestedMetaItem::MetaItem(syn::MetaItem::Word(syn::Ident::new("i16")))])
    }

    fn repr_i8() -> syn::MetaItem {
        syn::MetaItem::List(syn::Ident::new("repr"),
                            vec![syn::NestedMetaItem::MetaItem(syn::MetaItem::Word(syn::Ident::new("i8")))])
    }
}
