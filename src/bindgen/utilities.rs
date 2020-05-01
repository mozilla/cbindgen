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

pub trait SynItemFnHelpers: SynAttributeHelpers {
    fn exported_name(&self) -> Option<String>;
}

impl SynItemFnHelpers for syn::ItemFn {
    fn exported_name(&self) -> Option<String> {
        self.attrs
            .attr_name_value_lookup("export_name")
            .or_else(|| {
                if self.is_no_mangle() {
                    Some(self.sig.ident.to_string())
                } else {
                    None
                }
            })
    }
}

impl SynItemFnHelpers for syn::ImplItemMethod {
    fn exported_name(&self) -> Option<String> {
        self.attrs
            .attr_name_value_lookup("export_name")
            .or_else(|| {
                if self.is_no_mangle() {
                    Some(self.sig.ident.to_string())
                } else {
                    None
                }
            })
    }
}

pub trait SynAttributeHelpers {
    /// Returns the list of attributes for an item.
    fn attrs(&self) -> &[syn::Attribute];

    /// Searches for attributes like `#[test]`.
    /// Example:
    /// - `item.has_attr_word("test")` => `#[test]`
    fn has_attr_word(&self, name: &str) -> bool {
        self.attrs()
            .iter()
            .filter_map(|x| x.parse_meta().ok())
            .any(|attr| {
                if let syn::Meta::Path(ref path) = attr {
                    path.is_ident(name)
                } else {
                    false
                }
            })
    }

    /// Searches for attributes like `#[cfg(test)]`.
    /// Example:
    /// - `item.has_attr_list("cfg", &["test"])` => `#[cfg(test)]`
    fn has_attr_list(&self, name: &str, args: &[&str]) -> bool {
        self.attrs()
            .iter()
            .filter_map(|x| x.parse_meta().ok())
            .any(|attr| {
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

    fn is_no_mangle(&self) -> bool {
        self.has_attr_word("no_mangle")
    }

    /// Searches for attributes `#[test]` and/or `#[cfg(test)]`.
    fn has_test_attr(&self) -> bool {
        self.has_attr_list("cfg", &["test"]) || self.has_attr_word("test")
    }

    fn attr_name_value_lookup(&self, name: &str) -> Option<String> {
        self.attrs()
            .iter()
            .filter_map(|attr| {
                let attr = attr.parse_meta().ok()?;
                if let syn::Meta::NameValue(syn::MetaNameValue {
                    path,
                    lit: syn::Lit::Str(lit),
                    ..
                }) = attr
                {
                    if path.is_ident(name) {
                        return Some(lit.value());
                    }
                }
                None
            })
            .next()
    }

    fn get_comment_lines(&self) -> Vec<String> {
        let mut comment = Vec::new();

        for attr in self.attrs() {
            if attr.style == syn::AttrStyle::Outer {
                if let Ok(syn::Meta::NameValue(syn::MetaNameValue {
                    path,
                    lit: syn::Lit::Str(content),
                    ..
                })) = attr.parse_meta()
                {
                    if path.is_ident("doc") {
                        comment.extend(split_doc_attr(&content.value()));
                    }
                }
            }
        }

        comment
    }
}

macro_rules! syn_item_match_helper {
    ($s:ident => has_attrs: |$i:ident| $a:block, otherwise: || $b:block) => {
        match *$s {
            syn::Item::Const(ref $i) => $a,
            syn::Item::Enum(ref $i) => $a,
            syn::Item::ExternCrate(ref $i) => $a,
            syn::Item::Fn(ref $i) => $a,
            syn::Item::ForeignMod(ref $i) => $a,
            syn::Item::Impl(ref $i) => $a,
            syn::Item::Macro(ref $i) => $a,
            syn::Item::Macro2(ref $i) => $a,
            syn::Item::Mod(ref $i) => $a,
            syn::Item::Static(ref $i) => $a,
            syn::Item::Struct(ref $i) => $a,
            syn::Item::Trait(ref $i) => $a,
            syn::Item::Type(ref $i) => $a,
            syn::Item::Union(ref $i) => $a,
            syn::Item::Use(ref $i) => $a,
            syn::Item::TraitAlias(ref $i) => $a,
            syn::Item::Verbatim(_) => $b,
            _ => panic!("Unhandled syn::Item:  {:?}", $s),
        }
    };
}

impl SynAttributeHelpers for syn::Item {
    fn attrs(&self) -> &[syn::Attribute] {
        syn_item_match_helper!(self =>
            has_attrs: |item| { &item.attrs },
            otherwise: || { &[] }
        )
    }
}

macro_rules! impl_syn_item_helper {
    ($t:ty) => {
        impl SynAttributeHelpers for $t {
            fn attrs(&self) -> &[syn::Attribute] {
                &self.attrs
            }
        }
    };
}

impl_syn_item_helper!(syn::ItemExternCrate);
impl_syn_item_helper!(syn::ItemUse);
impl_syn_item_helper!(syn::ItemStatic);
impl_syn_item_helper!(syn::ItemConst);
impl_syn_item_helper!(syn::ItemFn);
impl_syn_item_helper!(syn::ImplItemMethod);
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

impl SynAttributeHelpers for [syn::Attribute] {
    fn attrs(&self) -> &[syn::Attribute] {
        self
    }
}

fn split_doc_attr(input: &str) -> Vec<String> {
    input
        // Convert two newline (indicate "new paragraph") into two line break.
        .replace("\n\n", "  \n  \n")
        // Convert newline after two spaces (indicate "line break") into line break.
        .split("  \n")
        // Convert single newline (indicate hard-wrapped) into space.
        .map(|s| s.replace('\n', " "))
        .map(|s| s.trim_end().to_string())
        .collect()
}
