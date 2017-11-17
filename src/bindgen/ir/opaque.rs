/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;

use syn;

use bindgen::config::{Config, Language};
use bindgen::dependencies::Dependencies;
use bindgen::ir::{AnnotationSet, Cfg, CfgWrite, Documentation, ItemContainer, Item, Path, Specialization, Type};
use bindgen::library::Library;
use bindgen::mangle;
use bindgen::monomorph::Monomorphs;
use bindgen::writer::{Source, SourceWriter};

#[derive(Debug, Clone)]
pub struct OpaqueItem {
    pub name: Path,
    pub generic_params: Vec<String>,
    pub cfg: Option<Cfg>,
    pub annotations: AnnotationSet,
    pub documentation: Documentation,
}

impl OpaqueItem {
    pub fn new(name: String,
               generics: &syn::Generics,
               attrs: &Vec<syn::Attribute>,
               mod_cfg: &Option<Cfg>) -> OpaqueItem {
        let generic_params = generics.ty_params.iter()
                                               .map(|x| x.ident.to_string())
                                               .collect::<Vec<_>>();

        OpaqueItem {
            name: name,
            generic_params: generic_params,
            cfg: Cfg::append(mod_cfg, Cfg::load(attrs)),
            annotations: AnnotationSet::load(attrs).unwrap_or(AnnotationSet::new()),
            documentation: Documentation::load(attrs),
        }
    }

    pub fn instantiate_monomorph(&self, generic_values: &Vec<Type>, out: &mut Monomorphs) {
        assert!(self.generic_params.len() > 0);

        let monomorph = OpaqueItem {
            name: mangle::mangle_path(&self.name, generic_values),
            generic_params: vec![],
            cfg: self.cfg.clone(),
            annotations: self.annotations.clone(),
            documentation: self.documentation.clone(),
        };

        out.insert_opaque(self, monomorph, generic_values.clone());
    }
}

impl Item for OpaqueItem {
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
        ItemContainer::OpaqueItem(self.clone())
    }

    fn specialize(&self, _: &Library, aliasee: &Specialization) -> Result<Box<Item>, String> {
        if aliasee.aliased.generics.len() !=
           self.generic_params.len() {
            return Err("Incomplete specialization, the amount of generics in the path doesn't match the amount of generics in the item.".to_owned());
        }

        Ok(Box::new(OpaqueItem {
            name: aliasee.name.clone(),
            generic_params: aliasee.generic_params.clone(),
            cfg: aliasee.cfg.clone(),
            annotations: aliasee.annotations.clone(),
            documentation: aliasee.documentation.clone(),
        }))
    }

    fn add_dependencies(&self, _: &Library, _: &mut Dependencies) {
    }
}

impl Source for OpaqueItem {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        self.cfg.write_before(config, out);

        self.documentation.write(config, out);

        if config.language == Language::C {
            write!(out, "typedef struct {} {};", self.name, self.name);
        } else {
            write!(out, "struct {};", self.name);
        }

        self.cfg.write_after(config, out);
    }
}
