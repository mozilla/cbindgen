/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::Ordering;
use std::fs::File;
use std::path;

use syn;

use bindgen::cargo::Cargo;
use bindgen::config::{self, Config, Language};
use bindgen::annotation::*;
use bindgen::items::*;
use bindgen::rust_lib;
use bindgen::utilities::*;
use bindgen::writer::{ListType, Source, SourceWriter};

/// A path ref is used to reference a path value
pub type PathRef = String;

/// A path value is any type of rust item besides a function
#[derive(Debug, Clone)]
pub enum PathValue {
    Enum(Enum),
    Struct(Struct),
    OpaqueItem(OpaqueItem),
    Typedef(Typedef),
    Specialization(Specialization),
}

impl PathValue {
    pub fn name(&self) -> &str {
        match self {
            &PathValue::Enum(ref x) => { &x.name },
            &PathValue::Struct(ref x) => { &x.name },
            &PathValue::OpaqueItem(ref x) => { &x.name },
            &PathValue::Typedef(ref x) => { &x.name },
            &PathValue::Specialization(ref x) => { &x.name },
        }
    }

    pub fn add_deps(&self, library: &Library, out: &mut DependencyList) {
        match self {
            &PathValue::Struct(ref x) => {
                x.add_deps(library, out);
            },
            &PathValue::Typedef(ref x) => {
                x.add_deps(library, out);
            },
            &PathValue::Specialization(..) => {
                unreachable!();
            },
            _ => { }
        }
    }

    pub fn add_specializations(&self, library: &Library, out: &mut SpecializationList) {
        match self {
            &PathValue::Struct(ref x) => {
                x.add_specializations(library, out);
            },
            &PathValue::Typedef(ref x) => {
                x.add_specializations(library, out);
            },
            &PathValue::Specialization(ref x) => {
                x.add_specializations(library, out);
            },
            _ => { }
        }
    }

    pub fn rename_fields(&mut self, config: &Config) {
        match self {
            &mut PathValue::Enum(ref mut x) => { x.rename_fields(config); },
            &mut PathValue::Struct(ref mut x) => { x.rename_fields(config); },
            _ => { },
        }
    }

    pub fn mangle_paths(&mut self, monomorphs: &Monomorphs) {
        match self {
            &mut PathValue::Enum(_) => { },
            &mut PathValue::Struct(ref mut x) => {
                x.mangle_paths(monomorphs);
            },
            &mut PathValue::OpaqueItem(_) => { },
            &mut PathValue::Typedef(ref mut x) => {
                x.mangle_paths(monomorphs);
            },
            &mut PathValue::Specialization(..) => {
                unreachable!();
            },
        }
    }
}

#[derive(Clone, Debug)]
pub enum Monomorph {
    Struct(Struct),
    OpaqueItem(OpaqueItem),
}
impl Monomorph {
    pub fn name(&self) -> &str {
        match self {
            &Monomorph::Struct(ref x) => &x.name,
            &Monomorph::OpaqueItem(ref x) => &x.name,
        }
    }

    pub fn is_opaque(&self) -> bool {
        match self {
            &Monomorph::Struct(_) => false,
            &Monomorph::OpaqueItem(_) => true,
        }
    }
}

pub type MonomorphList = HashMap<Vec<Type>, Monomorph>;
pub type Monomorphs = HashMap<PathRef, MonomorphList>;

/// A dependency list is used for gathering what order to output the types.
pub struct DependencyList {
    pub order: Vec<PathValue>,
    pub items: HashSet<PathRef>,
}

impl DependencyList {
    fn new() -> DependencyList {
        DependencyList {
            order: Vec::new(),
            items: HashSet::new(),
        }
    }
}

/// A specialization list is used for gathering what order to output the types.
pub struct SpecializationList {
    pub order: Vec<PathValue>,
    pub items: HashSet<PathRef>,
    pub errors: Vec<(String, String)>,
}

impl SpecializationList {
    fn new() -> SpecializationList {
        SpecializationList {
            order: Vec::new(),
            items: HashSet::new(),
            errors: Vec::new(),
        }
    }
}

/// A library contains all of the information needed to generate bindings for a rust library.
#[derive(Debug, Clone)]
pub struct Library {
    bindings_crate_name: String,
    config: Config,

    enums: BTreeMap<String, Enum>,
    structs: BTreeMap<String, Struct>,
    opaque_items: BTreeMap<String, OpaqueItem>,
    typedefs: BTreeMap<String, Typedef>,
    specializations: BTreeMap<String, Specialization>,
    functions: BTreeMap<String, Function>,
}

impl Library {
    fn blank(bindings_crate_name: &str, config: &Config) -> Library {
        Library {
            bindings_crate_name: String::from(bindings_crate_name),
            config: config.clone(),

            enums: BTreeMap::new(),
            structs: BTreeMap::new(),
            opaque_items: BTreeMap::new(),
            typedefs: BTreeMap::new(),
            specializations: BTreeMap::new(),
            functions: BTreeMap::new(),
        }
    }

    /// Parse the specified crate or source file and load #[repr(C)] types for binding generation.
    pub fn load_src(src: &path::Path,
                    config: &Config) -> Result<Library, String>
    {
        let mut library = Library::blank("", config);

        library.add_std_types();

        rust_lib::parse_src(src, &mut |crate_name, items| {
            library.load_from_crate_mod(&crate_name, items);
        })?;

        Ok(library)
    }

    /// Parse the specified crate or source file and load #[repr(C)] types for binding generation.
    pub fn load_crate(lib: Cargo,
                      config: &Config) -> Result<Library, String>
    {
        let mut library = Library::blank(lib.binding_crate_name(),
                                         config);

        library.add_std_types();

        rust_lib::parse_lib(lib,
                            config.parse.parse_deps,
                            &config.parse.include,
                            &config.parse.exclude,
                            &config.parse.expand,
                            &mut |crate_name, items| {
            library.load_from_crate_mod(&crate_name, items);
        })?;

        Ok(library)
    }

    fn add_std_types(&mut self) {
        // TODO
        self.opaque_items.insert("String".to_owned(), OpaqueItem {
            name: "String".to_owned(),
            generic_params: vec![],
            annotations: AnnotationSet::new(),
        });

        self.opaque_items.insert("Box".to_owned(), OpaqueItem {
            name: "Box".to_owned(),
            generic_params: vec!["T".to_owned()],
            annotations: AnnotationSet::new(),
        });

        self.opaque_items.insert("Rc".to_owned(), OpaqueItem {
            name: "Rc".to_owned(),
            generic_params: vec!["T".to_owned()],
            annotations: AnnotationSet::new(),
        });

        self.opaque_items.insert("Arc".to_owned(), OpaqueItem {
            name: "Arc".to_owned(),
            generic_params: vec!["T".to_owned()],
            annotations: AnnotationSet::new(),
        });

        self.opaque_items.insert("Result".to_owned(), OpaqueItem {
            name: "Result".to_owned(),
            generic_params: vec!["T".to_owned(),
                                 "E".to_owned()],
            annotations: AnnotationSet::new(),
        });

        self.opaque_items.insert("Option".to_owned(), OpaqueItem {
            name: "Option".to_owned(),
            generic_params: vec!["T".to_owned()],
            annotations: AnnotationSet::new(),
        });

        self.opaque_items.insert("Vec".to_owned(), OpaqueItem {
            name: "Vec".to_owned(),
            generic_params: vec!["T".to_owned()],
            annotations: AnnotationSet::new(),
        });

        self.opaque_items.insert("HashMap".to_owned(), OpaqueItem {
            name: "HashMap".to_owned(),
            generic_params: vec!["K".to_owned(),
                                 "V".to_owned()],
            annotations: AnnotationSet::new(),
        });

        self.opaque_items.insert("BTreeMap".to_owned(), OpaqueItem {
            name: "BTreeMap".to_owned(),
            generic_params: vec!["K".to_owned(),
                                 "V".to_owned()],
            annotations: AnnotationSet::new(),
        });

        self.opaque_items.insert("HashSet".to_owned(), OpaqueItem {
            name: "HashSet".to_owned(),
            generic_params: vec!["T".to_owned()],
            annotations: AnnotationSet::new(),
        });

        self.opaque_items.insert("BTreeSet".to_owned(), OpaqueItem {
            name: "BTreeSet".to_owned(),
            generic_params: vec!["T".to_owned()],
            annotations: AnnotationSet::new(),
        });

        self.opaque_items.insert("LinkedList".to_owned(), OpaqueItem {
            name: "LinkedList".to_owned(),
            generic_params: vec!["T".to_owned()],
            annotations: AnnotationSet::new(),
        });

        self.opaque_items.insert("VecDeque".to_owned(), OpaqueItem {
            name: "VecDeque".to_owned(),
            generic_params: vec!["T".to_owned()],
            annotations: AnnotationSet::new(),
        });
    }

    fn load_from_crate_mod(&mut self, crate_name: &str, items: &Vec<syn::Item>) {
        for item in items {
            match item.node {
                syn::ItemKind::ForeignMod(ref block) => {
                    if !block.abi.is_c() {
                        info!("skip {}::{} - (extern block must be extern C)", crate_name, &item.ident);
                        continue;
                    }

                    for foreign_item in &block.items {
                        match foreign_item.node {
                            syn::ForeignItemKind::Fn(ref decl,
                                                     ref _generic) => {
                                if crate_name != self.bindings_crate_name {
                                    info!("skip {}::{} - (fn's outside of the binding crate are not used)", crate_name, &foreign_item.ident);
                                    continue;
                                }

                                let annotations = match AnnotationSet::parse(foreign_item.get_doc_attr()) {
                                    Ok(x) => x,
                                    Err(msg) => {
                                        warn!("{}", msg);
                                        AnnotationSet::new()
                                    }
                                };

                                match Function::load(foreign_item.ident.to_string(), annotations, decl, true) {
                                    Ok(func) => {
                                        info!("take {}::{}", crate_name, &foreign_item.ident);

                                        self.functions.insert(func.name.clone(), func);
                                    }
                                    Err(msg) => {
                                        error!("Cannot use fn {}::{} ({})", crate_name, &foreign_item.ident, msg);
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
                    if crate_name != self.bindings_crate_name {
                        info!("skip {}::{} - (fn's outside of the binding crate are not used)", crate_name, &item.ident);
                        continue;
                    }

                    if item.is_no_mangle() && abi.is_c() {
                        let annotations = match AnnotationSet::parse(item.get_doc_attr()) {
                            Ok(x) => x,
                            Err(msg) => {
                                warn!("{}", msg);
                                AnnotationSet::new()
                            }
                        };

                        match Function::load(item.ident.to_string(), annotations, decl, false) {
                            Ok(func) => {
                                info!("take {}::{}", crate_name, &item.ident);

                                self.functions.insert(func.name.clone(), func);
                            }
                            Err(msg) => {
                                error!("Cannot use fn {}::{} ({})", crate_name, &item.ident, msg);
                            },
                        }
                    } else {
                        if item.is_no_mangle() != abi.is_c() {
                            warn!("skip {}::{} - (not both `no_mangle` and `extern \"C\"`)", crate_name, &item.ident);
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
                        match Struct::load(struct_name.clone(), annotations.clone(), variant, generics) {
                            Ok(st) => {
                                info!("take {}::{}", crate_name, &item.ident);
                                self.structs.insert(struct_name,
                                                    st);
                            }
                            Err(msg) => {
                                info!("take {}::{} - opaque ({})", crate_name, &item.ident, msg);
                                self.opaque_items.insert(struct_name.clone(),
                                                           OpaqueItem::new(struct_name,
                                                                             generics,
                                                                             annotations));
                            }
                        }
                    } else {
                        info!("take {}::{} - opaque (not marked as repr(C))", crate_name, &item.ident);
                        self.opaque_items.insert(struct_name.clone(),
                                                   OpaqueItem::new(struct_name,
                                                                     generics,
                                                                     annotations));
                    }
                }
                syn::ItemKind::Enum(ref variants, ref generics) => {
                    if !generics.lifetimes.is_empty() ||
                       !generics.ty_params.is_empty() ||
                       !generics.where_clause.predicates.is_empty() {
                        info!("skip {}::{} - (has generics or lifetimes or where bounds)", crate_name, &item.ident);
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

                    match Enum::load(enum_name.clone(), item.get_repr(), annotations.clone(), variants) {
                        Ok(en) => {
                            info!("take {}::{}", crate_name, &item.ident);
                            self.enums.insert(enum_name, en);
                        }
                        Err(msg) => {
                            info!("take {}::{} - opaque ({})", crate_name, &item.ident, msg);
                            self.opaque_items.insert(enum_name.clone(),
                                                       OpaqueItem::new(enum_name,
                                                                         generics,
                                                                         annotations));
                        }
                    }
                }
                syn::ItemKind::Ty(ref ty, ref generics) => {
                    let alias_name = item.ident.to_string();
                    let annotations = match AnnotationSet::parse(item.get_doc_attr()) {
                        Ok(x) => x,
                        Err(msg) => {
                            warn!("{}", msg);
                            AnnotationSet::new()
                        }
                    };

                    let fail1 = match Specialization::load(alias_name.clone(),
                                                           annotations.clone(),
                                                           generics,
                                                           ty) {
                        Ok(spec) => {
                            info!("take {}::{}", crate_name, &item.ident);
                            self.specializations.insert(alias_name, spec);
                            continue;
                        }
                        Err(msg) => msg,
                    };

                    if !generics.lifetimes.is_empty() ||
                       !generics.ty_params.is_empty() {
                        info!("skip {}::{} - (cannot have generics in typedef)", crate_name, &item.ident);
                        continue;
                    }

                    let fail2 = match Typedef::load(alias_name.clone(),
                                            annotations,
                                            ty)
                    {
                        Ok(typedef) => {
                            info!("take {}::{}", crate_name, &item.ident);
                            self.typedefs.insert(alias_name, typedef);
                            continue;
                        }
                        Err(msg) => msg,
                    };

                    info!("skip {}::{} - ({} and {})", crate_name, &item.ident, fail1, fail2);
                }
                _ => {}
            }
        }
    }

    pub fn resolve_path(&self, p: &PathRef) -> Option<PathValue> {
        if let Some(x) = self.enums.get(p) {
            return Some(PathValue::Enum(x.clone()));
        }
        if let Some(x) = self.structs.get(p) {
            return Some(PathValue::Struct(x.clone()));
        }
        if let Some(x) = self.opaque_items.get(p) {
            return Some(PathValue::OpaqueItem(x.clone()));
        }
        if let Some(x) = self.typedefs.get(p) {
            return Some(PathValue::Typedef(x.clone()));
        }
        if let Some(x) = self.specializations.get(p) {
            return Some(PathValue::Specialization(x.clone()));
        }

        None
    }

    /// Build a bindings file from this rust library.
    pub fn generate(mut self) -> Result<GeneratedBindings, String> {
        let mut result = GeneratedBindings::blank(&self.config);

        // Specialize types into new types and remove all the specializations
        // that are left behind
        let mut specializations = SpecializationList::new();
        for (_, function) in &self.functions {
            function.add_specializations(&self, &mut specializations);
        }
        self.specializations.clear();

        for specialization in specializations.order {
            let name = specialization.name().to_owned();
            match specialization {
                PathValue::Struct(x) => {
                    self.structs.insert(name, x);
                }
                PathValue::OpaqueItem(x) => {
                    self.opaque_items.insert(name, x);
                }
                PathValue::Enum(x) => {
                    self.enums.insert(name, x);
                }
                PathValue::Typedef(x) => {
                    self.typedefs.insert(name, x);
                }
                PathValue::Specialization(..) => {
                    unreachable!();
                }
            }
        }
        for (path, error) in specializations.errors {
            warn!("specializing {} failed - ({})", path, error);
        }

        // Gather only the items that we need for this
        // `extern "c"` interface
        let mut deps = DependencyList::new();
        for (_, function) in &self.functions {
            function.add_deps(&self, &mut deps);
        }

        // Gather a list of all the instantiations of generic structs
        // TODO - monomorphs of a single type are not sorted by dependencies
        let mut monomorphs = Monomorphs::new();
        for (_, function) in &self.functions {
            function.add_monomorphs(&self, &mut monomorphs);
        }

        // Copy the binding items in dependencies order into the generated bindings,
        // adding any instantiations of generic structs along the way.
        for dep in deps.order {
            match &dep {
                &PathValue::Struct(ref s) => {
                    if s.generic_params.len() != 0 {
                        if let Some(monomorphs) = monomorphs.get(&s.name) {
                            for (_, monomorph) in monomorphs {
                                result.items.push(match monomorph {
                                    &Monomorph::Struct(ref x) => {
                                        PathValue::Struct(x.clone())
                                    }
                                    &Monomorph::OpaqueItem(ref x) => {
                                        PathValue::OpaqueItem(x.clone())
                                    }
                                });
                            }
                        }
                        continue;
                    } else {
                        debug_assert!(!monomorphs.contains_key(&s.name));
                    }
                }
                &PathValue::OpaqueItem(ref s) => {
                    if s.generic_params.len() != 0 {
                        if let Some(monomorphs) = monomorphs.get(&s.name) {
                            for (_, monomorph) in monomorphs {
                                result.items.push(match monomorph {
                                    &Monomorph::Struct(ref x) => {
                                        PathValue::Struct(x.clone())
                                    }
                                    &Monomorph::OpaqueItem(ref x) => {
                                        PathValue::OpaqueItem(x.clone())
                                    }
                                });
                            }
                        }
                        continue;
                    } else {
                        debug_assert!(!monomorphs.contains_key(&s.name));
                    }
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

                (&PathValue::OpaqueItem(ref o1), &PathValue::OpaqueItem(ref o2)) => o1.name.cmp(&o2.name),
                (&PathValue::OpaqueItem(_), _) => Ordering::Less,
                (_, &PathValue::OpaqueItem(_)) => Ordering::Greater,

                _ => Ordering::Equal,
            }
        };
        result.items.sort_by(ordering);

        result.functions = self.functions.iter()
                                         .map(|(_, function)| function.clone())
                                         .collect::<Vec<_>>();

        // Rename all the fields according to their rules and mangle any
        // paths that refer to generic structs that have been monomorphed.
        for item in &mut result.items {
            item.mangle_paths(&monomorphs);
            item.rename_fields(&self.config);
        }

        // Rename all the arguments according to their rules and mangle any
        // paths that refer to generic structs that have been monomorph'ed.
        for func in &mut result.functions {
            func.mangle_paths(&monomorphs);
            func.rename_args(&self.config);
        }

        // The bindings writing code uses information about the monomorphs
        // to write out utility template specializations. We ideally should
        // send a different data structure. Currently we reuse the existing one,
        // but unfortunately the generic values have types that are not
        // mangled and need to be. So we build a copy and mangle along the way.
        // TODO
        let mut new_monomorphs = Monomorphs::new();
        for (path, monomorph_set) in monomorphs.iter() {
            let mut new_monomorph_set = HashMap::new();
            for (generic_values, monomorph) in monomorph_set.iter() {
                let mut new_generic_values = generic_values.clone();
                for generic_value in &mut new_generic_values {
                    generic_value.mangle_paths(&monomorphs);
                }
                new_monomorph_set.insert(new_generic_values, monomorph.clone());
            }
            new_monomorphs.insert(path.clone(), new_monomorph_set);
        }
        result.monomorphs = new_monomorphs;

        Ok(result)
    }
}

/// A GeneratedBindings is a completed bindings file ready to be written.
#[derive(Debug, Clone)]
pub struct GeneratedBindings {
    config: Config,

    monomorphs: Monomorphs,
    items: Vec<PathValue>,
    functions: Vec<Function>,
}

impl GeneratedBindings {
    fn blank(config: &Config) -> GeneratedBindings {
        GeneratedBindings {
            config: config.clone(),
            monomorphs: Monomorphs::new(),
            items: Vec::new(),
            functions: Vec::new(),
        }
    }

    pub fn write_to_file(&self, path: &str) {
        self.write(File::create(path).unwrap());
    }

    pub fn write<F: Write>(&self, file: F) {
        let mut out = SourceWriter::new(file, &self.config);

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
        if self.config.language == Language::C {
            out.write("#include <stdint.h>");
            out.new_line();
            out.write("#include <stdlib.h>");
            out.new_line();
            out.write("#include <stdbool.h>");
        } else {
            out.write("#include <cstdint>");
            out.new_line();
            out.write("#include <cstdlib>");
        }
        out.new_line();

        if self.config.language == Language::Cxx {
            out.new_line_if_not_start();
            out.write("extern \"C\" {");
            out.new_line();

            let mut wrote_namespace: bool = false;
            if let Some(ref namespace) = self.config.namespace {
                wrote_namespace = true;

                out.new_line();
                out.write("namespace ");
                out.write(namespace);
                out.write(" {");
            }
            if let Some(ref namespaces) = self.config.namespaces {
                wrote_namespace = true;
                for namespace in namespaces {
                    out.new_line();
                    out.write("namespace ");
                    out.write(namespace);
                    out.write(" {");
                }
            }
            if wrote_namespace {
                out.new_line();
            }
        }

        for item in &self.items {
            out.new_line_if_not_start();
            match item {
                &PathValue::Enum(ref x) => x.write(&self.config, &mut out),
                &PathValue::Struct(ref x) => x.write(&self.config, &mut out),
                &PathValue::OpaqueItem(ref x) => x.write(&self.config, &mut out),
                &PathValue::Typedef(ref x) => x.write(&self.config, &mut out),
                &PathValue::Specialization(_) => {
                    unreachable!("should not encounter a specialization in a generated library")
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
            function.write(&self.config, &mut out);
            out.new_line();
        }

        if self.config.language == Language::Cxx {
            let mut wrote_namespace: bool = false;
            if let Some(ref namespaces) = self.config.namespaces {
                wrote_namespace = true;

                for namespace in namespaces.iter().rev() {
                    out.new_line_if_not_start();
                    out.write("} // namespace ");
                    out.write(namespace);
                }
            }
            if let Some(ref namespace) = self.config.namespace {
                wrote_namespace = true;

                out.new_line_if_not_start();
                out.write("} // namespace ");
                out.write(namespace);
            }
            if wrote_namespace {
                out.new_line();
            }

            out.new_line_if_not_start();
            out.write("} // extern \"C\"");
            out.new_line();
        }

        if self.config.structure.generic_template_specialization &&
           self.config.language == Language::Cxx {
            for (path, monomorph_sets) in &self.monomorphs {
                if monomorph_sets.len() == 0 {
                    continue;
                }

                // TODO
                let is_opaque = monomorph_sets.iter().next().unwrap().1.is_opaque();
                let generics_count = monomorph_sets.iter().next().unwrap().0.len();
                let generics_names = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M",
                                      "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];

                if is_opaque || generics_count > 26 {
                    continue;
                }

                out.new_line_if_not_start();
                out.write("template<");
                for i in 0..generics_count {
                    if i != 0 {
                        out.write(", ")
                    }
                    out.write("typename ");
                    out.write(generics_names[i]);
                }
                out.write(">");
                out.new_line();
                out.write(&format!("struct {}", path));
                out.open_brace();
                out.close_brace(true);
                out.new_line();

                for (generic_values, monomorph) in monomorph_sets {
                    out.new_line();
                    out.write("template<>");
                    out.new_line();
                    out.write(&format!("struct {}<", path));
                    out.write_horizontal_source_list(generic_values, ListType::Join(", "));
                    out.write(&format!("> : {}", monomorph.name()));
                    out.open_brace();
                    out.close_brace(true);
                    out.new_line();
                }
            }
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
