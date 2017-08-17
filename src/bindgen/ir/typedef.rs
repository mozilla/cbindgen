/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::HashMap;
use std::io::Write;

use syn;

use bindgen::config::Config;
use bindgen::dependencies::DependencyKind;
use bindgen::ir::{AnnotationSet, Cfg, CfgWrite, Documentation, Path, Type, Specialization, Item};
use bindgen::library::Library;
use bindgen::writer::{Source, SourceWriter};
use bindgen::utilities::IterHelpers;

/// A type alias that is represented as a C typedef
#[derive(Debug, Clone)]
pub struct Typedef {
    pub name: String,
    pub aliased: Type,
    pub cfg: Option<Cfg>,
    pub annotations: AnnotationSet,
    pub generic_params: Vec<String>,
    pub generic_values: Vec<Type>,
    pub documentation: Documentation,
    pub specialization: Option<Specialization>,
}

impl Typedef {
    pub fn load(name: String,
                attrs: &Vec<syn::Attribute>,
                generics: &syn::Generics,
                ty: &syn::Ty,
                mod_cfg: &Option<Cfg>) -> Result<Typedef, String> {
        if let Some(x) = Type::load(ty)? {
            match ty {
                &syn::Ty::Path(_, ref p) => {
                   let generic_params = generics
                        .ty_params
                        .iter()
                        .map(|x| x.ident.to_string())
                        .collect::<Vec<_>>();


                    let generic_values = match p.segments[0].parameters {
                        syn::PathParameters::AngleBracketed(ref d) => {
                            if !d.lifetimes.is_empty() ||
                                !d.bindings.is_empty()
                            {
                                return Err(format!("path generic parameter contains bindings, or lifetimes"));
                            }

                         d.types.iter().try_skip_map(|x| Type::load(x))?
                          }
                        syn::PathParameters::Parenthesized(_) => {
                            return Err(format!("path contains parentheses"));
                        }
                    };
                    Ok(Typedef {
                        name: name,
                        aliased: x,
                        generic_params,
                        generic_values,
                        specialization: None,
                        cfg: Cfg::append(mod_cfg, Cfg::load(attrs)),
                        annotations: AnnotationSet::load(attrs)?,
                        documentation: Documentation::load(attrs),
                    })
                }
                _ if generics.ty_params.is_empty() &&
                    generics.lifetimes.is_empty() => {
                    Ok(Typedef {
                        name: name,
                        aliased: x,
                        generic_params: Vec::new(),
                        generic_values: Vec::new(),
                        specialization: None,
                        cfg: Cfg::append(mod_cfg, Cfg::load(attrs)),
                        annotations: AnnotationSet::load(attrs)?,
                        documentation: Documentation::load(attrs),
                    })
                }
                i => {
                    println!("{:?}", i);
                    unimplemented!()
                }
            }
        } else {
            Err(format!("cannot have a typedef of a zero sized type"))
        }
    }

    pub fn transfer_annotations(&mut self, out: &mut HashMap<Path, AnnotationSet>) {
        if self.annotations.is_empty() {
            return;
        }

        match self.aliased.get_root_path() {
            Some(alias_path) => {
                if out.contains_key(&alias_path) {
                    warn!("multiple typedef's with annotations for {}. ignoring annotations from {}.",
                          alias_path, self.name);
                    return;
                }

                out.insert(alias_path, self.annotations.clone());
                self.annotations = AnnotationSet::new();
            }
            None => { }
        }
    }

    pub fn mangle_paths(&mut self) {
        self.aliased.mangle_paths();
    }

    pub fn get_deps(&self, library: &Library) -> Vec<(Item, DependencyKind)> {
        assert!(self.generic_params.is_empty());
        let mut ret = self.aliased.get_items(library, DependencyKind::Normal);
        if let Some(ref s) = self.specialization {
            ret.push((Item::Specialization(s.clone()), DependencyKind::Normal));
        }
        ret
    }
}

impl Source for Typedef {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        self.cfg.write_before(config, out);

        self.documentation.write(config, out);

        out.write("typedef ");
        (self.name.clone(), self.aliased.clone()).write(config, out);
        out.write(";");

        self.cfg.write_after(config, out);
    }
}
