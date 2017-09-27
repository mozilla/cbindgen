/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use syn;

pub trait IterHelpers : Iterator {
    fn try_skip_map<F, T, E>(&mut self, f: F) -> Result<Vec<T>, E>
        where F: FnMut(&Self::Item) -> Result<Option<T>, E>;
}

impl<I> IterHelpers for I where I: Iterator {
    fn try_skip_map<F, T, E>(&mut self, mut f: F) -> Result<Vec<T>, E>
        where F: FnMut(&Self::Item) -> Result<Option<T>, E>
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
    fn has_attr(&self, target: syn::MetaItem) -> bool;

    fn is_no_mangle(&self) -> bool {
        self.has_attr(syn::MetaItem::Word(syn::Ident::new("no_mangle")))
    }
}

impl SynItemHelpers for syn::Item {
    fn has_attr(&self, target: syn::MetaItem) -> bool {
        return self.attrs
                   .iter()
                   .any(|ref attr| attr.style == syn::AttrStyle::Outer && attr.value == target);
    }
}

impl SynItemHelpers for syn::ForeignItem {
    fn has_attr(&self, target: syn::MetaItem) -> bool {
        return self.attrs
                   .iter()
                   .any(|ref attr| attr.style == syn::AttrStyle::Outer && attr.value == target);
    }
}

impl SynItemHelpers for syn::Variant {
    fn has_attr(&self, target: syn::MetaItem) -> bool {
        return self.attrs
                   .iter()
                   .any(|ref attr| attr.style == syn::AttrStyle::Outer && attr.value == target);
    }
}

impl SynItemHelpers for syn::Field {
    fn has_attr(&self, target: syn::MetaItem) -> bool {
        return self.attrs
                   .iter()
                   .any(|ref attr| attr.style == syn::AttrStyle::Outer && attr.value == target);
    }
}

/// Helper function for accessing Abi information
pub trait SynAbiHelpers {
    fn is_c(&self) -> bool;
    fn is_omitted(&self) -> bool;
}

impl SynAbiHelpers for Option<syn::Abi> {
    fn is_c(&self) -> bool {
        self == &Some(syn::Abi::Named(String::from("C")))
    }
    fn is_omitted(&self) -> bool {
        self == &Some(syn::Abi::Rust)
    }
}

impl SynAbiHelpers for syn::Abi {
    fn is_c(&self) -> bool {
        self == &syn::Abi::Named(String::from("C"))
    }
    fn is_omitted(&self) -> bool {
        self == &syn::Abi::Rust
    }
}
