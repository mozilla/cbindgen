/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::BTreeMap;
use std::io::Write;

use syn;

use bindgen::annotation::*;
use bindgen::config::{Config, Language, Layout};
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
    pub functions: Vec<Function>,
    pub destructor: Option<Function>,
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
            functions: Vec::new(),
            destructor: None,
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
            functions: vec![],
            destructor: None,
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

    pub fn add_specializations(&self, library: &Library, out: &mut SpecializationList) {
        for &(_, ref ty, _) in &self.fields {
            ty.add_specializations(library, out);
        }
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

        for f in &mut self.functions {
            f.rename_args(config);
        }
        if let Some(ref mut destructor) = self.destructor {
            destructor.rename_args(config);
        }
    }

    pub fn mangle_paths(&mut self, monomorphs: &Monomorphs) {
        for &mut (_, ref mut ty, _) in &mut self.fields {
            ty.mangle_paths(monomorphs);
        }
        for f in &mut self.functions {
            f.mangle_paths(monomorphs);
        }
        if let Some(ref mut destructor) = self.destructor {
            destructor.mangle_paths(monomorphs);
        }
    }

    pub fn add_member_function(&mut self, function: Function) {

        if function.annotations.bool("destructor").unwrap_or(false)
            && self.destructor.is_none() && function.args.is_empty()
        {
            self.destructor = Some(function);
        } else if !function.annotations.bool("destructor").unwrap_or(false) {
            self.functions.push(function);
        } else {
            warn!("Found double destructor annotation for struct {}", self.name);
        }
    }

    pub fn as_opaque(&self) -> OpaqueItem {
        OpaqueItem {
            name: self.name.clone(),
            generic_params: self.generic_params.clone(),
            annotations: self.annotations.clone(),
            documentation: self.documentation.clone(),
        }
    }

    pub fn write_destructor<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        if let Some(ref destructor) = self.destructor {
            if !destructor.extern_decl {
                out.new_line();
                out.new_line();
                // Explicitly disable copy constructor and assignment
                out.write(&format!("{0}(const {0}& ) = delete;", self.name));
                out.new_line();
                out.write(&format!("{0}& operator=(const {0}&) = delete;", self.name));
                out.new_line();
                out.write(&format!("{0}({0}&&) = default;", self.name));
                out.new_line();
                out.write(&format!("{0}& operator=({0}&&) = default;", self.name));
                out.new_line();
                out.new_line();

                out.write(&format!("~{}()", self.name));
                out.open_brace();
                let option_1 = out.measure(|out| format_function_call_1(destructor, out));

                if (config.function.args == Layout::Auto && option_1 <= config.line_length) ||
                    config.function.args == Layout::Horizontal {
                        format_function_call_1(destructor, out);
                    } else {
                        format_function_call_2(destructor, out);
                    }

                out.close_brace(false);
            }
        }
    }

    pub fn write_functions<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        if !self.functions.is_empty() {
            out.new_line();
        }
        for f in &self.functions {
            if f.extern_decl {
                continue;
            }
            out.new_line();
            f.write_formated(config, out, false);
            out.open_brace();
            let option_1 = out.measure(|out| format_function_call_1(f, out));

            if (config.function.args == Layout::Auto && option_1 <= config.line_length) ||
                config.function.args == Layout::Horizontal {
                    format_function_call_1(f, out);
                } else {
                    format_function_call_2(f, out);
                }

            out.close_brace(false);
            out.new_line();
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

        out.write_vertical_source_list(&self.fields, ListType::Cap(";"));

        if config.language == Language::Cxx {
            let mut wrote_start_newline = false;

            let other = if let Some(r) = config.function.rename_args {
                r.apply_to_snake_case("other", IdentifierType::FunctionArg)
            } else {
                String::from("other")
            };

            self.write_destructor(config, out);
            self.write_functions(config, out);

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

fn format_function_call_1<W: Write>(f: &Function, out: &mut SourceWriter<W>) {
    if f.ret == Type::Primitive(PrimitiveType::Void) {
        out.write("::");
    } else {
        out.write("return ::");
    }
    out.write(&f.name);
    out.write("(this");
    for &(ref name, _) in &f.args {
        out.write(", ");
        out.write(name);
    }
    out.write(");");
}

fn format_function_call_2<W: Write>(f: &Function, out: &mut SourceWriter<W>) {
    if f.ret == Type::Primitive(PrimitiveType::Void) {
        out.write("::");
    } else {
        out.write("return ::");
    }
    out.write(&f.name);
    out.write("(");
    let align_lenght = out.line_length_for_align();
    out.push_set_spaces(align_lenght);
    out.write("this");
    for &(ref name, _) in &f.args {
        out.write(",");
        out.new_line();
        out.write(name);
    }
    out.pop_tab();
    out.write(");");
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
