/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs::File;
use std::mem;
use std::path;
use std::fs;

use syn;

use bindgen::cargo::Cargo;
use bindgen::config::{Config, Language};
use bindgen::dependencies::Dependencies;
use bindgen::ir::{AnnotationSet, Documentation, Enum, Function};
use bindgen::ir::{Item, OpaqueItem, Path, Specialization, Struct, Typedef};
use bindgen::monomorph::{Monomorphs, TemplateSpecialization};
use bindgen::rust_lib;
use bindgen::utilities::{SynAbiHelpers, SynItemHelpers};
use bindgen::writer::{ListType, Source, SourceWriter};

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

    template_specializations: Vec<TemplateSpecialization>,
}

impl Library {
    fn new(bindings_crate_name: &str, config: &Config) -> Library {
        Library {
            bindings_crate_name: String::from(bindings_crate_name),
            config: config.clone(),

            enums: BTreeMap::new(),
            structs: BTreeMap::new(),
            opaque_items: BTreeMap::new(),
            typedefs: BTreeMap::new(),
            specializations: BTreeMap::new(),
            functions: BTreeMap::new(),

            template_specializations: Vec::new(),
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
        let mut library = Library::new("", config);
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
        let mut library = Library::new(lib.binding_crate_name(),
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

    pub fn resolve_path(&self, p: &Path) -> Option<Item> {
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
        // Transfer all typedef annotations to the type they alias
        self.transfer_annotations();

        // Rename internal parts of items according to rename rules
        self.rename_item_internals();

        // Specialize and remove 'specialization' items
        self.specialize_items();

        // Instantiate monomorphs for each generic path
        self.instantiate_monomorphs();

        let mut result = GeneratedBindings::new(&self.config);

        // Gather only the items that we need for this `extern "c"` interface
        let mut deps = Dependencies::new();

        for (_, function) in &self.functions {
            function.add_dependencies(&self, &mut deps);
        }

        if self.config.structure.generic_template_specialization &&
           self.config.language == Language::Cxx {
            for template_specialization in &self.template_specializations {
              template_specialization.add_dependencies(&self, &mut deps);
            }
        }

        deps.sort();

        result.items = deps.order;
        result.functions = self.functions.values().map(|x| x.clone()).collect();
        result.template_specializations = mem::replace(&mut self.template_specializations, Vec::new());

        Ok(result)
    }

    fn transfer_annotations(&mut self) {
        let mut annotations = HashMap::new();

        for (_, ref mut typedef) in &mut self.typedefs {
            typedef.transfer_annotations(&mut annotations);
        }

        for (alias_path, annotations) in annotations {
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
    }

    fn rename_item_internals(&mut self) {
        for item in self.structs.values_mut() {
            item.rename_fields(&self.config);
        }

        for item in self.enums.values_mut() {
            item.rename_values(&self.config);
        }

        for item in self.functions.values_mut() {
            item.rename_args(&self.config);
        }
    }

    fn specialize_items(&mut self) {
        let mut specializations = Vec::new();

        for specialization in self.specializations.values() {
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

    fn instantiate_monomorphs(&mut self) {
      assert!(self.specializations.len() == 0);

      let mut monomorphs = Monomorphs::new();

      for x in self.structs.values() {
        x.add_monomorphs(self, &mut monomorphs);
      }
      for x in self.typedefs.values() {
        x.add_monomorphs(self, &mut monomorphs);
      }
      for x in self.functions.values() {
        x.add_monomorphs(self, &mut monomorphs);
      }

      for monomorph in monomorphs.drain_structs() {
        self.structs.insert(monomorph.name.clone(), monomorph);
      }
      for monomorph in monomorphs.drain_opaques() {
        self.opaque_items.insert(monomorph.name.clone(), monomorph);
      }

      let opaque_items = mem::replace(&mut self.opaque_items, BTreeMap::new());
      for (path, item) in opaque_items {
        if item.generic_params.len() != 0 {
          continue;
        }
        self.opaque_items.insert(path, item);
      }

      let structs = mem::replace(&mut self.structs, BTreeMap::new());
      for (path, item) in structs {
        if item.generic_params.len() != 0 {
          continue;
        }
        self.structs.insert(path, item);
      }

      for x in self.structs.values_mut() {
        x.mangle_paths(&monomorphs);
      }
      for x in self.typedefs.values_mut() {
        x.mangle_paths(&monomorphs);
      }
      for x in self.functions.values_mut() {
        x.mangle_paths(&monomorphs);
      }

      self.template_specializations = monomorphs.drain_template_specializations();
    }
}

/// A GeneratedBindings is a completed bindings file ready to be written.
pub struct GeneratedBindings {
    config: Config,
    items: Vec<Item>,
    functions: Vec<Function>,
    template_specializations: Vec<TemplateSpecialization>,
}

impl GeneratedBindings {
    fn new(config: &Config) -> GeneratedBindings {
        GeneratedBindings {
            config: config.clone(),
            items: Vec::new(),
            functions: Vec::new(),
            template_specializations: Vec::new(),
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
            out.write(&format!("/* Generated with cbindgen:{} */",
                      ::bindgen::config::VERSION));
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
          for template in &self.template_specializations {
            out.new_line_if_not_start();
            out.write("template<");
            for (i, param) in template.generic.generic_params.iter().enumerate() {
                if i != 0 {
                    out.write(", ")
                }
                out.write("typename ");
                out.write(param);
            }
            out.write(">");
            out.new_line();
            out.write(&format!("struct {};", template.generic.name));
            out.new_line();

            for &(ref monomorph_path, ref generic_values) in &template.monomorphs {
              out.new_line();
              out.write("template<>");
              out.new_line();
              out.write(&format!("struct {}<", template.generic.name));
              out.write_horizontal_source_list(generic_values, ListType::Join(", "));
              out.write(&format!("> : public {}", monomorph_path));
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
