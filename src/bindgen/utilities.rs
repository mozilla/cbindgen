/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#![allow(clippy::redundant_closure_call)]

use syn;

pub trait IterHelpers: Iterator {
    fn try_skip_map<F, T, E>(&mut self, f: F) -> Result<Vec<T>, E>
    where
        F: FnMut(&Self::Item) -> Result<Option<T>, E>;
}

impl<I> IterHelpers for I
where
    I: Iterator,
{
    fn try_skip_map<F, T, E>(&mut self, mut f: F) -> Result<Vec<T>, E>
    where
        F: FnMut(&Self::Item) -> Result<Option<T>, E>,
    {
        let mut out = Vec::new();
        while let Some(item) = self.next() {
            if let Some(x) = f(&item)? {
                out.push(x);
            }
        }
        Ok(out)
    }
}

pub fn find_first_some<T>(slice: &[Option<T>]) -> Option<&T> {
    for x in slice {
        if let Some(ref x) = *x {
            return Some(x);
        }
    }
    None
}

pub trait SynItemHelpers {
    /// Searches for attributes like `#[test]`.
    /// Example:
    /// - `item.has_attr_word("test")` => `#[test]`
    fn has_attr_word(&self, name: &str) -> bool;

    /// Searches for attributes like `#[cfg(test)]`.
    /// Example:
    /// - `item.has_attr_list("cfg", &["test"])` => `#[cfg(test)]`
    fn has_attr_list(&self, name: &str, args: &[&str]) -> bool;

    /// Searches for attributes like `#[feature = "std"]`.
    /// Example:
    /// - `item.has_attr_name_value("feature", "std")` => `#[feature = "std"]`
    fn has_attr_name_value(&self, name: &str, value: &str) -> bool;

    fn is_no_mangle(&self) -> bool {
        self.has_attr_word("no_mangle")
    }

    /// Searches for attributes `#[test]` and/or `#[cfg(test)]`.
    fn has_test_attr(&self) -> bool {
        self.has_attr_list("cfg", &["test"]) || self.has_attr_word("test")
    }
}

macro_rules! syn_item_match_helper {
    ($s:ident => has_attrs: |$i:ident| $a:block, otherwise: || $b:block) => {
        match *$s {
            syn::Item::Const(ref item) => (|$i: &syn::ItemConst| $a)(item),
            syn::Item::Enum(ref item) => (|$i: &syn::ItemEnum| $a)(item),
            syn::Item::ExternCrate(ref item) => (|$i: &syn::ItemExternCrate| $a)(item),
            syn::Item::Fn(ref item) => (|$i: &syn::ItemFn| $a)(item),
            syn::Item::ForeignMod(ref item) => (|$i: &syn::ItemForeignMod| $a)(item),
            syn::Item::Impl(ref item) => (|$i: &syn::ItemImpl| $a)(item),
            syn::Item::Macro(ref item) => (|$i: &syn::ItemMacro| $a)(item),
            syn::Item::Macro2(ref item) => (|$i: &syn::ItemMacro2| $a)(item),
            syn::Item::Mod(ref item) => (|$i: &syn::ItemMod| $a)(item),
            syn::Item::Static(ref item) => (|$i: &syn::ItemStatic| $a)(item),
            syn::Item::Struct(ref item) => (|$i: &syn::ItemStruct| $a)(item),
            syn::Item::Trait(ref item) => (|$i: &syn::ItemTrait| $a)(item),
            syn::Item::Type(ref item) => (|$i: &syn::ItemType| $a)(item),
            syn::Item::Union(ref item) => (|$i: &syn::ItemUnion| $a)(item),
            syn::Item::Use(ref item) => (|$i: &syn::ItemUse| $a)(item),
            syn::Item::TraitAlias(ref item) => (|$i: &syn::ItemTraitAlias| $a)(item),
            syn::Item::Verbatim(_) => (|| $b)(),
            _ => panic!("Unhandled syn::Item:  {:?}", $s),
        }
    };
}

impl SynItemHelpers for syn::Item {
    fn has_attr_word(&self, name: &str) -> bool {
        syn_item_match_helper!(self =>
            has_attrs: |item| { item.has_attr_word(name) },
            otherwise: || { false }
        )
    }

    fn has_attr_list(&self, name: &str, args: &[&str]) -> bool {
        syn_item_match_helper!(self =>
            has_attrs: |item| { item.has_attr_list(name, args) },
            otherwise: || { false }
        )
    }

    fn has_attr_name_value(&self, name: &str, value: &str) -> bool {
        syn_item_match_helper!(self =>
            has_attrs: |item| { item.has_attr_name_value(name, value) },
            otherwise: || { false }
        )
    }
}

macro_rules! impl_syn_item_helper {
    ($t:ty) => {
        impl SynItemHelpers for $t {
            fn has_attr_word(&self, name: &str) -> bool {
                self.attrs.has_attr_word(name)
            }

            fn has_attr_list(&self, name: &str, args: &[&str]) -> bool {
                self.attrs.has_attr_list(name, args)
            }

            fn has_attr_name_value(&self, name: &str, value: &str) -> bool {
                self.attrs.has_attr_name_value(name, value)
            }
        }
    };
}

impl_syn_item_helper!(syn::ItemExternCrate);
impl_syn_item_helper!(syn::ItemUse);
impl_syn_item_helper!(syn::ItemStatic);
impl_syn_item_helper!(syn::ItemConst);
impl_syn_item_helper!(syn::ItemFn);
impl_syn_item_helper!(syn::ItemMod);
impl_syn_item_helper!(syn::ItemForeignMod);
impl_syn_item_helper!(syn::ItemType);
impl_syn_item_helper!(syn::ItemStruct);
impl_syn_item_helper!(syn::ItemEnum);
impl_syn_item_helper!(syn::ItemUnion);
impl_syn_item_helper!(syn::ItemTrait);
impl_syn_item_helper!(syn::ItemImpl);
impl_syn_item_helper!(syn::ItemMacro);
impl_syn_item_helper!(syn::ItemMacro2);
impl_syn_item_helper!(syn::ItemTraitAlias);

/// Helper function for accessing Abi information
pub trait SynAbiHelpers {
    fn is_c(&self) -> bool;
    fn is_omitted(&self) -> bool;
}

impl SynAbiHelpers for Option<syn::Abi> {
    fn is_c(&self) -> bool {
        if let Some(ref abi) = *self {
            if let Some(ref lit_string) = abi.name {
                return lit_string.value() == String::from("C");
            }
        }
        false
    }
    fn is_omitted(&self) -> bool {
        if let Some(ref abi) = *self {
            abi.name.is_none()
        } else {
            false
        }
    }
}

impl SynAbiHelpers for syn::Abi {
    fn is_c(&self) -> bool {
        if let Some(ref lit_string) = self.name {
            lit_string.value() == String::from("C")
        } else {
            false
        }
    }
    fn is_omitted(&self) -> bool {
        self.name.is_none()
    }
}

pub trait SynAttributeHelpers {
    fn get_comment_lines(&self) -> Vec<String>;
    fn has_attr_word(&self, name: &str) -> bool;
    fn has_attr_list(&self, name: &str, args: &[&str]) -> bool;
    fn has_attr_name_value(&self, name: &str, value: &str) -> bool;
}

impl SynAttributeHelpers for [syn::Attribute] {
    fn has_attr_word(&self, name: &str) -> bool {
        self.iter().filter_map(|x| x.parse_meta().ok()).any(|attr| {
            if let syn::Meta::Path(ref path) = attr {
                path.is_ident(name)
            } else {
                false
            }
        })
    }

    fn has_attr_list(&self, name: &str, args: &[&str]) -> bool {
        self.iter().filter_map(|x| x.parse_meta().ok()).any(|attr| {
            if let syn::Meta::List(syn::MetaList { path, nested, .. }) = attr {
                if !path.is_ident(name) {
                    return false;
                }
                args.iter().all(|arg| {
                    nested.iter().any(|nested_meta| {
                        if let syn::NestedMeta::Meta(syn::Meta::Path(path)) = nested_meta {
                            path.is_ident(arg)
                        } else {
                            false
                        }
                    })
                })
            } else {
                false
            }
        })
    }

    fn has_attr_name_value(&self, name: &str, value: &str) -> bool {
        self.iter().filter_map(|x| x.parse_meta().ok()).any(|attr| {
            if let syn::Meta::NameValue(syn::MetaNameValue { path, lit, .. }) = attr {
                if let syn::Lit::Str(lit) = lit {
                    path.is_ident(name) && (&lit.value() == value)
                } else {
                    false
                }
            } else {
                false
            }
        })
    }

    fn get_comment_lines(&self) -> Vec<String> {
        let mut comment = Vec::new();

        for attr in self {
            if attr.style == syn::AttrStyle::Outer {
                if let Ok(syn::Meta::NameValue(syn::MetaNameValue {
                    path,
                    lit: syn::Lit::Str(content),
                    ..
                })) = attr.parse_meta()
                {
                    if path.is_ident("doc") {
                        let text = content.value().trim_end().to_owned();
                        comment.push(text);
                    }
                }
            }
        }

        comment
    }
}
