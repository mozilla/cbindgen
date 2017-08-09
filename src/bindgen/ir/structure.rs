/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::BTreeMap;
use std::io::Write;

use syn;

use bindgen::annotation::*;
use bindgen::config::{Config, Language};
use bindgen::ir::*;
use bindgen::library::*;
use bindgen::mangle::*;
use bindgen::rename::*;
use bindgen::utilities::*;
use bindgen::writer::*;

#[derive(Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub annotations: AnnotationSet,
    pub fields: Vec<(String, Type, Documentation)>,
    pub generic_params: Vec<String>,
    pub documentation: Documentation,
}

impl Struct {
    pub fn load(name: String,
                annotations: AnnotationSet,
                decl: &syn::VariantData,
                generics: &syn::Generics,
                doc: String) -> Result<Struct, String>
    {
        let fields = match decl {
            &syn::VariantData::Struct(ref fields) => {
                fields.iter()
                      .try_skip_map(|x| x.as_ident_and_type())?
            }
            &syn::VariantData::Tuple(ref fields) => {
                let mut out = Vec::new();
                let mut current = 0;
                for field in fields {
                    if let Some(x) = Type::load(&field.ty)? {
                        out.push((format!("{}", current), x, Documentation::load(field.get_doc_attr())));
                        current += 1;
                    }
                }
                out
            }
            &syn::VariantData::Unit => {
                vec![]
            }
        };

        let generic_params = generics.ty_params.iter()
                                               .map(|x| x.ident.to_string())
                                               .collect::<Vec<_>>();

        Ok(Struct {
            name: name,
            annotations: annotations,
            fields: fields,
            generic_params: generic_params,
            documentation: Documentation::load(doc),
        })
    }

    pub fn add_deps(&self, library: &Library, out: &mut DependencyList) {
        for &(_, ref ty, _) in &self.fields {
            ty.add_deps_with_generics(&self.generic_params, library, out);
        }
    }

    pub fn add_monomorphs(&self, library: &Library, generic_values: &Vec<Type>, out: &mut Monomorphs) {
        assert!(self.generic_params.len() == generic_values.len());

        if self.generic_params.len() == 0 {
            for &(_, ref ty, _) in &self.fields {
                ty.add_monomorphs(library, out);
            }
            return;
        }

        let mappings = self.generic_params.iter()
                                          .zip(generic_values.iter())
                                          .collect::<Vec<_>>();

        let monomorph = Struct {
            name: mangle_path(&self.name, generic_values),
            annotations: self.annotations.clone(),
            fields: self.fields.iter()
                               .map(|x| (x.0.clone(), x.1.specialize(&mappings), x.2.clone()))
                               .collect(),
            generic_params: vec![],
            documentation: self.documentation.clone(),
        };

        for &(_, ref ty, _) in &monomorph.fields {
            ty.add_monomorphs(library, out);
        }

        if !out.contains_key(&self.name) {
            out.insert(self.name.clone(), BTreeMap::new());
        }
        out.get_mut(&self.name).unwrap().insert(generic_values.clone(), 
                                                Monomorph::Struct(monomorph));
    }

    pub fn rename_fields(&mut self, config: &Config) {
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
        }
    }

    pub fn mangle_paths(&mut self, monomorphs: &Monomorphs) {
        for &mut (_, ref mut ty, _) in &mut self.fields {
            ty.mangle_paths(monomorphs);
        }
    }
}

impl Source for Struct {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        assert!(self.generic_params.is_empty());

        self.documentation.write(config, out);
        if config.language == Language::C {
            out.write("typedef struct");
        } else {
            out.write(&format!("struct {}", self.name));
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

        if config.language == Language::Cxx {
            let mut wrote_start_newline = false;

            let other = if let Some(r) = config.function.rename_args {
                r.apply_to_snake_case("other", IdentifierType::FunctionArg)
            } else {
                String::from("other")
            };

            let mut emit_op = |op, conjuc| {
                if !wrote_start_newline {
                    wrote_start_newline = true;
                    out.new_line();
                }

                out.new_line();

                out.write(&format!("bool operator{}(const {}& {}) const", op, self.name, other));
                out.open_brace();
                out.write("return ");
                out.write_vertical_list(&self.fields.iter()
                                                    .map(|x| format!("{} {} {}.{}", x.0, op, other, x.0))
                                                    .collect(),
                                        ListType::Join(&format!(" {}", conjuc)));
                out.write(";");
                out.close_brace(false);
            };

            if config.structure.derive_eq(&self.annotations) &&
               !self.fields.is_empty() && self.fields.iter().all(|x| x.1.can_cmp_eq()) {
                emit_op("==", "&&");
            }
            if config.structure.derive_neq(&self.annotations) &&
               !self.fields.is_empty() && self.fields.iter().all(|x| x.1.can_cmp_eq()) {
                emit_op("!=", "||");
            }
            if config.structure.derive_lt(&self.annotations) &&
               self.fields.len() == 1 && self.fields[0].1.can_cmp_order() {
                emit_op("<", "&&");
            }
            if config.structure.derive_lte(&self.annotations) &&
               self.fields.len() == 1 && self.fields[0].1.can_cmp_order() {
                emit_op("<=", "&&");
            }
            if config.structure.derive_gt(&self.annotations) &&
               self.fields.len() == 1 && self.fields[0].1.can_cmp_order() {
                emit_op(">", "&&");
            }
            if config.structure.derive_gte(&self.annotations) &&
               self.fields.len() == 1 && self.fields[0].1.can_cmp_order() {
                emit_op(">=", "&&");
            }
        }

        if config.language == Language::C {
            out.close_brace(false);
            out.write(&format!(" {};", self.name));
        } else {
            out.close_brace(true);
        }
    }
}

pub trait SynFieldHelpers {
    fn as_ident_and_type(&self) -> Result<Option<(String, Type, Documentation)>, String>;
}

impl SynFieldHelpers for syn::Field {
    fn as_ident_and_type(&self) -> Result<Option<(String, Type, Documentation)>, String> {
        let ident = self.ident.as_ref().ok_or(format!("field is missing identifier"))?.clone();
        let converted_ty = Type::load(&self.ty)?;

        if let Some(x) = converted_ty {
            Ok(Some((ident.to_string(), x, Documentation::load(self.get_doc_attr()))))
        } else {
            Ok(None)
        }
    }
}
