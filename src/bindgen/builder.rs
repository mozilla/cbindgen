/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::BTreeMap;
use std::mem;
use std::path;

use syn;

use bindgen::cargo::Cargo;
use bindgen::config::Config;
use bindgen::ir::{AnnotationSet, Documentation, Enum, Function};
use bindgen::ir::{OpaqueItem, Specialization, Struct, Typedef};
use bindgen::library::Library;
use bindgen::rust_lib;
use bindgen::utilities::{SynAbiHelpers, SynItemHelpers};

#[derive(Debug, Clone)]
pub struct LibraryBuilder {
    config: Config,
    srcs: Vec<path::PathBuf>,
    lib: Option<Cargo>,
    enums: BTreeMap<String, Enum>,
    structs: BTreeMap<String, Struct>,
    opaque_items: BTreeMap<String, OpaqueItem>,
    typedefs: BTreeMap<String, Typedef>,
    specializations: BTreeMap<String, Specialization>,
    functions: BTreeMap<String, Function>,
}

impl LibraryBuilder {
    pub fn new() -> LibraryBuilder {
        LibraryBuilder {
            config: Config::default(),
            srcs: Vec::new(),
            lib: None,
            enums: BTreeMap::new(),
            structs: BTreeMap::new(),
            opaque_items: BTreeMap::new(),
            typedefs: BTreeMap::new(),
            specializations: BTreeMap::new(),
            functions: BTreeMap::new(),
        }
    }

    pub fn with_config(mut self, config: Config) -> LibraryBuilder {
        self.config = config;
        self
    }

    pub fn with_std_types(mut self) -> LibraryBuilder {
        {
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

        self
    }

    pub fn with_src(mut self, src: &path::Path) -> LibraryBuilder {
        self.srcs.push(src.to_owned());
        self
    }

    pub fn with_crate(mut self, lib: Cargo) -> LibraryBuilder {
        debug_assert!(self.lib.is_none());
        self.lib = Some(lib);
        self
    }

    pub fn build(mut self) -> Result<Library, String> {
        // Workaround the borrowchecker
        let srcs = mem::replace(&mut self.srcs, Vec::new());
        let lib = mem::replace(&mut self.lib, None);
        let config = self.config.clone();

        for x in &srcs {
            rust_lib::parse_src(x, &mut |crate_name, items| {
                self.load_syn_crate_mod("", &crate_name, items);
            })?;
        }

        if let Some(x) = lib {
            rust_lib::parse_lib(x,
                                config.parse.parse_deps,
                                &config.parse.include,
                                &config.parse.exclude,
                                &config.parse.expand,
                                &mut |binding_crate_name, crate_name, items| {
                self.load_syn_crate_mod(binding_crate_name, &crate_name, items);
            })?;
        }

        Ok(Library::new(self.config,
                        self.enums,
                        self.structs,
                        self.opaque_items,
                        self.typedefs,
                        self.specializations,
                        self.functions))
    }

    fn load_syn_crate_mod(&mut self, binding_crate_name: &str, crate_name: &str, items: &Vec<syn::Item>) {
        for item in items {
            match item.node {
                syn::ItemKind::ForeignMod(ref block) => {
                    self.load_syn_foreign_mod(binding_crate_name, crate_name, item, block);
                }
                syn::ItemKind::Fn(ref decl,
                                  ref _unsafe,
                                  ref _const,
                                  ref abi,
                                  ref _generic,
                                  ref _block) => {
                    self.load_syn_fn(binding_crate_name, crate_name, item, decl, abi);
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
                            binding_crate_name: &str,
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
                    if crate_name != binding_crate_name {
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
                   binding_crate_name: &str,
                   crate_name: &str,
                   item: &syn::Item,
                   decl: &syn::FnDecl,
                   abi: &Option<syn::Abi>) {
        if crate_name != binding_crate_name {
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
}
