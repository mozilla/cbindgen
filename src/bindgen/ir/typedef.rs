/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::HashMap;
use std::io::Write;

use syn;

use bindgen::config::Config;
use bindgen::dependencies::Dependencies;
use bindgen::ir::{AnnotationSet, Cfg, CfgWrite, Documentation, ItemContainer, Item, Path, Specialization, Type};
use bindgen::library::Library;
use bindgen::monomorph::Monomorphs;
use bindgen::writer::{Source, SourceWriter};

/// A type alias that is represented as a C typedef
#[derive(Debug, Clone)]
pub struct Typedef {
    pub name: String,
    pub aliased: Type,
    pub cfg: Option<Cfg>,
    pub annotations: AnnotationSet,
    pub documentation: Documentation,
}

impl Typedef {
    pub fn load(name: String,
                ty: &syn::Ty,
                attrs: &Vec<syn::Attribute>,
                mod_cfg: &Option<Cfg>) -> Result<Typedef, String> {
        if let Some(x) = Type::load(ty)? {
            Ok(Typedef {
                name: name,
                aliased: x,
                cfg: Cfg::append(mod_cfg, Cfg::load(attrs)),
                annotations: AnnotationSet::load(attrs)?,
                documentation: Documentation::load(attrs),
            })
        } else {
            Err("Cannot have a typedef of a zero sized type.".to_owned())
        }
    }

    pub fn simplify_option_to_ptr(&mut self) {
        self.aliased.simplify_option_to_ptr();
    }

    pub fn transfer_annotations(&mut self, out: &mut HashMap<Path, AnnotationSet>) {
        if self.annotations.is_empty() {
            return;
        }

        match self.aliased.get_root_path() {
            Some(alias_path) => {
                if out.contains_key(&alias_path) {
                    warn!("Multiple typedef's with annotations for {}. Ignoring annotations from {}.",
                          alias_path, self.name);
                    return;
                }

                out.insert(alias_path, self.annotations.clone());
                self.annotations = AnnotationSet::new();
            }
            None => { }
        }
    }

    pub fn add_monomorphs(&self, library: &Library, out: &mut Monomorphs) {
        self.aliased.add_monomorphs(library, out);
    }

    pub fn mangle_paths(&mut self, monomorphs: &Monomorphs) {
        self.aliased.mangle_paths(monomorphs);
    }
}

impl Item for Typedef {
    fn name(&self) -> &str {
        &self.name
    }

    fn cfg(&self) -> &Option<Cfg> {
        &self.cfg
    }

    fn annotations(&self) -> &AnnotationSet {
        &self.annotations
    }

    fn annotations_mut(&mut self) -> &mut AnnotationSet {
        &mut self.annotations
    }

    fn container(&self) -> ItemContainer {
        ItemContainer::Typedef(self.clone())
    }

    fn specialize(&self, _: &Library, aliasee: &Specialization) -> Result<Box<Item>, String> {
        Ok(Box::new(Typedef {
            name: aliasee.name.clone(),
            aliased: self.aliased.clone(),
            cfg: aliasee.cfg.clone(),
            annotations: aliasee.annotations.clone(),
            documentation: aliasee.documentation.clone(),
        }))
    }

    fn add_dependencies(&self, library: &Library, out: &mut Dependencies) {
        self.aliased.add_dependencies(library, out);
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
