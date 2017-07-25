/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use syn::*;

use bindgen::ir::*;

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
    fn has_attr(&self, target: MetaItem) -> bool;

    fn get_doc_attr(&self) -> String;

    fn is_no_mangle(&self) -> bool {
        self.has_attr(MetaItem::Word(Ident::new("no_mangle")))
    }

    fn is_repr_c(&self) -> bool {
        let repr_args = vec![NestedMetaItem::MetaItem(MetaItem::Word(Ident::new("C")))];
        self.has_attr(MetaItem::List(Ident::new("repr"), repr_args))
    }

    fn is_repr_u32(&self) -> bool {
        let repr_args = vec![NestedMetaItem::MetaItem(MetaItem::Word(Ident::new("u32")))];
        self.has_attr(MetaItem::List(Ident::new("repr"), repr_args))
    }

    fn is_repr_u16(&self) -> bool {
        let repr_args = vec![NestedMetaItem::MetaItem(MetaItem::Word(Ident::new("u16")))];
        self.has_attr(MetaItem::List(Ident::new("repr"), repr_args))
    }

    fn is_repr_u8(&self) -> bool {
        let repr_args = vec![NestedMetaItem::MetaItem(MetaItem::Word(Ident::new("u8")))];
        self.has_attr(MetaItem::List(Ident::new("repr"), repr_args))
    }

    fn get_repr(&self) -> Repr {
        if self.is_repr_c() {
            Repr::C
        } else if self.is_repr_u32() {
            Repr::U32
        } else if self.is_repr_u16() {
            Repr::U16
        } else if self.is_repr_u8() {
            Repr::U8
        } else {
            Repr::None
        }
    }
}

impl SynItemHelpers for Item {
    fn has_attr(&self, target: MetaItem) -> bool {
        return self.attrs
                   .iter()
                   .any(|ref attr| attr.style == AttrStyle::Outer && attr.value == target);
    }

    fn get_doc_attr(&self) -> String {
        let mut doc = String::new();
        for attr in &self.attrs {
            if attr.style == AttrStyle::Outer &&
               attr.is_sugared_doc {
                if let MetaItem::NameValue(_, Lit::Str(ref comment, _)) = attr.value {
                    doc.push_str(&comment);
                    doc.push('\n');
                }
            }
        }
        doc
    }
}

impl SynItemHelpers for ForeignItem {
    fn has_attr(&self, target: MetaItem) -> bool {
        return self.attrs
                   .iter()
                   .any(|ref attr| attr.style == AttrStyle::Outer && attr.value == target);
    }

    fn get_doc_attr(&self) -> String {
        let mut doc = String::new();
        for attr in &self.attrs {
            if attr.style == AttrStyle::Outer &&
               attr.is_sugared_doc {
                if let MetaItem::NameValue(_, Lit::Str(ref comment, _)) = attr.value {
                    doc.push_str(&comment);
                    doc.push('\n');
                }
            }
        }
        doc
    }
}

/// Helper function for accessing Abi information
pub trait SynAbiHelpers {
    fn is_c(&self) -> bool;
}

impl SynAbiHelpers for Option<Abi> {
    fn is_c(&self) -> bool {
        self == &Some(Abi::Named(String::from("C")))
    }
}

impl SynAbiHelpers for Abi {
    fn is_c(&self) -> bool {
        self == &Abi::Named(String::from("C"))
    }
}

/// Helper function for loading a Path and generics from a syn::Path
pub trait SynPathHelpers {
    fn convert_to_generic_single_segment(&self) -> Result<(String, Vec<Type>), String>;
}

impl SynPathHelpers for Path {
    fn convert_to_generic_single_segment(&self) -> Result<(String, Vec<Type>), String> {
        if self.segments.len() != 1 {
            return Err(format!("path contains more than one segment"));
        }

        let name = self.segments[0].ident.to_string();

        if name == "PhantomData" {
            return Ok((name, Vec::new()));
        }

        let generics = match &self.segments[0].parameters {
            &PathParameters::AngleBracketed(ref d) => {
                if !d.lifetimes.is_empty() ||
                   !d.bindings.is_empty() {
                    return Err(format!("path generic parameter contains bindings, or lifetimes"));
                }

                d.types.iter()
                       .try_skip_map(|x| Type::load(x))?
            }
            &PathParameters::Parenthesized(_) => {
                return Err(format!("path contains parentheses"));
            }
        };

        Ok((name, generics))
    }
}
