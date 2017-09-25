/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use syn;

use bindgen::ir::Type;
use bindgen::utilities::IterHelpers;
use bindgen::mangle;

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
        if path.segments.len() != 1 {
            return Err(format!("path contains more than one segment"));
        }

        let name = path.segments[0].ident.to_string();

        if name == "PhantomData" {
            return Ok(GenericPath::new(name, Vec::new()));
        }

        let generics = match &path.segments[0].parameters {
            &syn::PathParameters::AngleBracketed(ref d) => {
                if !d.lifetimes.is_empty() ||
                   !d.bindings.is_empty() {
                    return Err(format!("path generic parameter contains bindings, or lifetimes"));
                }

                d.types.iter()
                       .try_skip_map(|x| Type::load(x))?
            }
            &syn::PathParameters::Parenthesized(_) => {
                return Err(format!("path contains parentheses"));
            }
        };

        Ok(GenericPath::new(name, generics))
    }

    pub fn mangle(&self) -> Path {
        mangle::mangle_path(&self.name, &self.generics)
    }
}
