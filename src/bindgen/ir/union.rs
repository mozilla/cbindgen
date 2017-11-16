/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;

use syn;

use bindgen::config::{Config, Language};
use bindgen::dependencies::Dependencies;
use bindgen::ir::{AnnotationSet, Cfg, CfgWrite, Documentation, GenericParams, ItemContainer, Item, Repr, Type};
use bindgen::ir::SynFieldHelpers;
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
    pub fn load(name: String,
                decl: &syn::VariantData,
                generics: &syn::Generics,
                attrs: &Vec<syn::Attribute>,
                mod_cfg: &Option<Cfg>) -> Result<Union, String>
    {
        if Repr::load(attrs) != Repr::C {
            return Err("Union is not marked #[repr(C)].".to_owned());
        }

        let (fields, tuple_union) = match decl {
            &syn::VariantData::Struct(ref fields) => {
                let out = fields.iter()
                                .try_skip_map(|x| x.as_ident_and_type())?;
                (out, false)
            }
            &syn::VariantData::Tuple(ref fields) => {
                let mut out = Vec::new();
                let mut current = 0;
                for field in fields {
                    if let Some(x) = Type::load(&field.ty)? {
                        out.push((format!("{}", current), x, Documentation::load(&field.attrs)));
                        current += 1;
                    }
                }
                (out, true)
            }
            &syn::VariantData::Unit => {
                (vec![], false)
            }
        };

        Ok(Union {
            name: name,
            generic_params: GenericParams::new(generics),
            fields: fields,
            tuple_union: tuple_union,
            cfg: Cfg::append(mod_cfg, Cfg::load(attrs)),
            annotations: AnnotationSet::load(attrs)?,
            documentation: Documentation::load(attrs),
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

    fn rename_for_config(&mut self, config: &Config) {
        let rules = [self.annotations.parse_atom::<RenameRule>("rename-all"),
                     config.structure.rename_fields];

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
            self.fields = self.fields.iter()
                                     .map(|x| (r.apply_to_snake_case(&x.0,
                                                                     IdentifierType::StructMember),
                                               x.1.clone(),
                                               x.2.clone()))
                                     .collect();
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

    fn instantiate_monomorph(&self, generic_values: &Vec<Type>, library: &Library, out: &mut Monomorphs) {
        assert!(self.generic_params.len() > 0 &&
                self.generic_params.len() == generic_values.len());

        let mappings = self.generic_params.iter()
                                          .zip(generic_values.iter())
                                          .collect::<Vec<_>>();

        let monomorph = Union {
            name: mangle::mangle_path(&self.name, generic_values),
            generic_params: GenericParams::default(),
            fields: self.fields.iter()
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

        if config.language == Language::C {
            out.write("typedef union");
        } else {
            write!(out, "union {}", self.name);
        }
        out.open_brace();

        if config.documentation {
            out.write_vertical_source_list(&self.fields, ListType::Cap(";"));
        } else {
            out.write_vertical_source_list(&self.fields.iter()
                .map(|&(ref name, ref ty, _)| (name.clone(), ty.clone()))
                .collect(),
                ListType::Cap(";"));
        }

        if config.language == Language::C {
            out.close_brace(false);
            write!(out, " {};", self.name);
        } else {
            out.close_brace(true);
        }

        self.cfg.write_after(config, out);
    }
}
