/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::HashMap;
use std::io::Write;

use syn;

use bindgen::config::{Config, Language};
use bindgen::dependencies::Dependencies;
use bindgen::ir::{AnnotationSet, Cfg, CfgWrite, Documentation, GenericParams, Item, ItemContainer,
                  Path, Type};
use bindgen::library::Library;
use bindgen::mangle;
use bindgen::monomorph::Monomorphs;
use bindgen::writer::{Source, SourceWriter};

/// A type alias that is represented as a C typedef
#[derive(Debug, Clone)]
pub struct Typedef {
    pub name: String,
    pub generic_params: GenericParams,
    pub aliased: Type,
    pub cfg: Option<Cfg>,
    pub annotations: AnnotationSet,
    pub documentation: Documentation,
}

impl Typedef {
    pub fn load(item: &syn::ItemType, mod_cfg: &Option<Cfg>) -> Result<Typedef, String> {
        if let Some(x) = Type::load(&item.ty)? {
            Ok(Typedef {
                name: item.ident.to_string(),
                generic_params: GenericParams::new(&item.generics),
                aliased: x,
                cfg: Cfg::append(mod_cfg, Cfg::load(&item.attrs)),
                annotations: AnnotationSet::load(&item.attrs)?,
                documentation: Documentation::load(&item.attrs),
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
                    warn!(
                        "Multiple typedef's with annotations for {}. Ignoring annotations from {}.",
                        alias_path, self.name
                    );
                    return;
                }

                out.insert(alias_path, self.annotations.clone());
                self.annotations = AnnotationSet::new();
            }
            None => {}
        }
    }

    pub fn is_generic(&self) -> bool {
        self.generic_params.len() > 0
    }

    pub fn add_monomorphs(&self, library: &Library, out: &mut Monomorphs) {
        // Generic structs can instantiate monomorphs only once they've been
        // instantiated. See `instantiate_monomorph` for more details.
        if self.is_generic() {
            return;
        }

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

    fn rename_for_config(&mut self, config: &Config) {
        config.export.rename(&mut self.name);
        self.aliased.rename_for_config(config);
    }

    fn add_dependencies(&self, library: &Library, out: &mut Dependencies) {
        self.aliased
            .add_dependencies_ignoring_generics(&self.generic_params, library, out);
    }

    fn instantiate_monomorph(
        &self,
        generic_values: &Vec<Type>,
        library: &Library,
        out: &mut Monomorphs,
    ) {
        assert!(self.generic_params.len() > 0 && self.generic_params.len() == generic_values.len());

        let mappings = self.generic_params
            .iter()
            .zip(generic_values.iter())
            .collect::<Vec<_>>();

        let monomorph = Typedef {
            name: mangle::mangle_path(&self.name, generic_values),
            generic_params: GenericParams::default(),
            aliased: self.aliased.specialize(&mappings),
            cfg: self.cfg.clone(),
            annotations: self.annotations.clone(),
            documentation: self.documentation.clone(),
        };

        // Instantiate any monomorphs for any generic paths we may have just created.
        monomorph.add_monomorphs(library, out);

        out.insert_typedef(self, monomorph, generic_values.clone());
    }
}

impl Source for Typedef {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        self.cfg.write_before(config, out);

        self.documentation.write(config, out);

        self.generic_params.write(config, out);

        if config.language == Language::C {
            out.write("typedef ");
            (self.name.clone(), self.aliased.clone()).write(config, out);
        } else {
            write!(out, "using {} = ", self.name);
            self.aliased.write(config, out);
        }
        out.write(";");

        self.cfg.write_after(config, out);
    }
}
