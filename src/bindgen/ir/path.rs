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
        assert!(path.segments.len() > 0);
        let last_segment = path.segments.last().unwrap();

        let name = last_segment.ident.to_string();

        if name == "PhantomData" {
            return Ok(GenericPath::new(name, Vec::new()));
        }

        let generics = match &last_segment.parameters {
            &syn::PathParameters::AngleBracketed(ref d) => {
                d.types.iter()
                       .try_skip_map(|x| Type::load(x))?
            }
            &syn::PathParameters::Parenthesized(_) => {
                return Err("Path contains parentheses.".to_owned());
            }
        };

        Ok(GenericPath::new(name, generics))
    }
}
