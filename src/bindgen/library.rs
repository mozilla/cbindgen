use std::io::Write;
use std::collections::BTreeMap;
use std::cmp::Ordering;
use std::fs::File;

use syn;

use bindgen::config;
use bindgen::config::{Config, Language};
use bindgen::annotation::*;
use bindgen::items::*;
use bindgen::rust_lib;
use bindgen::utilities::*;
use bindgen::writer::Writer;

pub type ConvertResult<T> = Result<T, String>;
pub type GenerateResult<T> = Result<T, String>;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Repr {
    None,
    C,
    U8,
    U16,
    U32,
}

pub type PathRef = String;
#[derive(Debug, Clone)]
pub enum PathValue {
    Enum(Enum),
    Struct(Struct),
    OpaqueStruct(OpaqueStruct),
    Typedef(Typedef),
    Specialization(Specialization),
}
impl PathValue {
    pub fn name(&self) -> &String {
        match self {
            &PathValue::Enum(ref x) => { &x.name },
            &PathValue::Struct(ref x) => { &x.name },
            &PathValue::OpaqueStruct(ref x) => { &x.name },
            &PathValue::Typedef(ref x) => { &x.name },
            &PathValue::Specialization(ref x) => { &x.name },
        }
    }

    pub fn add_deps(&self, library: &Library, out: &mut Vec<PathValue>) {
        match self {
            &PathValue::Enum(_) => { },
            &PathValue::Struct(ref x) => { x.add_deps(library, out); },
            &PathValue::OpaqueStruct(_) => { },
            &PathValue::Typedef(ref x) => { x.add_deps(library, out); },
            &PathValue::Specialization(ref x) => { x.add_deps(library, out); },
        }
    }

    pub fn apply_renaming(&mut self, config: &Config) {
        match self {
            &mut PathValue::Enum(ref mut x) => { x.apply_renaming(config); },
            &mut PathValue::Struct(ref mut x) => { x.apply_renaming(config); },
            _ => { },
        }
    }
}

/// A library contains all of the information needed to generate bindings for a rust library.
#[derive(Debug, Clone)]
pub struct Library<'a> {
    config: &'a Config,

    enums: BTreeMap<String, Enum>,
    structs: BTreeMap<String, Struct>,
    opaque_structs: BTreeMap<String, OpaqueStruct>,
    typedefs: BTreeMap<String, Typedef>,
    specializations: BTreeMap<String, Specialization>,
    functions: BTreeMap<String, Function>,
}

impl<'a> Library<'a> {
    fn blank(config: &'a Config) -> Library<'a> {
        Library {
            config: config,
            enums: BTreeMap::new(),
            structs: BTreeMap::new(),
            opaque_structs: BTreeMap::new(),
            typedefs: BTreeMap::new(),
            specializations: BTreeMap::new(),
            functions: BTreeMap::new(),
        }
    }

    fn parse_crate_mod(&mut self, mod_name: &str, items: &Vec<syn::Item>) {
        for item in items {
            match item.node {
                syn::ItemKind::ForeignMod(ref block) => {
                    if !block.abi.is_c() {
                        info!("skip {}::{} - non c abi extern block", mod_name, &item.ident);
                        continue;
                    }

                    for foreign_item in &block.items {
                        match foreign_item.node {
                            syn::ForeignItemKind::Fn(ref decl,
                                                     ref _generic) => {
                                let annotations = match AnnotationSet::parse(foreign_item.get_doc_attr()) {
                                    Ok(x) => x,
                                    Err(msg) => {
                                        warn!("{}", msg);
                                        AnnotationSet::new()
                                    }
                                };

                                match Function::convert(foreign_item.ident.to_string(), annotations, decl, true) {
                                    Ok(func) => {
                                        info!("take {}::{}", mod_name, &foreign_item.ident);

                                        self.functions.insert(func.name.clone(), func);
                                    }
                                    Err(msg) => {
                                        info!("skip {}::{} - ({})", mod_name, &foreign_item.ident, msg);
                                    },
                                }
                            }
                            _ => {}
                        }
                    }
                }
                syn::ItemKind::Fn(ref decl,
                                  ref _unsafe,
                                  ref _const,
                                  ref abi,
                                  ref _generic,
                                  ref _block) => {
                    if item.is_no_mangle() && abi.is_c() {
                        let annotations = match AnnotationSet::parse(item.get_doc_attr()) {
                            Ok(x) => x,
                            Err(msg) => {
                                warn!("{}", msg);
                                AnnotationSet::new()
                            }
                        };

                        match Function::convert(item.ident.to_string(), annotations, decl, false) {
                            Ok(func) => {
                                info!("take {}::{}", mod_name, &item.ident);

                                self.functions.insert(func.name.clone(), func);
                            }
                            Err(msg) => {
                                info!("skip {}::{} - ({})", mod_name, &item.ident, msg);
                            },
                        }
                    }
                }
                syn::ItemKind::Struct(ref variant,
                                      ref generics) => {
                    let struct_name = item.ident.to_string();
                    let annotations = match AnnotationSet::parse(item.get_doc_attr()) {
                        Ok(x) => x,
                        Err(msg) => {
                            warn!("{}", msg);
                            AnnotationSet::new()
                        }
                    };

                    if item.is_repr_c() {
                        match Struct::convert(struct_name.clone(), annotations.clone(), variant, generics) {
                            Ok(st) => {
                                info!("take {}::{}", mod_name, &item.ident);
                                self.structs.insert(struct_name,
                                                    st);
                            }
                            Err(msg) => {
                                info!("take {}::{} - opaque ({})", mod_name, &item.ident, msg);
                                self.opaque_structs.insert(struct_name.clone(),
                                                           OpaqueStruct::new(struct_name, annotations));
                            }
                        }
                    } else {
                        info!("take {}::{} - opaque (not marked as repr(C))", mod_name, &item.ident);
                        self.opaque_structs.insert(struct_name.clone(),
                                                   OpaqueStruct::new(struct_name, annotations));
                    }
                }
                syn::ItemKind::Enum(ref variants, ref generics) => {
                    if !generics.lifetimes.is_empty() ||
                       !generics.ty_params.is_empty() ||
                       !generics.where_clause.predicates.is_empty() {
                        info!("skip {}::{} - (has generics or lifetimes or where bounds)", mod_name, &item.ident);
                        continue;
                    }

                    let enum_name = item.ident.to_string();
                    let annotations = match AnnotationSet::parse(item.get_doc_attr()) {
                        Ok(x) => x,
                        Err(msg) => {
                            warn!("{}", msg);
                            AnnotationSet::new()
                        }
                    };

                    match Enum::convert(enum_name.clone(), item.get_repr(), annotations.clone(), variants) {
                        Ok(en) => {
                            info!("take {}::{}", mod_name, &item.ident);
                            self.enums.insert(enum_name, en);
                        }
                        Err(msg) => {
                            info!("take {}::{} - opaque ({})", mod_name, &item.ident, msg);
                            self.opaque_structs.insert(enum_name.clone(),
                                                       OpaqueStruct::new(enum_name, annotations));
                        }
                    }
                }
                syn::ItemKind::Ty(ref ty, ref generics) => {
                    if !generics.lifetimes.is_empty() ||
                       !generics.ty_params.is_empty() ||
                       !generics.where_clause.predicates.is_empty() {
                        info!("skip {}::{} - (has generics or lifetimes or where bounds)", mod_name, &item.ident);
                        continue;
                    }

                    let alias_name = item.ident.to_string();
                    let annotations = match AnnotationSet::parse(item.get_doc_attr()) {
                        Ok(x) => x,
                        Err(msg) => {
                            warn!("{}", msg);
                            AnnotationSet::new()
                        }
                    };

                    let fail1 = match Specialization::convert(alias_name.clone(), annotations.clone(), ty) {
                        Ok(spec) => {
                            info!("take {}::{}", mod_name, &item.ident);
                            self.specializations.insert(alias_name, spec);
                            continue;
                        }
                        Err(msg) => msg,
                    };
                    let fail2 = match Typedef::convert(alias_name.clone(), annotations, ty) {
                        Ok(typedef) => {
                            info!("take {}::{}", mod_name, &item.ident);
                            self.typedefs.insert(alias_name, typedef);
                            continue;
                        }
                        Err(msg) => msg,
                    };
                    info!("skip {}::{} - ({} and {})", mod_name, &item.ident, fail1, fail2);
                }
                _ => {}
            }
        }
    }

    /// Parse the specified crate or source file and load #[repr(C)] types for binding generation.
    pub fn load(crate_or_src: &str, config: &'a Config) -> Library<'a>
    {
        let mut library = Library::blank(config);

        rust_lib::parse(crate_or_src, &mut |mod_name, items| {
            library.parse_crate_mod(&mod_name, items);
        });

        library
    }

    pub fn resolve_path(&self, p: &PathRef) -> Option<PathValue> {
        if let Some(x) = self.enums.get(p) {
            return Some(PathValue::Enum(x.clone()));
        }
        if let Some(x) = self.structs.get(p) {
            return Some(PathValue::Struct(x.clone()));
        }
        if let Some(x) = self.opaque_structs.get(p) {
            return Some(PathValue::OpaqueStruct(x.clone()));
        }
        if let Some(x) = self.typedefs.get(p) {
            return Some(PathValue::Typedef(x.clone()));
        }
        if let Some(x) = self.specializations.get(p) {
            return Some(PathValue::Specialization(x.clone()));
        }

        None
    }

    pub fn add_deps_for_path(&self, p: &PathRef, out: &mut Vec<PathValue>) {
        if let Some(value) = self.resolve_path(p) {
            value.add_deps(self, out);

            if !out.iter().any(|x| x.name() == value.name()) {
                out.push(value);
            }
        } else {
            warn!("can't find {}", p);
        }
    }

    pub fn add_deps_for_path_deps(&self, p: &PathRef, out: &mut Vec<PathValue>) {
        if let Some(value) = self.resolve_path(p) {
            value.add_deps(self, out);
        } else {
            warn!("can't find {}", p);
        }
    }

    /// Build a bindings file from this rust library.
    pub fn generate(self) -> GenerateResult<BuiltBindings<'a>> {
        let mut result = BuiltBindings::blank(self.config);

        // Gather only the items that we need for this
        // `extern "c"` interface
        let mut deps = Vec::new();
        for (_, function) in &self.functions {
            function.add_deps(&self, &mut deps);
        }

        // Copy the binding items in dependencies order
        // into the BuiltBindings, specializing any type
        // aliases we encounter
        for dep in deps {
            match &dep {
                &PathValue::Struct(ref s) => {
                    if !s.generic_params.is_empty() {
                        continue;
                    }
                }
                &PathValue::Specialization(ref s) => {
                    match s.specialize(self.config, &self) {
                        Ok(value) => {
                            result.items.push(value);
                        }
                        Err(msg) => {
                            warn!("specializing {} failed - ({})", dep.name(), msg);
                        }
                    }
                    continue;
                }
                _ => { }
            }
            result.items.push(dep);
        }

        // Sort enums and opaque structs into their own layers because they don't
        // depend on each other or anything else.
        let ordering = |a: &PathValue, b: &PathValue| {
            match (a, b) {
                (&PathValue::Enum(ref e1), &PathValue::Enum(ref e2)) => e1.name.cmp(&e2.name),
                (&PathValue::Enum(_), _) => Ordering::Less,
                (_, &PathValue::Enum(_)) => Ordering::Greater,

                (&PathValue::OpaqueStruct(ref o1), &PathValue::OpaqueStruct(ref o2)) => o1.name.cmp(&o2.name),
                (&PathValue::OpaqueStruct(_), _) => Ordering::Less,
                (_, &PathValue::OpaqueStruct(_)) => Ordering::Greater,

                _ => Ordering::Equal,
            }
        };
        result.items.sort_by(ordering);

        result.functions = self.functions.iter()
                                         .map(|(_, function)| function.clone())
                                         .collect::<Vec<_>>();

        // Do one last pass to do renaming for all the items
        for item in &mut result.items {
            item.apply_renaming(self.config);
        }
        for func in &mut result.functions {
            func.apply_renaming(self.config);
        }

        Ok(result)
    }
}

/// A BuiltBindings is a completed bindings file ready to be written.
#[derive(Debug, Clone)]
pub struct BuiltBindings<'a> {
    config: &'a Config,

    items: Vec<PathValue>,
    functions: Vec<Function>,
}

impl<'a> BuiltBindings<'a> {
    fn blank(config: &'a Config) -> BuiltBindings<'a> {
        BuiltBindings {
            config: config,
            items: Vec::new(),
            functions: Vec::new(),
        }
    }

    pub fn write_to_file(&self, path: &str) {
        self.write(&mut File::create(path).unwrap());
    }

    pub fn write<F: Write>(&self, file: &mut F) {
        let mut out = Writer::new(file, self.config);

        if let Some(ref f) = self.config.header {
            out.new_line_if_not_start();
            out.write(&f);
            out.new_line();
        }
        if let Some(ref f) = self.config.include_guard {
            out.new_line_if_not_start();
            out.write(&format!("#ifndef {}", f));
            out.new_line();
            out.write(&format!("#define {}", f));
            out.new_line();
        }
        if self.config.include_version {
            out.new_line_if_not_start();
            out.write(&format!("/* Generated with cbindgen:{} */", config::VERSION));
            out.new_line();
        }
        if let Some(ref f) = self.config.autogen_warning {
            out.new_line_if_not_start();
            out.write(&f);
            out.new_line();
        }

        out.new_line_if_not_start();
        out.write("#include <stdint.h>");
        if self.config.language == Language::C {
            out.new_line();
            out.write("#include <stdbool.h>");
        }
        out.new_line();

        if self.config.language == Language::Cxx {
            out.new_line_if_not_start();
            out.write("extern \"C\" {");
            out.new_line();
        }

        for item in &self.items {
            out.new_line_if_not_start();
            match item {
                &PathValue::Enum(ref x) => x.write(self.config, &mut out),
                &PathValue::Struct(ref x) => x.write(self.config, &mut out),
                &PathValue::OpaqueStruct(ref x) => x.write(self.config, &mut out),
                &PathValue::Typedef(ref x) => x.write(&mut out),
                &PathValue::Specialization(_) => {
                    panic!("should not encounter a specialization in a built library")
                }
            }
            out.new_line();
        }

        if let Some(ref f) = self.config.autogen_warning {
            out.new_line_if_not_start();
            out.write(&f);
            out.new_line();
        }

        for function in &self.functions {
            if function.extern_decl {
                continue;
            }

            out.new_line_if_not_start();
            function.write(self.config, &mut out);
            out.new_line();
        }

        if self.config.language == Language::Cxx {
            out.new_line_if_not_start();
            out.write("} // extern \"C\"");
            out.new_line();
        }

        if let Some(ref f) = self.config.autogen_warning {
            out.new_line_if_not_start();
            out.write(&f);
            out.new_line();
        }
        if let Some(ref f) = self.config.include_guard {
            out.new_line_if_not_start();
            out.write(&format!("#endif // {}", f));
            out.new_line();
        }
        if let Some(ref f) = self.config.trailer {
            out.new_line_if_not_start();
            out.write(&f);
            out.new_line();
        }
    }
}
