/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use syn;

use bindgen::ir::Type;
use bindgen::utilities::IterHelpers;

pub type Path = String;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GenericPath {
    pub name: String,
    pub generics: Vec<Type>,
}

impl GenericPath {
    pub fn new(name: String, generics: Vec<Type>) -> GenericPath {
        GenericPath {
            name: name,
            generics: generics,
        }
    }

    pub fn load(path: &syn::Path) -> Result<GenericPath, String> {
        assert!(path.segments.len() > 0, "{:?} doesn't have any segments", path);
        let last_segment_token = path.segments.last().unwrap();
        let last_segment = last_segment_token.value();
        let name = last_segment.ident.to_string();

        if name == "PhantomData" {
            return Ok(GenericPath::new(name, Vec::new()));
        }

        let generics = match &last_segment.arguments {
            &syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                ref args,
                ..
            }) => args.iter().try_skip_map(|x| match *x {
                &syn::GenericArgument::Type(ref x) => Type::load(x),
                _ => Err(String::new()),
            })?,
            &syn::PathArguments::Parenthesized(_) => {
                return Err("Path contains parentheses.".to_owned());
            }
            _ => Vec::new(),
        };

        Ok(GenericPath::new(name, generics))
    }
}
