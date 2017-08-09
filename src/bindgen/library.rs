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
use std::fs;

use syn;

use bindgen::cargo::Cargo;
use bindgen::config::{self, Config, Language};
use bindgen::annotation::*;
use bindgen::ir::*;
use bindgen::rust_lib;
use bindgen::utilities::*;
use bindgen::writer::{ListType, Source, SourceWriter};

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

pub type MonomorphList = BTreeMap<Vec<Type>, Monomorph>;
pub type Monomorphs = BTreeMap<PathRef, MonomorphList>;

/// A dependency list is used for gathering what order to output the types.
pub struct DependencyList {
    pub order: Vec<Item>,
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

    fn add_std_types(&mut self) {
        let mut add_opaque = |name: &str, generic_params: Vec<&str>| {
            self.opaque_items.insert(name.to_owned(), OpaqueItem {
                name: name.to_owned(),
                generic_params: generic_params.iter()
                                              .map(|x| (*x).to_owned())
                                              .collect(),
                annotations: AnnotationSet::new(),
                documentation: Documentation::none(),
            })
        };

        add_opaque("String", vec![]);
        add_opaque("Box", vec!["T"]);
        add_opaque("Rc", vec!["T"]);
        add_opaque("Arc", vec!["T"]);
        add_opaque("Result", vec!["T", "E"]);
        add_opaque("Option", vec!["T"]);
        add_opaque("Vec", vec!["T"]);
        add_opaque("HashMap", vec!["K", "V"]);
        add_opaque("BTreeMap", vec!["K", "V"]);
        add_opaque("HashSet", vec!["T"]);
        add_opaque("BTreeSet", vec!["T"]);
        add_opaque("LinkedList", vec!["T"]);
        add_opaque("VecDeque", vec!["T"]);
    }

    /// Parse the specified crate or source file and load #[repr(C)] types for binding generation.
    pub fn load_src(src: &path::Path,
                    config: &Config) -> Result<Library, String>
    {
        let mut library = Library::blank("", config);
        library.add_std_types();

        rust_lib::parse_src(src, &mut |crate_name, items| {
            library.load_syn_crate_mod(&crate_name, items);
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
            library.load_syn_crate_mod(&crate_name, items);
        })?;

        Ok(library)
    }

    fn load_syn_crate_mod(&mut self, crate_name: &str, items: &Vec<syn::Item>) {
        for item in items {
            match item.node {
                syn::ItemKind::ForeignMod(ref block) => {
                    self.load_syn_foreign_mod(crate_name, item, block);
                }
                syn::ItemKind::Fn(ref decl,
                                  ref _unsafe,
                                  ref _const,
                                  ref abi,
                                  ref _generic,
                                  ref _block) => {
                    self.load_syn_fn(crate_name, item, decl, abi);
                }
                syn::ItemKind::Struct(ref variant, ref generics) => {
                    self.load_syn_struct(crate_name, item, variant, generics);
                }
                syn::ItemKind::Enum(ref variants, ref generics) => {
                    self.load_syn_enum(crate_name, item, variants, generics);
                }
                syn::ItemKind::Ty(ref ty, ref generics) => {
                    self.load_syn_ty(crate_name, item, ty, generics);
                }
                _ => { }
            }
        }
    }

    /// Enters a `extern "C" { }` declaration and loads function declarations.
    fn load_syn_foreign_mod(&mut self,
                            crate_name: &str,
                            item: &syn::Item,
                            block: &syn::ForeignMod) {
        if !block.abi.is_c() {
            info!("skip {}::{} - (extern block must be extern C)", crate_name, &item.ident);
            return;
        }

        for foreign_item in &block.items {
            match foreign_item.node {
                syn::ForeignItemKind::Fn(ref decl,
                                         ref _generic) => {
                    if crate_name != self.bindings_crate_name {
                        info!("skip {}::{} - (fn's outside of the binding crate are not used)",
                              crate_name,
                              &foreign_item.ident);
                        return;
                    }

                    let annotations = match AnnotationSet::parse(foreign_item.get_doc_attr()) {
                        Ok(x) => x,
                        Err(msg) => {
                            warn!("{}", msg);
                            AnnotationSet::new()
                        }
                    };

                    match Function::load(foreign_item.ident.to_string(),
                                         annotations,
                                         decl,
                                         true,
                                         foreign_item.get_doc_attr()) {
                        Ok(func) => {
                            info!("take {}::{}", crate_name, &foreign_item.ident);

                            self.functions.insert(func.name.clone(), func);
                        }
                        Err(msg) => {
                            error!("Cannot use fn {}::{} ({})",
                                   crate_name,
                                   &foreign_item.ident,
                                   msg);
                        },
                    }
                }
                _ => {}
            }
        }
    }

    /// Loads a `fn` declaration
    fn load_syn_fn(&mut self,
                   crate_name: &str,
                   item: &syn::Item,
                   decl: &syn::FnDecl,
                   abi: &Option<syn::Abi>) {
        if crate_name != self.bindings_crate_name {
            info!("skip {}::{} - (fn's outside of the binding crate are not used)",
                  crate_name,
                  &item.ident);
            return;
        }

        if item.is_no_mangle() && abi.is_c() {
            let annotations = match AnnotationSet::parse(item.get_doc_attr()) {
                Ok(x) => x,
                Err(msg) => {
                    warn!("{}", msg);
                    AnnotationSet::new()
                }
            };

            match Function::load(item.ident.to_string(),
                                 annotations,
                                 decl,
                                 false,
                                 item.get_doc_attr()) {
                Ok(func) => {
                    info!("take {}::{}", crate_name, &item.ident);

                    self.functions.insert(func.name.clone(), func);
                }
                Err(msg) => {
                    error!("cannot use fn {}::{} ({})",
                           crate_name,
                           &item.ident,
                           msg);
                },
            }
        } else {
            if item.is_no_mangle() != abi.is_c() {
                warn!("skip {}::{} - (not both `no_mangle` and `extern \"C\"`)",
                      crate_name,
                      &item.ident);
            }
        }
    }

    /// Loads a `struct` declaration
    fn load_syn_struct(&mut self,
                       crate_name: &str,
                       item: &syn::Item,
                       variant: &syn::VariantData,
                       generics: &syn::Generics) {
        let struct_name = item.ident.to_string();
        let annotations = match AnnotationSet::parse(item.get_doc_attr()) {
            Ok(x) => x,
            Err(msg) => {
                warn!("{}", msg);
                AnnotationSet::new()
            }
        };

        if item.is_repr_c() {
            match Struct::load(struct_name.clone(),
                               annotations.clone(),
                               variant,
                               generics,
                               item.get_doc_attr()) {
                Ok(st) => {
                    info!("take {}::{}", crate_name, &item.ident);
                    self.structs.insert(struct_name,
                                        st);
                }
                Err(msg) => {
                    info!("take {}::{} - opaque ({})",
                          crate_name,
                          &item.ident,
                          msg);
                    self.opaque_items.insert(struct_name.clone(),
                                             OpaqueItem::new(struct_name,
                                                             generics,
                                                             annotations,
                                                             item.get_doc_attr()));
                }
            }
        } else {
            info!("take {}::{} - opaque (not marked as repr(C))",
                  crate_name,
                  &item.ident);
            self.opaque_items.insert(struct_name.clone(),
                                     OpaqueItem::new(struct_name,
                                                     generics,
                                                     annotations,
                                                     item.get_doc_attr()));
        }
    }

    /// Loads a `enum` declaration
    fn load_syn_enum(&mut self,
                     crate_name: &str,
                     item: &syn::Item,
                     variants: &Vec<syn::Variant>,
                     generics: &syn::Generics) {
        if !generics.lifetimes.is_empty() ||
           !generics.ty_params.is_empty() ||
           !generics.where_clause.predicates.is_empty() {
            info!("skip {}::{} - (has generics or lifetimes or where bounds)",
                  crate_name,
                  &item.ident);
            return;
        }

        let enum_name = item.ident.to_string();
        let annotations = match AnnotationSet::parse(item.get_doc_attr()) {
            Ok(x) => x,
            Err(msg) => {
                warn!("{}", msg);
                AnnotationSet::new()
            }
        };

        match Enum::load(enum_name.clone(),
                         item.get_repr(),
                         annotations.clone(),
                         variants,
                         item.get_doc_attr()) {
            Ok(en) => {
                info!("take {}::{}", crate_name, &item.ident);
                self.enums.insert(enum_name, en);
            }
            Err(msg) => {
                info!("take {}::{} - opaque ({})", crate_name, &item.ident, msg);
                self.opaque_items.insert(enum_name.clone(),
                                         OpaqueItem::new(enum_name,
                                                         generics,
                                                         annotations,
                                                         item.get_doc_attr()));
            }
        }
    }

    /// Loads a `type` declaration
    fn load_syn_ty(&mut self,
                   crate_name: &str,
                   item: &syn::Item,
                   ty: &syn::Ty,
                   generics: &syn::Generics) {
        let alias_name = item.ident.to_string();
        let annotations = match AnnotationSet::parse(item.get_doc_attr()) {
            Ok(x) => x,
            Err(msg) => {
                warn!("{}", msg);
                AnnotationSet::new()
            }
        };


        let fail1 = if generics.lifetimes.is_empty() &&
                       generics.ty_params.is_empty()
        {
            match Typedef::load(alias_name.clone(),
                                annotations.clone(),
                                ty,
                                item.get_doc_attr())
            {
                Ok(typedef) => {
                    info!("take {}::{}", crate_name, &item.ident);
                    self.typedefs.insert(alias_name, typedef);
                    return;
                }
                Err(msg) => msg,
            }
        } else {
            format!("cannot have generics in typedef")
        };

        let fail2 = match Specialization::load(alias_name.clone(),
                                               annotations.clone(),
                                               generics,
                                               ty,
                                               item.get_doc_attr()) {
            Ok(spec) => {
                info!("take {}::{}", crate_name, &item.ident);
                self.specializations.insert(alias_name, spec);
                return;
            }
            Err(msg) => msg,
        };

        info!("skip {}::{} - ({} and {})",
              crate_name,
              &item.ident,
              fail1,
              fail2);
    }

    pub fn insert_item(&mut self, item: Item) {
        match item {
            Item::OpaqueItem(x) => { self.opaque_items.insert(x.name.clone(), x); },
            Item::Struct(x) => { self.structs.insert(x.name.clone(), x); },
            Item::Enum(x) => { self.enums.insert(x.name.clone(), x); },
            Item::Typedef(x) => { self.typedefs.insert(x.name.clone(), x); },
            Item::Specialization(x) => { self.specializations.insert(x.name.clone(), x); },
        };
    }

    pub fn resolve_path(&self, p: &PathRef) -> Option<Item> {
        if let Some(x) = self.enums.get(p) {
            return Some(Item::Enum(x.clone()));
        }
        if let Some(x) = self.structs.get(p) {
            return Some(Item::Struct(x.clone()));
        }
        if let Some(x) = self.opaque_items.get(p) {
            return Some(Item::OpaqueItem(x.clone()));
        }
        if let Some(x) = self.typedefs.get(p) {
            return Some(Item::Typedef(x.clone()));
        }
        if let Some(x) = self.specializations.get(p) {
            return Some(Item::Specialization(x.clone()));
        }

        None
    }

    /// Build a bindings file from this rust library.
    pub fn generate(mut self) -> Result<GeneratedBindings, String> {
        let mut result = GeneratedBindings::blank(&self.config);

        // Specialize 'specialization' items into new items and remove the
        // 'specialization' items
        self.specialize_items();

        // Transfer all typedef annotations to the type they alias
        let mut typedef_annotations = HashMap::new();
        for (_, ref mut typedef) in &mut self.typedefs {
            typedef.transfer_annotations(&mut typedef_annotations);
        }
        for (alias_path, annotations) in typedef_annotations.drain() {
            // TODO
            if let Some(x) = self.enums.get_mut(&alias_path) {
                if !x.annotations.is_empty() {
                    warn!("can't transfer annotations from typedef to alias ({}) that already has annotations.",
                          alias_path);
                    continue;
                }
                x.annotations = annotations;
                continue;
            }
            if let Some(x) = self.structs.get_mut(&alias_path) {
                if !x.annotations.is_empty() {
                    warn!("can't transfer annotations from typedef to alias ({}) that already has annotations.",
                          alias_path);
                    continue;
                }
                x.annotations = annotations;
                continue;
            }
            if let Some(x) = self.opaque_items.get_mut(&alias_path) {
                if !x.annotations.is_empty() {
                    warn!("can't transfer annotations from typedef to alias ({}) that already has annotations.",
                          alias_path);
                    continue;
                }
                x.annotations = annotations;
                continue;
            }
            if let Some(x) = self.typedefs.get_mut(&alias_path) {
                if !x.annotations.is_empty() {
                    warn!("can't transfer annotations from typedef to alias ({}) that already has annotations.",
                          alias_path);
                    continue;
                }
                x.annotations = annotations;
                continue;
            }
            if let Some(x) = self.specializations.get_mut(&alias_path) {
                if !x.annotations.is_empty() {
                    warn!("can't transfer annotations from typedef to alias ({}) that already has annotations.",
                          alias_path);
                    continue;
                }
                x.annotations = annotations;
                continue;
            }
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
                &Item::Struct(ref s) => {
                    if s.generic_params.len() != 0 {
                        if let Some(monomorphs) = monomorphs.get(&s.name) {
                            for (_, monomorph) in monomorphs {
                                result.items.push(match monomorph {
                                    &Monomorph::Struct(ref x) => {
                                        Item::Struct(x.clone())
                                    }
                                    &Monomorph::OpaqueItem(ref x) => {
                                        Item::OpaqueItem(x.clone())
                                    }
                                });
                            }
                        }
                        continue;
                    } else {
                        debug_assert!(!monomorphs.contains_key(&s.name));
                    }
                }
                &Item::OpaqueItem(ref s) => {
                    if s.generic_params.len() != 0 {
                        if let Some(monomorphs) = monomorphs.get(&s.name) {
                            for (_, monomorph) in monomorphs {
                                result.items.push(match monomorph {
                                    &Monomorph::Struct(ref x) => {
                                        Item::Struct(x.clone())
                                    }
                                    &Monomorph::OpaqueItem(ref x) => {
                                        Item::OpaqueItem(x.clone())
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
        let ordering = |a: &Item, b: &Item| {
            match (a, b) {
                (&Item::Enum(ref e1), &Item::Enum(ref e2)) => e1.name.cmp(&e2.name),
                (&Item::Enum(_), _) => Ordering::Less,
                (_, &Item::Enum(_)) => Ordering::Greater,

                (&Item::OpaqueItem(ref o1), &Item::OpaqueItem(ref o2)) => o1.name.cmp(&o2.name),
                (&Item::OpaqueItem(_), _) => Ordering::Less,
                (_, &Item::OpaqueItem(_)) => Ordering::Greater,

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
            let mut new_monomorph_set = BTreeMap::new();
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

    fn specialize_items(&mut self) {
        let mut specializations = Vec::new();

        for (_, specialization) in &self.specializations {
            match specialization.specialize(&self) {
                Ok(Some(specialization)) => {
                    specializations.push(specialization);
                }
                Ok(None) => { }
                Err(msg) => {
                    warn!("specializing {} failed - ({})", specialization.name.clone(), msg);
                }
            }
        }

        for specialization in specializations {
            self.insert_item(specialization);
        }

        self.specializations.clear();
    }
}

/// A GeneratedBindings is a completed bindings file ready to be written.
#[derive(Debug, Clone)]
pub struct GeneratedBindings {
    config: Config,

    monomorphs: Monomorphs,
    items: Vec<Item>,
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
        if let Some(parent) = path::Path::new(path).parent() {
            fs::create_dir_all(parent).unwrap();
        }

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
                &Item::Enum(ref x) => x.write(&self.config, &mut out),
                &Item::Struct(ref x) => x.write(&self.config, &mut out),
                &Item::OpaqueItem(ref x) => x.write(&self.config, &mut out),
                &Item::Typedef(ref x) => x.write(&self.config, &mut out),
                &Item::Specialization(_) => {
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
            let mut specialization = Vec::new();
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
                // Collect all specializations and print them after theall generic versions are generated
                // This is needed because the specilizations could have dependencies to each other
                specialization.push((path, monomorph_sets));
            }

            for (path, monomorph_sets) in specialization {
                for (generic_values, monomorph) in monomorph_sets {
                    out.new_line();
                    out.write("template<>");
                    out.new_line();
                    out.write(&format!("struct {}<", path));
                    out.write_horizontal_source_list(generic_values, ListType::Join(", "));
                    out.write(&format!("> : public {}", monomorph.name()));
                    out.open_brace();
                    out.close_brace(true);
                    out.new_line();
                }
            }
        }

        if self.config.language == Language::Cxx {
            out.new_line_if_not_start();
            out.write("static_assert(sizeof(float) == 4);");
            out.new_line();
            out.write("static_assert(sizeof(double) == 8);");
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
