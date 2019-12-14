/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::path::{Path as FilePath, PathBuf as FilePathBuf};

use syn;

use bindgen::bitflags;
use bindgen::cargo::{Cargo, PackageRef};
use bindgen::config::{Config, ParseConfig};
use bindgen::error::Error;
use bindgen::ir::{
    AnnotationSet, Cfg, Constant, Documentation, Enum, Function, GenericParams, ItemMap,
    OpaqueItem, Path, Static, Struct, Type, Typedef, Union,
};
use bindgen::utilities::{SynAbiHelpers, SynItemHelpers};

const STD_CRATES: &[&str] = &[
    "std",
    "std_unicode",
    "alloc",
    "collections",
    "core",
    "proc_macro",
];

type ParseResult = Result<Parse, Error>;

/// Parses a single rust source file, not following `mod` or `extern crate`.
pub fn parse_src(src_file: &FilePath, config: &Config) -> ParseResult {
    let mod_name = src_file.file_stem().unwrap().to_str().unwrap();
    let mut config = config.clone();
    config.parse = ParseConfig {
        parse_deps: true,
        ..ParseConfig::default()
    };

    let mut context = Parser {
        binding_crate_name: mod_name.to_owned(),
        config: &config,
        lib: None,
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
pub(crate) fn parse_lib(lib: Cargo, config: &Config) -> ParseResult {
    let mut context = Parser {
        binding_crate_name: lib.binding_crate_name().to_owned(),
        config,
        lib: Some(lib),
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
struct Parser<'a> {
    binding_crate_name: String,
    lib: Option<Cargo>,
    config: &'a Config,

    parsed_crates: HashSet<String>,
    cache_src: HashMap<FilePathBuf, Vec<syn::Item>>,
    cache_expanded_crate: HashMap<String, Vec<syn::Item>>,

    cfg_stack: Vec<Cfg>,

    out: Parse,
}

impl<'a> Parser<'a> {
    fn should_parse_dependency(&self, pkg_name: &String) -> bool {
        if self.parsed_crates.contains(pkg_name) {
            return false;
        }

        if !self.config.parse.parse_deps {
            return false;
        }

        // Skip any whitelist or blacklist for expand
        if self.config.parse.expand.crates.contains(&pkg_name) {
            return true;
        }

        // If we have a whitelist, check it
        if let Some(ref include) = self.config.parse.include {
            if !include.contains(&pkg_name) {
                return false;
            }
        }

        // Check the blacklist
        !STD_CRATES.contains(&pkg_name.as_ref()) && !self.config.parse.exclude.contains(&pkg_name)
    }

    fn parse_crate(&mut self, pkg: &PackageRef) -> Result<(), Error> {
        assert!(self.lib.is_some());
        self.parsed_crates.insert(pkg.name.clone());

        // Check if we should use cargo expand for this crate
        if self.config.parse.expand.crates.contains(&pkg.name) {
            self.parse_expand_crate(pkg)?;
        } else {
            // Parse the crate before the dependencies otherwise the same-named idents we
            // want to generate bindings for would be replaced by the ones provided
            // by the first dependency containing it.
            let crate_src = self.lib.as_ref().unwrap().find_crate_src(pkg);

            match crate_src {
                Some(crate_src) => self.parse_mod(pkg, crate_src.as_path())?,
                None => {
                    // This should be an error, but is common enough to just elicit a warning
                    warn!(
                        "Parsing crate `{}`: can't find lib.rs with `cargo metadata`.",
                        pkg.name
                    );
                }
            }
        }

        for (dep_pkg, cfg) in self.lib.as_ref().unwrap().dependencies(&pkg) {
            if !self.should_parse_dependency(&dep_pkg.name) {
                continue;
            }

            if let Some(ref cfg) = cfg {
                self.cfg_stack.push(cfg.clone());
            }

            self.parse_crate(&dep_pkg)?;

            if cfg.is_some() {
                self.cfg_stack.pop();
            }
        }

        Ok(())
    }

    fn parse_expand_crate(&mut self, pkg: &PackageRef) -> Result<(), Error> {
        assert!(self.lib.is_some());

        // If you want to expand the crate you run cbindgen on you might end up in an endless
        // recursion if the cbindgen generation is triggered from build.rs. Hence don't run the
        // expansion if the build was already triggered by cbindgen.
        if std::env::var("_CBINDGEN_IS_RUNNING").is_ok() {
            return Ok(());
        }

        let mod_parsed = {
            if !self.cache_expanded_crate.contains_key(&pkg.name) {
                let s = self
                    .lib
                    .as_ref()
                    .unwrap()
                    .expand_crate(
                        pkg,
                        self.config.parse.expand.all_features,
                        self.config.parse.expand.default_features,
                        &self.config.parse.expand.features,
                    )
                    .map_err(|x| Error::CargoExpand(pkg.name.clone(), x))?;
                let i = syn::parse_file(&s).map_err(|x| Error::ParseSyntaxError {
                    crate_name: pkg.name.clone(),
                    src_path: "".to_owned(),
                    error: x,
                })?;
                self.cache_expanded_crate.insert(pkg.name.clone(), i.items);
            }

            self.cache_expanded_crate.get(&pkg.name).unwrap().clone()
        };

        self.process_expanded_mod(pkg, &mod_parsed)
    }

    fn process_expanded_mod(&mut self, pkg: &PackageRef, items: &[syn::Item]) -> Result<(), Error> {
        self.out.load_syn_crate_mod(
            &self.config,
            &self.binding_crate_name,
            &pkg.name,
            Cfg::join(&self.cfg_stack).as_ref(),
            items,
        );

        for item in items {
            if item.has_test_attr() {
                continue;
            }
            if let syn::Item::Mod(ref item) = *item {
                let cfg = Cfg::load(&item.attrs);
                if let Some(ref cfg) = cfg {
                    self.cfg_stack.push(cfg.clone());
                }

                if let Some((_, ref inline_items)) = item.content {
                    self.process_expanded_mod(pkg, inline_items)?;
                } else {
                    unreachable!();
                }

                if cfg.is_some() {
                    self.cfg_stack.pop();
                }
            }
        }

        Ok(())
    }

    fn parse_mod(&mut self, pkg: &PackageRef, mod_path: &FilePath) -> Result<(), Error> {
        let mod_parsed = {
            let owned_mod_path = mod_path.to_path_buf();

            if !self.cache_src.contains_key(&owned_mod_path) {
                let mut s = String::new();
                let mut f = File::open(mod_path).map_err(|_| Error::ParseCannotOpenFile {
                    crate_name: pkg.name.clone(),
                    src_path: mod_path.to_str().unwrap().to_owned(),
                })?;
                f.read_to_string(&mut s)
                    .map_err(|_| Error::ParseCannotOpenFile {
                        crate_name: pkg.name.clone(),
                        src_path: mod_path.to_str().unwrap().to_owned(),
                    })?;

                let i = syn::parse_file(&s).map_err(|x| Error::ParseSyntaxError {
                    crate_name: pkg.name.clone(),
                    src_path: owned_mod_path.to_string_lossy().into(),
                    error: x,
                })?;

                self.cache_src.insert(owned_mod_path.clone(), i.items);
            }

            self.cache_src.get(&owned_mod_path).unwrap().clone()
        };

        let mod_dir = mod_path.parent().unwrap();

        self.process_mod(pkg, mod_dir, &mod_parsed)
    }

    fn process_mod(
        &mut self,
        pkg: &PackageRef,
        mod_dir: &FilePath,
        items: &[syn::Item],
    ) -> Result<(), Error> {
        self.out.load_syn_crate_mod(
            &self.config,
            &self.binding_crate_name,
            &pkg.name,
            Cfg::join(&self.cfg_stack).as_ref(),
            items,
        );

        for item in items {
            if item.has_test_attr() {
                continue;
            }
            if let syn::Item::Mod(ref item) = *item {
                let next_mod_name = item.ident.to_string();

                let cfg = Cfg::load(&item.attrs);
                if let Some(ref cfg) = cfg {
                    self.cfg_stack.push(cfg.clone());
                }

                if let Some((_, ref inline_items)) = item.content {
                    self.process_mod(pkg, &mod_dir.join(&next_mod_name), inline_items)?;
                } else {
                    let next_mod_path1 = mod_dir.join(next_mod_name.clone() + ".rs");
                    let next_mod_path2 = mod_dir.join(next_mod_name.clone()).join("mod.rs");

                    if next_mod_path1.exists() {
                        self.parse_mod(pkg, next_mod_path1.as_path())?;
                    } else if next_mod_path2.exists() {
                        self.parse_mod(pkg, next_mod_path2.as_path())?;
                    } else {
                        // Last chance to find a module path
                        let mut path_attr_found = false;
                        for attr in &item.attrs {
                            match attr.parse_meta() {
                                Ok(syn::Meta::NameValue(syn::MetaNameValue {
                                    path, lit, ..
                                })) => match lit {
                                    syn::Lit::Str(ref path_lit) if path.is_ident("path") => {
                                        path_attr_found = true;
                                        self.parse_mod(pkg, &mod_dir.join(path_lit.value()))?;
                                        break;
                                    }
                                    _ => (),
                                },
                                _ => (),
                            }
                        }

                        // This should be an error, but it's common enough to
                        // just elicit a warning
                        if !path_attr_found {
                            warn!(
                                "Parsing crate `{}`: can't find mod {}`.",
                                pkg.name, next_mod_name
                            );
                        }
                    }
                }

                if cfg.is_some() {
                    self.cfg_stack.pop();
                }
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
            functions: Vec::new(),
        }
    }

    pub fn add_std_types(&mut self) {
        let mut add_opaque = |path: &str, generic_params: Vec<&str>| {
            let path = Path::new(path);
            let generic_params: Vec<_> = generic_params.into_iter().map(Path::new).collect();
            self.opaque_items.try_insert(OpaqueItem::new(
                path,
                GenericParams(generic_params),
                None,
                AnnotationSet::new(),
                Documentation::none(),
            ))
        };

        add_opaque("String", vec![]);
        add_opaque("Box", vec!["T"]);
        add_opaque("Rc", vec!["T"]);
        add_opaque("Arc", vec!["T"]);
        add_opaque("Result", vec!["T", "E"]);
        add_opaque("Option", vec!["T"]);
        add_opaque("NonNull", vec!["T"]);
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
        self.functions.extend_from_slice(&other.functions);
    }

    pub fn load_syn_crate_mod(
        &mut self,
        config: &Config,
        binding_crate_name: &str,
        crate_name: &str,
        mod_cfg: Option<&Cfg>,
        items: &[syn::Item],
    ) {
        let mut impls_with_assoc_consts = Vec::new();

        for item in items {
            if item.has_test_attr() {
                continue;
            }
            match item {
                syn::Item::ForeignMod(ref item) => {
                    self.load_syn_foreign_mod(
                        config,
                        binding_crate_name,
                        crate_name,
                        mod_cfg,
                        item,
                    );
                }
                syn::Item::Fn(ref item) => {
                    self.load_syn_fn(config, binding_crate_name, crate_name, mod_cfg, item);
                }
                syn::Item::Const(ref item) => {
                    self.load_syn_const(config, binding_crate_name, crate_name, mod_cfg, item);
                }
                syn::Item::Static(ref item) => {
                    self.load_syn_static(config, binding_crate_name, crate_name, mod_cfg, item);
                }
                syn::Item::Struct(ref item) => {
                    self.load_syn_struct(config, crate_name, mod_cfg, item);
                }
                syn::Item::Union(ref item) => {
                    self.load_syn_union(config, crate_name, mod_cfg, item);
                }
                syn::Item::Enum(ref item) => {
                    self.load_syn_enum(crate_name, mod_cfg, item);
                }
                syn::Item::Type(ref item) => {
                    self.load_syn_ty(crate_name, mod_cfg, item);
                }
                syn::Item::Impl(ref item_impl) => {
                    let has_assoc_const = item_impl.items.iter().any(|item| match item {
                        syn::ImplItem::Const(_) => true,
                        _ => false,
                    });
                    if has_assoc_const {
                        impls_with_assoc_consts.push(item_impl);
                    }
                }
                syn::Item::Macro(ref item) => {
                    self.load_builtin_macro(config, crate_name, mod_cfg, item)
                }
                _ => {}
            }
        }

        for item_impl in impls_with_assoc_consts {
            self.load_syn_assoc_consts_from_impl(crate_name, mod_cfg, item_impl)
        }
    }

    fn load_syn_assoc_consts_from_impl(
        &mut self,
        crate_name: &str,
        mod_cfg: Option<&Cfg>,
        item_impl: &syn::ItemImpl,
    ) {
        let associated_constants = item_impl.items.iter().filter_map(|item| match item {
            syn::ImplItem::Const(ref associated_constant) => Some(associated_constant),
            _ => None,
        });
        self.load_syn_assoc_consts(
            crate_name,
            mod_cfg,
            &item_impl.self_ty,
            associated_constants,
        );
    }

    /// Enters a `extern "C" { }` declaration and loads function declarations.
    fn load_syn_foreign_mod(
        &mut self,
        config: &Config,
        binding_crate_name: &str,
        crate_name: &str,
        mod_cfg: Option<&Cfg>,
        item: &syn::ItemForeignMod,
    ) {
        if !item.abi.is_c() {
            info!("Skip {} - (extern block must be extern C).", crate_name);
            return;
        }

        for foreign_item in &item.items {
            if let syn::ForeignItem::Fn(ref function) = *foreign_item {
                if !config
                    .parse
                    .should_generate_top_level_item(crate_name, binding_crate_name)
                {
                    info!(
                        "Skip {}::{} - (fn's outside of the binding crate are not used).",
                        crate_name, &function.sig.ident
                    );
                    return;
                }
                let path = Path::new(function.sig.ident.to_string());
                match Function::load(path, &function.sig, true, &function.attrs, mod_cfg) {
                    Ok(func) => {
                        info!("Take {}::{}.", crate_name, &function.sig.ident);

                        self.functions.push(func);
                    }
                    Err(msg) => {
                        error!(
                            "Cannot use fn {}::{} ({}).",
                            crate_name, &function.sig.ident, msg
                        );
                    }
                }
            }
        }
    }

    /// Loads a `fn` declaration
    fn load_syn_fn(
        &mut self,
        config: &Config,
        binding_crate_name: &str,
        crate_name: &str,
        mod_cfg: Option<&Cfg>,
        item: &syn::ItemFn,
    ) {
        if !config
            .parse
            .should_generate_top_level_item(crate_name, binding_crate_name)
        {
            info!(
                "Skip {}::{} - (fn's outside of the binding crate are not used).",
                crate_name, &item.sig.ident
            );
            return;
        }

        if let syn::Visibility::Public(_) = item.vis {
            if item.is_no_mangle() && (item.sig.abi.is_omitted() || item.sig.abi.is_c()) {
                let path = Path::new(item.sig.ident.to_string());
                match Function::load(path, &item.sig, false, &item.attrs, mod_cfg) {
                    Ok(func) => {
                        info!("Take {}::{}.", crate_name, &item.sig.ident);

                        self.functions.push(func);
                    }
                    Err(msg) => {
                        error!(
                            "Cannot use fn {}::{} ({}).",
                            crate_name, &item.sig.ident, msg
                        );
                    }
                }
                return;
            }
        }

        // TODO
        if let syn::Visibility::Public(_) = item.vis {
        } else {
            warn!("Skip {}::{} - (not `pub`).", crate_name, &item.sig.ident);
        }
        if (item.sig.abi.is_omitted() || item.sig.abi.is_c()) && !item.is_no_mangle() {
            warn!(
                "Skip {}::{} - (`extern` but not `no_mangle`).",
                crate_name, &item.sig.ident
            );
        }
        if item.sig.abi.is_some() && !(item.sig.abi.is_omitted() || item.sig.abi.is_c()) {
            warn!(
                "Skip {}::{} - (non `extern \"C\"`).",
                crate_name, &item.sig.ident
            );
        }
    }

    /// Loads associated `const` declarations
    fn load_syn_assoc_consts<'a, I>(
        &mut self,
        crate_name: &str,
        mod_cfg: Option<&Cfg>,
        impl_ty: &syn::Type,
        items: I,
    ) where
        I: IntoIterator<Item = &'a syn::ImplItemConst>,
    {
        let ty = match Type::load(impl_ty) {
            Ok(ty) => ty,
            Err(e) => {
                warn!("Skipping associated constants for {:?}: {:?}", impl_ty, e);
                return;
            }
        };
        if ty.is_none() {
            return;
        }

        let impl_path = ty.unwrap().get_root_path().unwrap();

        for item in items.into_iter() {
            if let syn::Visibility::Public(_) = item.vis {
            } else {
                warn!("Skip {}::{} - (not `pub`).", crate_name, &item.ident);
                return;
            }

            let path = Path::new(item.ident.to_string());
            match Constant::load(
                path,
                mod_cfg,
                &item.ty,
                &item.expr,
                &item.attrs,
                Some(impl_path.clone()),
            ) {
                Ok(constant) => {
                    info!("Take {}::{}::{}.", crate_name, impl_path, &item.ident);
                    let mut any = false;
                    self.structs.for_items_mut(&impl_path, |item| {
                        any = true;
                        item.add_associated_constant(constant.clone());
                    });
                    // Handle associated constants to other item types that are
                    // not structs like enums or such as regular constants.
                    if !any && !self.constants.try_insert(constant) {
                        error!(
                            "Conflicting name for constant {}::{}::{}.",
                            crate_name, impl_path, &item.ident,
                        );
                    }
                }
                Err(msg) => {
                    warn!("Skip {}::{} - ({})", crate_name, &item.ident, msg);
                }
            }
        }
    }

    /// Loads a `const` declaration
    fn load_syn_const(
        &mut self,
        config: &Config,
        binding_crate_name: &str,
        crate_name: &str,
        mod_cfg: Option<&Cfg>,
        item: &syn::ItemConst,
    ) {
        if !config
            .parse
            .should_generate_top_level_item(crate_name, binding_crate_name)
        {
            info!(
                "Skip {}::{} - (const's outside of the binding crate are not used).",
                crate_name, &item.ident
            );
            return;
        }

        if let syn::Visibility::Public(_) = item.vis {
        } else {
            warn!("Skip {}::{} - (not `pub`).", crate_name, &item.ident);
            return;
        }

        let path = Path::new(item.ident.to_string());
        match Constant::load(path, mod_cfg, &item.ty, &item.expr, &item.attrs, None) {
            Ok(constant) => {
                info!("Take {}::{}.", crate_name, &item.ident);

                let full_name = constant.path.clone();
                if !self.constants.try_insert(constant) {
                    error!("Conflicting name for constant {}", full_name);
                }
            }
            Err(msg) => {
                warn!("Skip {}::{} - ({})", crate_name, &item.ident, msg);
            }
        }
    }

    /// Loads a `static` declaration
    fn load_syn_static(
        &mut self,
        config: &Config,
        binding_crate_name: &str,
        crate_name: &str,
        mod_cfg: Option<&Cfg>,
        item: &syn::ItemStatic,
    ) {
        if !config
            .parse
            .should_generate_top_level_item(crate_name, binding_crate_name)
        {
            info!(
                "Skip {}::{} - (static's outside of the binding crate are not used).",
                crate_name, &item.ident
            );
            return;
        }

        if let syn::Visibility::Public(_) = item.vis {
            if item.is_no_mangle() {
                match Static::load(item, mod_cfg) {
                    Ok(constant) => {
                        info!("Take {}::{}.", crate_name, &item.ident);

                        self.globals.try_insert(constant);
                    }
                    Err(msg) => {
                        warn!("Skip {}::{} - ({})", crate_name, &item.ident, msg);
                    }
                }
            }
        }

        // TODO
        if let syn::Visibility::Public(_) = item.vis {
        } else {
            warn!("Skip {}::{} - (not `pub`).", crate_name, &item.ident);
        }
        if !item.is_no_mangle() {
            warn!("Skip {}::{} - (not `no_mangle`).", crate_name, &item.ident);
        }
    }

    /// Loads a `struct` declaration
    fn load_syn_struct(
        &mut self,
        config: &Config,
        crate_name: &str,
        mod_cfg: Option<&Cfg>,
        item: &syn::ItemStruct,
    ) {
        match Struct::load(&config.layout, item, mod_cfg) {
            Ok(st) => {
                info!("Take {}::{}.", crate_name, &item.ident);
                self.structs.try_insert(st);
            }
            Err(msg) => {
                info!("Take {}::{} - opaque ({}).", crate_name, &item.ident, msg);
                let path = Path::new(item.ident.to_string());
                self.opaque_items.try_insert(
                    OpaqueItem::load(path, &item.generics, &item.attrs, mod_cfg).unwrap(),
                );
            }
        }
    }

    /// Loads a `union` declaration
    fn load_syn_union(
        &mut self,
        config: &Config,
        crate_name: &str,
        mod_cfg: Option<&Cfg>,
        item: &syn::ItemUnion,
    ) {
        match Union::load(&config.layout, item, mod_cfg) {
            Ok(st) => {
                info!("Take {}::{}.", crate_name, &item.ident);

                self.unions.try_insert(st);
            }
            Err(msg) => {
                info!("Take {}::{} - opaque ({}).", crate_name, &item.ident, msg);
                let path = Path::new(item.ident.to_string());
                self.opaque_items.try_insert(
                    OpaqueItem::load(path, &item.generics, &item.attrs, mod_cfg).unwrap(),
                );
            }
        }
    }

    /// Loads a `enum` declaration
    fn load_syn_enum(&mut self, crate_name: &str, mod_cfg: Option<&Cfg>, item: &syn::ItemEnum) {
        if item.generics.lifetimes().count() > 0 {
            info!(
                "Skip {}::{} - (has generics or lifetimes or where bounds).",
                crate_name, &item.ident
            );
            return;
        }

        match Enum::load(item, mod_cfg) {
            Ok(en) => {
                info!("Take {}::{}.", crate_name, &item.ident);
                self.enums.try_insert(en);
            }
            Err(msg) => {
                info!("Take {}::{} - opaque ({}).", crate_name, &item.ident, msg);
                let path = Path::new(item.ident.to_string());
                self.opaque_items.try_insert(
                    OpaqueItem::load(path, &item.generics, &item.attrs, mod_cfg).unwrap(),
                );
            }
        }
    }

    /// Loads a `type` declaration
    fn load_syn_ty(&mut self, crate_name: &str, mod_cfg: Option<&Cfg>, item: &syn::ItemType) {
        match Typedef::load(item, mod_cfg) {
            Ok(st) => {
                info!("Take {}::{}.", crate_name, &item.ident);

                self.typedefs.try_insert(st);
            }
            Err(msg) => {
                info!("Take {}::{} - opaque ({}).", crate_name, &item.ident, msg);
                let path = Path::new(item.ident.to_string());
                self.opaque_items.try_insert(
                    OpaqueItem::load(path, &item.generics, &item.attrs, mod_cfg).unwrap(),
                );
            }
        }
    }

    fn load_builtin_macro(
        &mut self,
        config: &Config,
        crate_name: &str,
        mod_cfg: Option<&Cfg>,
        item: &syn::ItemMacro,
    ) {
        let name = match item.mac.path.segments.last() {
            Some(ref n) => n.ident.to_string(),
            None => return,
        };

        if name != "bitflags" || !config.macro_expansion.bitflags {
            return;
        }

        let bitflags = match bitflags::parse(item.mac.tokens.clone()) {
            Ok(b) => b,
            Err(e) => {
                warn!("Failed to parse bitflags invocation: {:?}", e);
                return;
            }
        };

        let (struct_, impl_) = bitflags.expand();
        self.load_syn_struct(config, crate_name, mod_cfg, &struct_);
        // We know that the expansion will only reference `struct_`, so it's
        // fine to just do it here instead of deferring it like we do with the
        // other calls to this function.
        self.load_syn_assoc_consts_from_impl(crate_name, mod_cfg, &impl_);
    }
}
