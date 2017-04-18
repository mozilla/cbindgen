use std::io::Write;
use std::collections::BTreeMap;
use std::cmp::Ordering;
use std::fs::File;

use syn::*;

use bindgen::config;
use bindgen::config::Config;
use bindgen::directive::*;
use bindgen::items::*;
use bindgen::rust_lib;
use bindgen::syn_helpers::*;

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
}

/// A library collects all of the information needed to generate
/// bindings for a specified rust library. It is turned into a
/// GeneratedLibrary, and in the process filters out unneeded information
/// and in the future will do validation.
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

    pub fn load(crate_or_src: &str, config: &'a Config) -> Library<'a>
    {
        let mut library = Library::blank(config);

        rust_lib::parse(crate_or_src, &mut |mod_name, items| {
            for item in items {
                match item.node {
                    ItemKind::Fn(ref decl,
                                 ref _unsafe,
                                 ref _const,
                                 ref abi,
                                 ref _generic,
                                 ref _block) => {
                        if item.is_no_mangle() && abi.is_c() {
                            let directives = match Directive::parse(item.get_doc_attr()) {
                                Ok(x) => x,
                                Err(msg) => {
                                    warn!("{}", msg);
                                    vec![]
                                }
                            };

                            match Function::convert(item.ident.to_string(), directives, decl) {
                                Ok(func) => {
                                    info!("take {}::{}", mod_name, &item.ident);

                                    library.functions.insert(func.name.clone(), func);
                                }
                                Err(msg) => {
                                    info!("skip {}::{} - ({})", mod_name, &item.ident, msg);
                                },
                            }
                        }
                    }
                    ItemKind::Struct(ref variant,
                                     ref generics) => {
                        let struct_name = item.ident.to_string();
                        let directives = match Directive::parse(item.get_doc_attr()) {
                            Ok(x) => x,
                            Err(msg) => {
                                warn!("{}", msg);
                                vec![]
                            }
                        };

                        if item.is_repr_c() {
                            match Struct::convert(struct_name.clone(), directives.clone(), variant, generics) {
                                Ok(st) => {
                                    info!("take {}::{}", mod_name, &item.ident);
                                    library.structs.insert(struct_name,
                                                           st);
                                }
                                Err(msg) => {
                                    info!("take {}::{} - opaque ({})", mod_name, &item.ident, msg);
                                    library.opaque_structs.insert(struct_name.clone(),
                                                                  OpaqueStruct::new(struct_name, directives));
                                }
                            }
                        } else {
                            info!("take {}::{} - opaque (not marked as repr(C))", mod_name, &item.ident);
                            library.opaque_structs.insert(struct_name.clone(),
                                                          OpaqueStruct::new(struct_name, directives));
                        }
                    }
                    ItemKind::Enum(ref variants, ref generics) => {
                        if !generics.lifetimes.is_empty() ||
                           !generics.ty_params.is_empty() ||
                           !generics.where_clause.predicates.is_empty() {
                            info!("skip {}::{} - (has generics or lifetimes or where bounds)", mod_name, &item.ident);
                            continue;
                        }

                        let enum_name = item.ident.to_string();
                        let directives = match Directive::parse(item.get_doc_attr()) {
                            Ok(x) => x,
                            Err(msg) => {
                                warn!("{}", msg);
                                vec![]
                            }
                        };

                        match Enum::convert(enum_name.clone(), item.get_repr(), directives.clone(), variants) {
                            Ok(en) => {
                                info!("take {}::{}", mod_name, &item.ident);
                                library.enums.insert(enum_name, en);
                            }
                            Err(msg) => {
                                info!("take {}::{} - opaque ({})", mod_name, &item.ident, msg);
                                library.opaque_structs.insert(enum_name.clone(),
                                                              OpaqueStruct::new(enum_name, directives));
                            }
                        }
                    }
                    ItemKind::Ty(ref ty, ref generics) => {
                        if !generics.lifetimes.is_empty() ||
                           !generics.ty_params.is_empty() ||
                           !generics.where_clause.predicates.is_empty() {
                            info!("skip {}::{} - (has generics or lifetimes or where bounds)", mod_name, &item.ident);
                            continue;
                        }

                        let alias_name = item.ident.to_string();
                        let directives = match Directive::parse(item.get_doc_attr()) {
                            Ok(x) => x,
                            Err(msg) => {
                                warn!("{}", msg);
                                vec![]
                            }
                        };

                        let fail1 = match Specialization::convert(alias_name.clone(), directives.clone(), ty) {
                            Ok(spec) => {
                                info!("take {}::{}", mod_name, &item.ident);
                                library.specializations.insert(alias_name, spec);
                                continue;
                            }
                            Err(msg) => msg,
                        };
                        let fail2 = match Typedef::convert(alias_name.clone(), directives, ty) {
                            Ok(typedef) => {
                                info!("take {}::{}", mod_name, &item.ident);
                                library.typedefs.insert(alias_name, typedef);
                                continue;
                            }
                            Err(msg) => msg,
                        };
                        info!("skip {}::{} - ({} and {})", mod_name, &item.ident, fail1, fail2);
                    }
                    _ => {}
                }
            }
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

    pub fn generate(self) -> GenerateResult<GeneratedLibrary<'a>> {
        let mut result = GeneratedLibrary::blank(self.config);

        // Gather only the items that we need for this
        // `extern "c"` interface
        let mut deps = Vec::new();
        for (_, function) in &self.functions {
            function.add_deps(&self, &mut deps);
        }

        // Copy the binding items in dependencies order
        // into the GeneratedLibrary, specializing any type
        // aliases we encounter
        for dep in deps {
            match &dep {
                &PathValue::Struct(ref s) => {
                    if !s.generic_params.is_empty() {
                        continue;
                    }
                }
                &PathValue::Specialization(ref s) => {
                    match s.specialize(&self) {
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

        // Bring the enums all the way to the top because they
        // don't depend on anyone else, and it makes the output
        // nicer
        result.items.sort_by(|a, b| {
            match (a, b) {
                (&PathValue::Enum(ref e1), &PathValue::Enum(ref e2)) => e1.name.cmp(&e2.name),
                (&PathValue::Enum(_), _) => Ordering::Less,
                (_, &PathValue::Enum(_)) => Ordering::Greater,
                _ => Ordering::Equal,
            }
        });

        result.functions = self.functions.iter()
                                         .map(|(_, function)| function.clone())
                                         .collect::<Vec<_>>();

        Ok(result)
    }
}

/// A GeneratedLibrary represents a completed bindings file ready to be printed.
#[derive(Debug, Clone)]
pub struct GeneratedLibrary<'a> {
    config: &'a Config,

    items: Vec<PathValue>,
    functions: Vec<Function>,
}

impl<'a> GeneratedLibrary<'a> {
    fn blank(config: &'a Config) -> GeneratedLibrary<'a> {
        GeneratedLibrary {
            config: config,
            items: Vec::new(),
            functions: Vec::new(),
        }
    }

    pub fn write_to_file(&self, path: &str) {
        self.write(&mut File::create(path).unwrap());
    }

    pub fn write<F: Write>(&self, out: &mut F) {
        if let Some(ref f) = self.config.file_header {
            write!(out, "{}\n", f).unwrap();
        }
        if self.config.file_include_version {
            write!(out, "\n/* Generated with cbindgen:{} */\n", config::VERSION).unwrap();
        }
        if let Some(ref f) = self.config.file_autogen_warning {
            write!(out, "\n{}\n", f).unwrap();
        }

        for item in &self.items {
            write!(out, "\n").unwrap();
            match item {
                &PathValue::Enum(ref x) => x.write(self.config, out),
                &PathValue::Struct(ref x) => x.write(self.config, out),
                &PathValue::OpaqueStruct(ref x) => x.write(out),
                &PathValue::Typedef(ref x) => x.write(out),
                &PathValue::Specialization(_) => {
                    panic!("should not encounter a specialization in a built library")
                }
            }
            write!(out, "\n").unwrap();
        }

        if let Some(ref f) = self.config.file_autogen_warning {
            write!(out, "\n{}\n", f).unwrap();
        }

        for function in &self.functions {
            write!(out, "\n").unwrap();
            function.write(self.config, out);
            write!(out, "\n").unwrap();
        }

        if let Some(ref f) = self.config.file_autogen_warning {
            write!(out, "\n{}\n", f).unwrap();
        }
        if let Some(ref f) = self.config.file_trailer {
            write!(out, "\n{}\n", f).unwrap();
        }
    }
}
