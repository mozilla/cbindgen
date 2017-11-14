/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use syn;

use bindgen::cargo::{Cargo, PackageRef};
use bindgen::ir::{AnnotationSet, Cfg, Constant, Documentation, Enum, Function};
use bindgen::ir::{ItemMap, OpaqueItem, Specialization, Static, Struct, Typedef, Union};
use bindgen::utilities::{SynAbiHelpers, SynItemHelpers};

const STD_CRATES: &'static [&'static str] = &["std",
                                              "std_unicode",
                                              "alloc",
                                              "collections",
                                              "core",
                                              "proc_macro"];

type ParseResult = Result<Parse, String>;

/// Parses a single rust source file, not following `mod` or `extern crate`.
pub fn parse_src(src_file: &Path) -> ParseResult {
    let mod_name = src_file.file_stem().unwrap().to_str().unwrap();

    let mut context = Parser {
        binding_crate_name: mod_name.to_owned(),
        lib: None,
        parse_deps: true,
        include: None,
        exclude: Vec::new(),
        expand: Vec::new(),
        parsed_crates: HashSet::new(),
        cache_src: HashMap::new(),
        cache_expanded_crate: HashMap::new(),
        cfg_stack: Vec::new(),
        out: Parse::new(),
    };

    let pkg_ref = PackageRef {
        name: mod_name.to_owned(),
        version: "0.0.0".to_owned(),
    };

    context.parse_mod(&pkg_ref, src_file)?;
    Ok(context.out)
}

/// Recursively parses a rust library starting at the root crate's directory.
///
/// Inside a crate, `mod` and `extern crate` declarations are followed
/// and parsed. To find an external crate, the parser uses the `cargo metadata`
/// command to find the location of dependencies.
pub(crate) fn parse_lib(lib: Cargo,
                        parse_deps: bool,
                        include: &Option<Vec<String>>,
                        exclude: &Vec<String>,
                        expand: &Vec<String>) -> ParseResult {
    let mut context = Parser {
        binding_crate_name: lib.binding_crate_name().to_owned(),
        lib: Some(lib),
        parse_deps: parse_deps,
        include: include.clone(),
        exclude: exclude.clone(),
        expand: expand.clone(),
        parsed_crates: HashSet::new(),
        cache_src: HashMap::new(),
        cache_expanded_crate: HashMap::new(),
        cfg_stack: Vec::new(),
        out: Parse::new(),
    };

    let binding_crate = context.lib.as_ref().unwrap().binding_crate_ref();
    context.parse_crate(&binding_crate)?;
    Ok(context.out)
}

#[derive(Debug, Clone)]
struct Parser {
    binding_crate_name: String,
    lib: Option<Cargo>,
    parse_deps: bool,

    include: Option<Vec<String>>,
    exclude: Vec<String>,
    expand: Vec<String>,

    parsed_crates: HashSet<String>,
    cache_src: HashMap<PathBuf, Vec<syn::Item>>,
    cache_expanded_crate: HashMap<String, Vec<syn::Item>>,

    cfg_stack: Vec<Cfg>,

    out: Parse,
}

impl Parser {
    fn should_parse_dependency(&self, pkg_name: &String) -> bool {
        if self.parsed_crates.contains(pkg_name) {
            return false;
        }

        if !self.parse_deps {
            return false;
        }

        // Skip any whitelist or blacklist for expand
        if self.expand.contains(&pkg_name) {
            return true;
        }

        // If we have a whitelist, check it
        if let Some(ref include) = self.include {
            if !include.contains(&pkg_name) {
                return false;
            }
        }

        // Check the blacklist
        return !STD_CRATES.contains(&pkg_name.as_ref()) &&
               !self.exclude.contains(&pkg_name);
    }

    fn parse_crate(&mut self, pkg: &PackageRef) -> Result<(), String> {
        assert!(self.lib.is_some());
        self.parsed_crates.insert(pkg.name.clone());

        // Check if we should use cargo expand for this crate
        if self.expand.contains(&pkg.name) {
            return self.parse_expand_crate(pkg);
        }

        // Otherwise do our normal parse
        let crate_src = self.lib.as_ref().unwrap().find_crate_src(pkg);

        match crate_src {
            Some(crate_src) => {
                self.parse_mod(pkg, crate_src.as_path())
            },
            None => {
                // This should be an error, but is common enough to just elicit a warning
                warn!("Parsing crate `{}`: can't find lib.rs with `cargo metadata`.", pkg.name);
                Ok(())
            },
        }
    }

    fn parse_expand_crate(&mut self, pkg: &PackageRef) -> Result<(), String> {
        assert!(self.lib.is_some());

        let mod_parsed = {
            if !self.cache_expanded_crate.contains_key(&pkg.name) {
                let s = self.lib.as_ref().unwrap().expand_crate(pkg)?;
                let i = syn::parse_crate(&s).map_err(|msg| format!("Parsing crate `{}`:\n{}.", pkg.name, msg))?;
                self.cache_expanded_crate.insert(pkg.name.clone(), i.items);
            }

            self.cache_expanded_crate.get(&pkg.name).unwrap().clone()
        };

        self.process_expanded_mod(pkg, &mod_parsed)
    }

    fn process_expanded_mod(&mut self,
                            pkg: &PackageRef,
                            items: &Vec<syn::Item>) -> Result<(), String> {
        self.out.load_syn_crate_mod(&self.binding_crate_name,
                                    &pkg.name,
                                    &Cfg::join(&self.cfg_stack),
                                    items);

        for item in items {
            match item.node {
                syn::ItemKind::Mod(ref inline_items) => {
                    let cfg = Cfg::load(&item.attrs);
                    if let &Some(ref cfg) = &cfg {
                        self.cfg_stack.push(cfg.clone());
                    }

                    if let &Some(ref inline_items) = inline_items {
                        self.process_expanded_mod(pkg, inline_items)?;
                    } else {
                        error!("Parsing crate `{}`: external mod found in expanded source, this shouldn't be possible.", pkg.name);
                    }

                    if cfg.is_some() {
                        self.cfg_stack.pop();
                    }
                }
                syn::ItemKind::ExternCrate(_) => {
                    let dep_pkg_name = item.ident.to_string();

                    let cfg = Cfg::load(&item.attrs);
                    if let &Some(ref cfg) = &cfg {
                        self.cfg_stack.push(cfg.clone());
                    }

                    if self.should_parse_dependency(&dep_pkg_name) {
                        if self.lib.is_some() {
                            let dep_pkg_ref = self.lib.as_ref().unwrap().find_dep_ref(pkg, &dep_pkg_name);

                            if let Some(dep_pkg_ref) = dep_pkg_ref {
                                self.parse_crate(&dep_pkg_ref)?;
                            } else {
                                error!("Parsing crate `{}`: can't find dependency version for `{}`.", pkg.name, dep_pkg_name);
                            }
                        } else {
                            error!("Parsing crate `{}`: cannot parse external crate `{}` because cbindgen is in single source mode. Consider specifying a crate directory instead of a source file.", pkg.name, dep_pkg_name);
                        }
                    }

                    if cfg.is_some() {
                        self.cfg_stack.pop();
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn parse_mod(&mut self,
                 pkg: &PackageRef,
                 mod_path: &Path) -> Result<(), String> {
        let mod_parsed = {
            let owned_mod_path = mod_path.to_path_buf();

            if !self.cache_src.contains_key(&owned_mod_path) {
                let mut s = String::new();
                let mut f = File::open(mod_path).map_err(|_| format!("Parsing crate `{}`: cannot open file `{:?}`.", pkg.name, mod_path))?;
                f.read_to_string(&mut s).map_err(|_| format!("Parsing crate `{}`: cannot open file `{:?}`.", pkg.name, mod_path))?;
                let i = syn::parse_crate(&s).map_err(|msg| format!("Parsing crate `{}`:\n{}.", pkg.name, msg))?;
                self.cache_src.insert(owned_mod_path.clone(), i.items);
            }

            self.cache_src.get(&owned_mod_path).unwrap().clone()
        };

        let mod_dir = mod_path.parent().unwrap();

        self.process_mod(pkg,
                         mod_dir,
                        &mod_parsed)
    }

    fn process_mod(&mut self,
                   pkg: &PackageRef,
                   mod_dir: &Path,
                   items: &Vec<syn::Item>) -> Result<(), String> {
        self.out.load_syn_crate_mod(&self.binding_crate_name,
                                    &pkg.name,
                                    &Cfg::join(&self.cfg_stack),
                                    items);

        for item in items {
            match item.node {
                syn::ItemKind::Mod(ref inline_items) => {
                    let next_mod_name = item.ident.to_string();

                    let cfg = Cfg::load(&item.attrs);
                    if let &Some(ref cfg) = &cfg {
                        self.cfg_stack.push(cfg.clone());
                    }

                    if let &Some(ref inline_items) = inline_items {
                        self.process_mod(pkg,
                                         &mod_dir.join(&next_mod_name),
                                         inline_items)?;
                    } else {
                        let next_mod_path1 = mod_dir.join(next_mod_name.clone() + ".rs");
                        let next_mod_path2 = mod_dir.join(next_mod_name.clone()).join("mod.rs");

                        if next_mod_path1.exists() {
                            self.parse_mod(pkg,
                                           next_mod_path1.as_path())?;
                        } else if next_mod_path2.exists() {
                            self.parse_mod(pkg,
                                           next_mod_path2.as_path())?;
                        } else {
                            // This should be an error, but is common enough to just elicit a warning
                            warn!("Parsing crate `{}`: can't find mod {}`.", pkg.name, next_mod_name);
                        }
                    }

                    if cfg.is_some() {
                        self.cfg_stack.pop();
                    }
                }
                syn::ItemKind::ExternCrate(ref cr) => {
                    let dep_pkg_name = if let Some(ref name) = *cr {
                        name.to_string()
                    } else {
                        item.ident.to_string()
                    };

                    let cfg = Cfg::load(&item.attrs);
                    if let &Some(ref cfg) = &cfg {
                        self.cfg_stack.push(cfg.clone());
                    }

                    if self.should_parse_dependency(&dep_pkg_name) {
                        if self.lib.is_some() {
                            let dep_pkg_ref = self.lib.as_ref().unwrap().find_dep_ref(pkg, &dep_pkg_name);

                            if let Some(dep_pkg_ref) = dep_pkg_ref {
                                self.parse_crate(&dep_pkg_ref)?;
                            } else {
                                error!("Parsing crate `{}`: can't find dependency version for `{}`.", pkg.name, dep_pkg_name);
                            }
                        } else {
                            error!("Parsing crate `{}`: cannot parse external crate `{}` because cbindgen is in single source mode. Consider specifying a crate directory instead of a source file.", pkg.name, dep_pkg_name);
                        }
                    }

                    if cfg.is_some() {
                        self.cfg_stack.pop();
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Parse {
    pub constants: ItemMap<Constant>,
    pub globals: ItemMap<Static>,
    pub enums: ItemMap<Enum>,
    pub structs: ItemMap<Struct>,
    pub unions: ItemMap<Union>,
    pub opaque_items: ItemMap<OpaqueItem>,
    pub typedefs: ItemMap<Typedef>,
    pub specializations: ItemMap<Specialization>,
    pub functions: Vec<Function>,
}

impl Parse {
    pub fn new() -> Parse {
        Parse {
            constants: ItemMap::new(),
            globals: ItemMap::new(),
            enums: ItemMap::new(),
            structs: ItemMap::new(),
            unions: ItemMap::new(),
            opaque_items: ItemMap::new(),
            typedefs: ItemMap::new(),
            specializations: ItemMap::new(),
            functions: Vec::new(),
        }
    }

    pub fn add_std_types(&mut self) {
        let mut add_opaque = |name: &str, generic_params: Vec<&str>| {
            self.opaque_items.try_insert(OpaqueItem {
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

    pub fn extend_with(&mut self, other: &Parse) {
        self.constants.extend_with(&other.constants);
        self.globals.extend_with(&other.globals);
        self.enums.extend_with(&other.enums);
        self.structs.extend_with(&other.structs);
        self.unions.extend_with(&other.unions);
        self.opaque_items.extend_with(&other.opaque_items);
        self.typedefs.extend_with(&other.typedefs);
        self.specializations.extend_with(&other.specializations);
        self.functions.extend_from_slice(&other.functions);
    }

    pub fn load_syn_crate_mod(&mut self,
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
                syn::ItemKind::Const(ref ty, ref expr) => {
                    self.load_syn_const(binding_crate_name,
                                        crate_name,
                                        mod_cfg,
                                        item,
                                        ty,
                                        expr);
                }
                syn::ItemKind::Static(ref ty, ref mutability, ref _expr) => {
                    self.load_syn_static(binding_crate_name,
                                         crate_name,
                                         mod_cfg,
                                         item,
                                         ty,
                                         mutability);
                }
                syn::ItemKind::Struct(ref variant, ref generics) => {
                    self.load_syn_struct(crate_name, mod_cfg, item, variant, generics);
                }
                syn::ItemKind::Union(ref variant, ref generics) => {
                    self.load_syn_union(crate_name, mod_cfg, item, variant, generics);
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
            info!("Skip {}::{} - (extern block must be extern C).", crate_name, &item.ident);
            return;
        }

        for foreign_item in &block.items {
            match foreign_item.node {
                syn::ForeignItemKind::Fn(ref decl,
                                         ref _generic) => {
                    if crate_name != binding_crate_name {
                        info!("Skip {}::{} - (fn's outside of the binding crate are not used).",
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
                            info!("Take {}::{}.", crate_name, &foreign_item.ident);

                            self.functions.push(func);
                        }
                        Err(msg) => {
                            error!("Cannot use fn {}::{} ({}).",
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
            info!("Skip {}::{} - (fn's outside of the binding crate are not used).",
                  crate_name,
                  &item.ident);
            return;
        }

        if item.is_no_mangle() && (abi.is_omitted() || abi.is_c()) {
            match Function::load(item.ident.to_string(),
                                 decl,
                                 false,
                                 &item.attrs,
                                 mod_cfg) {
                Ok(func) => {
                    info!("Take {}::{}.", crate_name, &item.ident);

                    self.functions.push(func);
                }
                Err(msg) => {
                    error!("Cannot use fn {}::{} ({}).",
                           crate_name,
                           &item.ident,
                           msg);
                },
            }
        } else {
            if (abi.is_omitted() || abi.is_c()) && !item.is_no_mangle() {
                warn!("Skip {}::{} - (`extern` but not `no_mangle`).",
                      crate_name,
                      &item.ident);
            }
            if abi.is_some() && !(abi.is_omitted() || abi.is_c()) {
                warn!("Skip {}::{} - (non `extern \"C\"`).",
                      crate_name,
                      &item.ident);
            }
        }
    }

    /// Loads a `const` declaration
    fn load_syn_const(&mut self,
                      binding_crate_name: &str,
                      crate_name: &str,
                      mod_cfg: &Option<Cfg>,
                      item: &syn::Item,
                      ty: &syn::Ty,
                      expr: &syn::Expr) {
        if crate_name != binding_crate_name {
            info!("Skip {}::{} - (const's outside of the binding crate are not used).",
                  crate_name,
                  &item.ident);
            return;
        }

        let const_name = item.ident.to_string();

        match Constant::load(const_name.clone(),
                             ty,
                             expr,
                             &item.attrs,
                             mod_cfg) {
            Ok(constant) => {
                info!("Take {}::{}.", crate_name, &item.ident);

                self.constants.try_insert(constant);
            }
            Err(msg) => {
                warn!("Skip {}::{} - ({})",
                      crate_name,
                      &item.ident,
                      msg);
            }
        }
    }

    /// Loads a `static` declaration
    fn load_syn_static(&mut self,
                       binding_crate_name: &str,
                       crate_name: &str,
                       mod_cfg: &Option<Cfg>,
                       item: &syn::Item,
                       ty: &syn::Ty,
                       mutability: &syn::Mutability) {
        if crate_name != binding_crate_name {
            info!("Skip {}::{} - (static's outside of the binding crate are not used).",
                  crate_name,
                  &item.ident);
            return;
        }

        let static_name = item.ident.to_string();

        match Static::load(static_name.clone(),
                           ty,
                           mutability,
                           &item.attrs,
                            mod_cfg) {
            Ok(constant) => {
                info!("Take {}::{}.", crate_name, &item.ident);

                self.globals.try_insert(constant);
            }
            Err(msg) => {
                warn!("Skip {}::{} - ({})",
                      crate_name,
                      &item.ident,
                      msg);
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
                info!("Take {}::{}.", crate_name, &item.ident);

                self.structs.try_insert(st);
            }
            Err(msg) => {
                info!("Take {}::{} - opaque ({}).",
                      crate_name,
                      &item.ident,
                      msg);
                self.opaque_items.try_insert(OpaqueItem::new(struct_name,
                                                             generics,
                                                             &item.attrs,
                                                             mod_cfg));
            }
        }
    }

    /// Loads a `union` declaration
    fn load_syn_union(&mut self,
                      crate_name: &str,
                      mod_cfg: &Option<Cfg>,
                      item: &syn::Item,
                      variant: &syn::VariantData,
                      generics: &syn::Generics) {
        let union_name = item.ident.to_string();

        match Union::load(union_name.clone(),
                          variant,
                          generics,
                          &item.attrs,
                          mod_cfg) {
            Ok(st) => {
                info!("Take {}::{}.", crate_name, &item.ident);

                self.unions.try_insert(st);
            }
            Err(msg) => {
                info!("Take {}::{} - opaque ({}).",
                      crate_name,
                      &item.ident,
                      msg);
                self.opaque_items.try_insert(OpaqueItem::new(union_name,
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
            info!("Skip {}::{} - (has generics or lifetimes or where bounds).",
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
                info!("Take {}::{}.", crate_name, &item.ident);
                self.enums.try_insert(en);
            }
            Err(msg) => {
                info!("Take {}::{} - opaque ({}).", crate_name, &item.ident, msg);
                self.opaque_items.try_insert(OpaqueItem::new(enum_name,
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
                    info!("Take {}::{}.", crate_name, &item.ident);
                    self.typedefs.try_insert(typedef);
                    return;
                }
                Err(msg) => msg,
            }
        } else {
            format!("Cannot have generics in typedef.")
        };

        let fail2 = match Specialization::load(alias_name.clone(),
                                               generics,
                                               ty,
                                               &item.attrs,
                                               mod_cfg) {
            Ok(spec) => {
                info!("Take {}::{}.", crate_name, &item.ident);
                self.specializations.try_insert(spec);
                return;
            }
            Err(msg) => msg,
        };

        info!("Skip {}::{} - ({} and {}).",
              crate_name,
              &item.ident,
              fail1,
              fail2);
    }
}
