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
    fn has_attr_word(&self, word: &str) -> bool;

    fn is_no_mangle(&self) -> bool {
        self.has_attr_word("no_mangle")
    }
}

fn has_attr_word(attrs: &Vec<syn::Attribute>, word: &str) -> bool {
    return attrs
        .iter()
        .filter_map(|x| x.interpret_meta())
        .any(|attr| match attr {
            syn::Meta::Word(ref ident) if ident == word => true,
            // e.g.: #[cfg_attr(feature = "cffi", no_mangle)]
            // TODO: interpret firts part like `feature = "cffi"` and check out cfg
            syn::Meta::List(syn::MetaList {
                ref ident,
                ref nested,
                ..
            })
                if ident == "cfg_attr" =>
            {
                if nested.into_iter().any(|attr| match attr {
                    syn::NestedMeta::Meta(syn::Meta::Word(ref ident)) if ident == word => true,
                    syn::NestedMeta::Meta(syn::Meta::List(syn::MetaList { ident, .. }))
                        if ident == word =>
                    {
                        true
                    }
                    _ => false,
                }) {
                    return true;
                }
                false
            }
            _ => false,
        });
}

impl SynItemHelpers for syn::ItemStruct {
    fn has_attr_word(&self, word: &str) -> bool {
        return has_attr_word(&self.attrs, word);
    }
}

impl SynItemHelpers for syn::ItemFn {
    fn has_attr_word(&self, word: &str) -> bool {
        return has_attr_word(&self.attrs, word);
    }
}

impl SynItemHelpers for syn::ItemStatic {
    fn has_attr_word(&self, word: &str) -> bool {
        return has_attr_word(&self.attrs, word);
    }
}

impl SynItemHelpers for syn::Variant {
    fn has_attr_word(&self, word: &str) -> bool {
        return has_attr_word(&self.attrs, word);
    }
}

impl SynItemHelpers for syn::Field {
    fn has_attr_word(&self, word: &str) -> bool {
        return has_attr_word(&self.attrs, word);
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
