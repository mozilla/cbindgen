/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;

use syn;

use bindgen::config::{Config, Language};
use bindgen::declarationtyperesolver::DeclarationTypeResolver;
use bindgen::dependencies::Dependencies;
use bindgen::ir::SynFieldHelpers;
use bindgen::ir::{
    AnnotationSet, Cfg, CfgWrite, Documentation, GenericParams, Item, ItemContainer, Repr, Type,
};
use bindgen::library::Library;
use bindgen::mangle;
use bindgen::monomorph::Monomorphs;
use bindgen::rename::{IdentifierType, RenameRule};
use bindgen::utilities::{find_first_some, IterHelpers};
use bindgen::writer::{ListType, Source, SourceWriter};

#[derive(Debug, Clone)]
pub struct Union {
    pub name: String,
    pub generic_params: GenericParams,
    pub fields: Vec<(String, Type, Documentation)>,
    pub tuple_union: bool,
    pub cfg: Option<Cfg>,
    pub annotations: AnnotationSet,
    pub documentation: Documentation,
}

impl Union {
    pub fn load(item: &syn::ItemUnion, mod_cfg: &Option<Cfg>) -> Result<Union, String> {
        if Repr::load(&item.attrs)? != Repr::C {
            return Err("Union is not marked #[repr(C)].".to_owned());
        }

        let (fields, tuple_union) = {
            let out = item
                .fields
                .named
                .iter()
                .try_skip_map(|x| x.as_ident_and_type())?;
            (out, false)
        };

        Ok(Union {
            name: item.ident.to_string(),
            generic_params: GenericParams::new(&item.generics),
            fields: fields,
            tuple_union: tuple_union,
            cfg: Cfg::append(mod_cfg, Cfg::load(&item.attrs)),
            annotations: AnnotationSet::load(&item.attrs)?,
            documentation: Documentation::load(&item.attrs),
        })
    }

    pub fn simplify_option_to_ptr(&mut self) {
        for &mut (_, ref mut ty, _) in &mut self.fields {
            ty.simplify_option_to_ptr();
        }
    }

    pub fn is_generic(&self) -> bool {
        self.generic_params.len() > 0
    }

    pub fn add_monomorphs(&self, library: &Library, out: &mut Monomorphs) {
        // Generic unions can instantiate monomorphs only once they've been
        // instantiated. See `instantiate_monomorph` for more details.
        if self.is_generic() {
            return;
        }

        for &(_, ref ty, _) in &self.fields {
            ty.add_monomorphs(library, out);
        }
    }

    pub fn mangle_paths(&mut self, monomorphs: &Monomorphs) {
        for &mut (_, ref mut ty, _) in &mut self.fields {
            ty.mangle_paths(monomorphs);
        }
    }
}

impl Item for Union {
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
        ItemContainer::Union(self.clone())
    }

    fn collect_declaration_types(&self, resolver: &mut DeclarationTypeResolver) {
        resolver.add_union(&self.name);
    }

    fn resolve_declaration_types(&mut self, resolver: &DeclarationTypeResolver) {
        for &mut (_, ref mut ty, _) in &mut self.fields {
            ty.resolve_declaration_types(resolver);
        }
    }

    fn rename_for_config(&mut self, config: &Config) {
        config.export.rename(&mut self.name);
        for &mut (_, ref mut ty, _) in &mut self.fields {
            let generic_parameter = match ty.get_root_path() {
                Some(ref p) => self.generic_params.contains(p),
                None => false,
            };
            if !generic_parameter {
                ty.rename_for_config(config);
            }
        }

        let rules = [
            self.annotations.parse_atom::<RenameRule>("rename-all"),
            config.structure.rename_fields,
        ];

        if let Some(o) = self.annotations.list("field-names") {
            let mut overriden_fields = Vec::new();

            for (i, &(ref name, ref ty, ref doc)) in self.fields.iter().enumerate() {
                if i >= o.len() {
                    overriden_fields.push((name.clone(), ty.clone(), doc.clone()));
                } else {
                    overriden_fields.push((o[i].clone(), ty.clone(), doc.clone()));
                }
            }

            self.fields = overriden_fields;
        } else if let Some(r) = find_first_some(&rules) {
            self.fields = self
                .fields
                .iter()
                .map(|x| {
                    (
                        r.apply_to_snake_case(&x.0, IdentifierType::StructMember),
                        x.1.clone(),
                        x.2.clone(),
                    )
                }).collect();
        } else if self.tuple_union {
            // If we don't have any rules for a tuple union, prefix them with
            // an underscore so it still compiles
            for &mut (ref mut name, ..) in &mut self.fields {
                name.insert(0, '_');
            }
        }
    }

    fn add_dependencies(&self, library: &Library, out: &mut Dependencies) {
        for &(_, ref ty, _) in &self.fields {
            ty.add_dependencies_ignoring_generics(&self.generic_params, library, out);
        }
    }

    fn instantiate_monomorph(
        &self,
        generic_values: &Vec<Type>,
        library: &Library,
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

        let mappings = self
            .generic_params
            .iter()
            .zip(generic_values.iter())
            .collect::<Vec<_>>();

        let monomorph = Union {
            name: mangle::mangle_path(&self.name, generic_values),
            generic_params: GenericParams::default(),
            fields: self
                .fields
                .iter()
                .map(|x| (x.0.clone(), x.1.specialize(&mappings), x.2.clone()))
                .collect(),
            tuple_union: self.tuple_union,
            cfg: self.cfg.clone(),
            annotations: self.annotations.clone(),
            documentation: self.documentation.clone(),
        };

        // Instantiate any monomorphs for any generic paths we may have just created.
        monomorph.add_monomorphs(library, out);

        out.insert_union(self, monomorph, generic_values.clone());
    }
}

impl Source for Union {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        self.cfg.write_before(config, out);

        self.documentation.write(config, out);

        self.generic_params.write(config, out);

        // The following results in
        // C++ or C with Tag as style:
        //   union Name {
        // C with Type only style:
        //   typedef union {
        // C with Both as style:
        //   typedef union Name {
        if config.language == Language::C && config.style.generate_typedef() {
            out.write("typedef ");
        }

        out.write("union");

        if config.language == Language::Cxx || config.style.generate_tag() {
            write!(out, " {}", self.name);
        }

        out.open_brace();

        if config.documentation {
            out.write_vertical_source_list(&self.fields, ListType::Cap(";"));
        } else {
            out.write_vertical_source_list(
                &self
                    .fields
                    .iter()
                    .map(|&(ref name, ref ty, _)| (name.clone(), ty.clone()))
                    .collect(),
                ListType::Cap(";"),
            );
        }

        if config.language == Language::C && config.style.generate_typedef() {
            out.close_brace(false);
            write!(out, " {};", self.name);
        } else {
            out.close_brace(true);
        }

        self.cfg.write_after(config, out);
    }
}
