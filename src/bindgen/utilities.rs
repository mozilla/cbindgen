/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

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
        if let &Some(ref x) = x {
            return Some(x);
        }
    }
    return None;
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
            syn::Item::Existential(ref item) => (|$i: &syn::ItemExistential| $a)(item),
            syn::Item::TraitAlias(ref item) => (|$i: &syn::ItemTraitAlias| $a)(item),
            syn::Item::Verbatim(_) => (|| $b)(),
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
                return self
                    .attrs
                    .iter()
                    .filter_map(|x| x.interpret_meta())
                    .any(|attr| {
                        if let syn::Meta::Word(ref ident) = attr {
                            ident == name
                        } else {
                            false
                        }
                    });
            }

            fn has_attr_list(&self, name: &str, args: &[&str]) -> bool {
                return self
                    .attrs
                    .iter()
                    .filter_map(|x| x.interpret_meta())
                    .any(|attr| {
                        if let syn::Meta::List(syn::MetaList { ident, nested, .. }) = attr {
                            if ident != name {
                                return false;
                            }
                            args.iter().all(|arg| {
                                nested.iter().any(|nested_meta| {
                                    if let syn::NestedMeta::Meta(syn::Meta::Word(ident)) =
                                        nested_meta
                                    {
                                        ident == arg
                                    } else {
                                        false
                                    }
                                })
                            })
                        } else {
                            false
                        }
                    });
            }

            fn has_attr_name_value(&self, name: &str, value: &str) -> bool {
                return self
                    .attrs
                    .iter()
                    .filter_map(|x| x.interpret_meta())
                    .any(|attr| {
                        if let syn::Meta::NameValue(syn::MetaNameValue { ident, lit, .. }) = attr {
                            if let syn::Lit::Str(lit) = lit {
                                (ident == name) && (&lit.value() == value)
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    });
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
impl_syn_item_helper!(syn::ItemExistential);
impl_syn_item_helper!(syn::ItemTraitAlias);

impl SynItemHelpers for syn::ItemVerbatim {
    fn has_attr_word(&self, _name: &str) -> bool {
        false
    }

    fn has_attr_list(&self, _name: &str, _args: &[&str]) -> bool {
        false
    }

    fn has_attr_name_value(&self, _name: &str, _value: &str) -> bool {
        false
    }
}

/// Helper function for accessing Abi information
pub trait SynAbiHelpers {
    fn is_c(&self) -> bool;
    fn is_omitted(&self) -> bool;
}

impl SynAbiHelpers for Option<syn::Abi> {
    fn is_c(&self) -> bool {
        if let &Some(ref abi) = self {
            if let Some(ref lit_string) = abi.name {
                return lit_string.value() == String::from("C");
            }
        }
        false
    }
    fn is_omitted(&self) -> bool {
        if let &Some(ref abi) = self {
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
}

impl SynAttributeHelpers for [syn::Attribute] {
    fn get_comment_lines(&self) -> Vec<String> {
        let mut comment_lines = Vec::new();

        for attr in self {
            if attr.style == syn::AttrStyle::Outer {
                if let Some(syn::Meta::NameValue(syn::MetaNameValue {
                    ident,
                    lit: syn::Lit::Str(comment),
                    ..
                })) = attr.interpret_meta()
                {
                    let name = ident.to_string();
                    let comment = comment.value();

                    if &*name == "doc" {
                        for raw in comment.lines() {
                            let line = raw
                                .trim_left_matches(" ")
                                .trim_left_matches("//")
                                .trim_left_matches("///")
                                .trim_left_matches("/**")
                                .trim_left_matches("/*")
                                .trim_left_matches("*/")
                                .trim_left_matches("*")
                                .trim_right();
                            comment_lines.push(line.to_owned());
                        }
                    }
                }
            }
        }

        comment_lines
    }
}
