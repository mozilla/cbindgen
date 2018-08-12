/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;

use syn;

use bindgen::config::{Config, Language};
use bindgen::declarationtyperesolver::DeclarationTypeResolver;
use bindgen::dependencies::Dependencies;
use bindgen::ir::{
    AnnotationSet, Cfg, ConditionWrite, Documentation, GenericParams, Item, ItemContainer, Path,
    ToCondition, Type,
};
use bindgen::library::Library;
use bindgen::mangle;
use bindgen::monomorph::Monomorphs;
use bindgen::writer::{Source, SourceWriter};

#[derive(Debug, Clone)]
pub struct OpaqueItem {
    pub name: Path,
    pub generic_params: GenericParams,
    pub cfg: Option<Cfg>,
    pub annotations: AnnotationSet,
    pub documentation: Documentation,
}

impl OpaqueItem {
    pub fn new(
        name: String,
        generics: &syn::Generics,
        attrs: &Vec<syn::Attribute>,
        mod_cfg: &Option<Cfg>,
    ) -> OpaqueItem {
        OpaqueItem {
            name: name,
            generic_params: GenericParams::new(generics),
            cfg: Cfg::append(mod_cfg, Cfg::load(attrs)),
            annotations: AnnotationSet::load(attrs).unwrap_or(AnnotationSet::new()),
            documentation: Documentation::load(attrs),
        }
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

    fn collect_declaration_types(&self, resolver: &mut DeclarationTypeResolver) {
        resolver.add_struct(&self.name);
    }

    fn rename_for_config(&mut self, config: &Config) {
        config.export.rename(&mut self.name);
    }

    fn add_dependencies(&self, _: &Library, _: &mut Dependencies) {}

    fn instantiate_monomorph(
        &self,
        generic_values: &Vec<Type>,
        _library: &Library,
        out: &mut Monomorphs,
    ) {
        assert!(
            self.generic_params.len() > 0,
            "{} is not generic",
            self.name
        );
        assert!(
            self.generic_params.len() == generic_values.len(),
            "{} has {} params but is being instantiated with {} values",
            self.name,
            self.generic_params.len(),
            generic_values.len(),
        );

        let monomorph = OpaqueItem {
            name: mangle::mangle_path(&self.name, generic_values),
            generic_params: GenericParams::default(),
            cfg: self.cfg.clone(),
            annotations: self.annotations.clone(),
            documentation: self.documentation.clone(),
        };

        out.insert_opaque(self, monomorph, generic_values.clone());
    }
}

impl Source for OpaqueItem {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        let condition = (&self.cfg).to_condition(config);
        condition.write_before(config, out);

        self.documentation.write(config, out);

        self.generic_params.write(config, out);

        if config.style.generate_typedef() && config.language == Language::C {
            write!(out, "typedef struct {} {};", self.name, self.name);
        } else {
            write!(out, "struct {};", self.name);
        }

        condition.write_after(config, out);
    }
}
