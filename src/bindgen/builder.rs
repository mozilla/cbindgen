/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::BTreeMap;
use std::mem;
use std::path;

use syn;

use bindgen::cargo::Cargo;
use bindgen::config::Config;
use bindgen::ir::{AnnotationSet, Cfg, Documentation, Enum, Function};
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
    functions: Vec<Function>,
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
            functions: Vec::new(),
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
                    cfg: None,
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
        // Workaround the borrow checker
        let srcs = mem::replace(&mut self.srcs, Vec::new());
        let lib = mem::replace(&mut self.lib, None);
        let config = self.config.clone();

        for x in &srcs {
            rust_lib::parse_src(x, &mut |crate_name, items| {
                self.load_syn_crate_mod("", &crate_name, &None, items);
            })?;
        }

        if let Some(x) = lib {
            rust_lib::parse_lib(x,
                                config.parse.parse_deps,
                                &config.parse.include,
                                &config.parse.exclude,
                                &config.parse.expand,
                                &mut |binding_crate_name, crate_name, mod_cfg, items| {
                self.load_syn_crate_mod(binding_crate_name, &crate_name, &mod_cfg, items);
            })?;
        }

        self.functions.sort_by(|x, y| x.name.cmp(&y.name));

        Ok(Library::new(self.config,
                        self.enums,
                        self.structs,
                        self.opaque_items,
                        self.typedefs,
                        self.specializations,
                        self.functions))
    }

    fn load_syn_crate_mod(&mut self,
                          binding_crate_name: &str,
                          crate_name: &str,
                          mod_cfg: &Option<Cfg>,
                          items: &Vec<syn::Item>) {
        for item in items {
            match item.node {
                syn::ItemKind::ForeignMod(ref block) => {
                    self.load_syn_foreign_mod(binding_crate_name,
                                              crate_name,
                                              mod_cfg,
                                              item,
                                              block);
                }
                syn::ItemKind::Fn(ref decl,
                                  ref _unsafe,
                                  ref _const,
                                  ref abi,
                                  ref _generic,
                                  ref _block) => {
                    self.load_syn_fn(binding_crate_name,
                                     crate_name,
                                     mod_cfg,
                                     item,
                                     decl,
                                     abi);
                }
                syn::ItemKind::Struct(ref variant, ref generics) => {
                    self.load_syn_struct(crate_name, mod_cfg, item, variant, generics);
                }
                syn::ItemKind::Enum(ref variants, ref generics) => {
                    self.load_syn_enum(crate_name, mod_cfg, item, variants, generics);
                }
                syn::ItemKind::Ty(ref ty, ref generics) => {
                    self.load_syn_ty(crate_name, mod_cfg, item, ty, generics);
                }
                _ => { }
            }
        }
    }

    /// Enters a `extern "C" { }` declaration and loads function declarations.
    fn load_syn_foreign_mod(&mut self,
                            binding_crate_name: &str,
                            crate_name: &str,
                            mod_cfg: &Option<Cfg>,
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

                    match Function::load(foreign_item.ident.to_string(),
                                         decl,
                                         true,
                                         &foreign_item.attrs,
                                         mod_cfg) {
                        Ok(func) => {
                            info!("take {}::{}", crate_name, &foreign_item.ident);

                            self.functions.push(func);
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
                   mod_cfg: &Option<Cfg>,
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
            match Function::load(item.ident.to_string(),
                                 decl,
                                 false,
                                 &item.attrs,
                                 mod_cfg) {
                Ok(func) => {
                    info!("take {}::{}", crate_name, &item.ident);

                    self.functions.push(func);
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
                       mod_cfg: &Option<Cfg>,
                       item: &syn::Item,
                       variant: &syn::VariantData,
                       generics: &syn::Generics) {
        let struct_name = item.ident.to_string();

        match Struct::load(struct_name.clone(),
                           variant,
                           generics,
                           &item.attrs,
                           mod_cfg) {
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
                                                         &item.attrs,
                                                         mod_cfg));
            }
        }
    }

    /// Loads a `enum` declaration
    fn load_syn_enum(&mut self,
                     crate_name: &str,
                     mod_cfg: &Option<Cfg>,
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

        match Enum::load(enum_name.clone(),
                         variants,
                         &item.attrs,
                         mod_cfg) {
            Ok(en) => {
                info!("take {}::{}", crate_name, &item.ident);
                self.enums.insert(enum_name, en);
            }
            Err(msg) => {
                info!("take {}::{} - opaque ({})", crate_name, &item.ident, msg);
                self.opaque_items.insert(enum_name.clone(),
                                         OpaqueItem::new(enum_name,
                                                         generics,
                                                         &item.attrs,
                                                         mod_cfg));
            }
        }
    }

    /// Loads a `type` declaration
    fn load_syn_ty(&mut self,
                   crate_name: &str,
                   mod_cfg: &Option<Cfg>,
                   item: &syn::Item,
                   ty: &syn::Ty,
                   generics: &syn::Generics) {
        let alias_name = item.ident.to_string();

        let fail1 = if generics.lifetimes.is_empty() &&
                       generics.ty_params.is_empty()
        {
            match Typedef::load(alias_name.clone(),
                                ty,
                                &item.attrs,
                                mod_cfg)
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
                                               generics,
                                               ty,
                                               &item.attrs,
                                               mod_cfg) {
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
