/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use syn;

use bindgen::cargo::{Cargo, PackageRef};
use bindgen::error::Error;
use bindgen::ir::{AnnotationSet, Cfg, Constant, Documentation, Enum, Function, GenericParams};
use bindgen::ir::{ItemMap, OpaqueItem, Static, Struct, Typedef, Union};
use bindgen::utilities::{SynAbiHelpers, SynItemHelpers};

const STD_CRATES: &'static [&'static str] = &[
    "std",
    "std_unicode",
    "alloc",
    "collections",
    "core",
    "proc_macro",
];

type ParseResult = Result<Parse, Error>;

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
        expand_all_features: true,
        expand_default_features: true,
        expand_features: None,
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
pub(crate) fn parse_lib(
    lib: Cargo,
    parse_deps: bool,
    include: &Option<Vec<String>>,
    exclude: &Vec<String>,
    expand: &Vec<String>,
    expand_all_features: bool,
    expand_default_features: bool,
    expand_features: &Option<Vec<String>>,
) -> ParseResult {
    let mut context = Parser {
        binding_crate_name: lib.binding_crate_name().to_owned(),
        lib: Some(lib),
        parse_deps: parse_deps,
        include: include.clone(),
        exclude: exclude.clone(),
        expand: expand.clone(),
        expand_all_features,
        expand_default_features,
        expand_features: expand_features.clone(),
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
    expand_all_features: bool,
    expand_default_features: bool,
    expand_features: Option<Vec<String>>,

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
        return !STD_CRATES.contains(&pkg_name.as_ref()) && !self.exclude.contains(&pkg_name);
    }

    fn parse_crate(&mut self, pkg: &PackageRef) -> Result<(), Error> {
        assert!(self.lib.is_some());
        self.parsed_crates.insert(pkg.name.clone());

        // Check if we should use cargo expand for this crate
        if self.expand.contains(&pkg.name) {
            return self.parse_expand_crate(pkg);
        }

        // Otherwise do our normal parse
        let crate_src = self.lib.as_ref().unwrap().find_crate_src(pkg);

        match crate_src {
            Some(crate_src) => self.parse_mod(pkg, crate_src.as_path()),
            None => {
                // This should be an error, but is common enough to just elicit a warning
                warn!(
                    "Parsing crate `{}`: can't find lib.rs with `cargo metadata`.",
                    pkg.name
                );
                Ok(())
            }
        }
    }

    fn parse_expand_crate(&mut self, pkg: &PackageRef) -> Result<(), Error> {
        assert!(self.lib.is_some());

        let mod_parsed = {
            if !self.cache_expanded_crate.contains_key(&pkg.name) {
                let s = self
                    .lib
                    .as_ref()
                    .unwrap()
                    .expand_crate(
                        pkg,
                        self.expand_all_features,
                        self.expand_default_features,
                        &self.expand_features,
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
            &self.binding_crate_name,
            &pkg.name,
            &Cfg::join(&self.cfg_stack),
            items,
        );

        for item in items {
            if item.has_test_attr() {
                continue;
            }
            match item {
                &syn::Item::Mod(ref item) => {
                    let cfg = Cfg::load(&item.attrs);
                    if let &Some(ref cfg) = &cfg {
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
                &syn::Item::ExternCrate(ref item) => {
                    let dep_pkg_name = item.ident.to_string();

                    let cfg = Cfg::load(&item.attrs);
                    if let &Some(ref cfg) = &cfg {
                        self.cfg_stack.push(cfg.clone());
                    }

                    if self.should_parse_dependency(&dep_pkg_name) {
                        if self.lib.is_some() {
                            let dep_pkg_ref =
                                self.lib.as_ref().unwrap().find_dep_ref(pkg, &dep_pkg_name);

                            if let Some(dep_pkg_ref) = dep_pkg_ref {
                                self.parse_crate(&dep_pkg_ref)?;
                            } else {
                                error!(
                                    "Parsing crate `{}`: can't find dependency version for `{}`.",
                                    pkg.name, dep_pkg_name
                                );
                            }
                        } else {
                            error!(
                                "Parsing crate `{}`: cannot parse external crate `{}` because \
                                 cbindgen is in single source mode. Consider specifying a crate \
                                 directory instead of a source file.",
                                pkg.name, dep_pkg_name
                            );
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

    fn parse_mod(&mut self, pkg: &PackageRef, mod_path: &Path) -> Result<(), Error> {
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
        mod_dir: &Path,
        items: &[syn::Item],
    ) -> Result<(), Error> {
        self.out.load_syn_crate_mod(
            &self.binding_crate_name,
            &pkg.name,
            &Cfg::join(&self.cfg_stack),
            items,
        );

        for item in items {
            if item.has_test_attr() {
                continue;
            }
            match item {
                &syn::Item::Mod(ref item) => {
                    let next_mod_name = item.ident.to_string();

                    let cfg = Cfg::load(&item.attrs);
                    if let &Some(ref cfg) = &cfg {
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
                                if attr.is_sugared_doc {
                                    continue;
                                }

                                match attr.interpret_meta() {
                                    Some(syn::Meta::NameValue(syn::MetaNameValue {
                                        ident,
                                        lit,
                                        ..
                                    })) => match lit {
                                        syn::Lit::Str(ref path) if ident == "path" => {
                                            path_attr_found = true;
                                            self.parse_mod(pkg, &mod_dir.join(path.value()))?;
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
                &syn::Item::ExternCrate(ref item) => {
                    let dep_pkg_name = item.ident.to_string();

                    let cfg = Cfg::load(&item.attrs);
                    if let &Some(ref cfg) = &cfg {
                        self.cfg_stack.push(cfg.clone());
                    }

                    if self.should_parse_dependency(&dep_pkg_name) {
                        if self.lib.is_some() {
                            let dep_pkg_ref =
                                self.lib.as_ref().unwrap().find_dep_ref(pkg, &dep_pkg_name);

                            if let Some(dep_pkg_ref) = dep_pkg_ref {
                                self.parse_crate(&dep_pkg_ref)?;
                            } else {
                                error!(
                                    "Parsing crate `{}`: can't find dependency version for `{}`.",
                                    pkg.name, dep_pkg_name
                                );
                            }
                        } else {
                            error!(
                                "Parsing crate `{}`: cannot parse external crate `{}` because \
                                 cbindgen is in single source mode. Consider specifying a crate \
                                 directory instead of a source file.",
                                pkg.name, dep_pkg_name
                            );
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
        let mut add_opaque = |name: &str, generic_params: Vec<&str>| {
            self.opaque_items.try_insert(OpaqueItem {
                name: name.to_owned(),
                generic_params: GenericParams(
                    generic_params.iter().map(|x| (*x).to_owned()).collect(),
                ),
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
        binding_crate_name: &str,
        crate_name: &str,
        mod_cfg: &Option<Cfg>,
        items: &[syn::Item],
    ) {
        for item in items {
            if item.has_test_attr() {
                continue;
            }
            match item {
                &syn::Item::ForeignMod(ref item) => {
                    self.load_syn_foreign_mod(binding_crate_name, crate_name, mod_cfg, item);
                }
                &syn::Item::Fn(ref item) => {
                    self.load_syn_fn(binding_crate_name, crate_name, mod_cfg, item);
                }
                &syn::Item::Const(ref item) => {
                    self.load_syn_const(binding_crate_name, crate_name, mod_cfg, item);
                }
                &syn::Item::Static(ref item) => {
                    self.load_syn_static(binding_crate_name, crate_name, mod_cfg, item);
                }
                &syn::Item::Struct(ref item) => {
                    self.load_syn_struct(crate_name, mod_cfg, item);
                }
                &syn::Item::Union(ref item) => {
                    self.load_syn_union(crate_name, mod_cfg, item);
                }
                &syn::Item::Enum(ref item) => {
                    self.load_syn_enum(crate_name, mod_cfg, item);
                }
                &syn::Item::Type(ref item) => {
                    self.load_syn_ty(crate_name, mod_cfg, item);
                }
                &syn::Item::Impl(ref item_impl) => {
                    for item in &item_impl.items {
                        match item {
                            &syn::ImplItem::Const(ref item) => {
                                self.load_syn_assoc_const(
                                    binding_crate_name,
                                    crate_name,
                                    mod_cfg,
                                    item,
                                );
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }

    /// Enters a `extern "C" { }` declaration and loads function declarations.
    fn load_syn_foreign_mod(
        &mut self,
        binding_crate_name: &str,
        crate_name: &str,
        mod_cfg: &Option<Cfg>,
        item: &syn::ItemForeignMod,
    ) {
        if !item.abi.is_c() {
            info!("Skip {} - (extern block must be extern C).", crate_name);
            return;
        }

        for foreign_item in &item.items {
            match foreign_item {
                &syn::ForeignItem::Fn(ref function) => {
                    if crate_name != binding_crate_name {
                        info!(
                            "Skip {}::{} - (fn's outside of the binding crate are not used).",
                            crate_name, &function.ident
                        );
                        return;
                    }

                    match Function::load(
                        function.ident.to_string(),
                        &function.decl,
                        true,
                        &function.attrs,
                        mod_cfg,
                    ) {
                        Ok(func) => {
                            info!("Take {}::{}.", crate_name, &function.ident);

                            self.functions.push(func);
                        }
                        Err(msg) => {
                            error!(
                                "Cannot use fn {}::{} ({}).",
                                crate_name, &function.ident, msg
                            );
                        }
                    }
                }
                _ => {}
            }
        }
    }

    /// Loads a `fn` declaration
    fn load_syn_fn(
        &mut self,
        binding_crate_name: &str,
        crate_name: &str,
        mod_cfg: &Option<Cfg>,
        item: &syn::ItemFn,
    ) {
        if crate_name != binding_crate_name {
            info!(
                "Skip {}::{} - (fn's outside of the binding crate are not used).",
                crate_name, &item.ident
            );
            return;
        }

        if let syn::Visibility::Public(_) = item.vis {
            if item.is_no_mangle() && (item.abi.is_omitted() || item.abi.is_c()) {
                match Function::load(
                    item.ident.to_string(),
                    &item.decl,
                    false,
                    &item.attrs,
                    mod_cfg,
                ) {
                    Ok(func) => {
                        info!("Take {}::{}.", crate_name, &item.ident);

                        self.functions.push(func);
                    }
                    Err(msg) => {
                        error!("Cannot use fn {}::{} ({}).", crate_name, &item.ident, msg);
                    }
                }
                return;
            }
        }

        // TODO
        if let syn::Visibility::Public(_) = item.vis {
        } else {
            warn!("Skip {}::{} - (not `pub`).", crate_name, &item.ident);
        }
        if (item.abi.is_omitted() || item.abi.is_c()) && !item.is_no_mangle() {
            warn!(
                "Skip {}::{} - (`extern` but not `no_mangle`).",
                crate_name, &item.ident
            );
        }
        if item.abi.is_some() && !(item.abi.is_omitted() || item.abi.is_c()) {
            warn!(
                "Skip {}::{} - (non `extern \"C\"`).",
                crate_name, &item.ident
            );
        }
    }

    /// Loads an associated `const` declaration
    fn load_syn_assoc_const(
        &mut self,
        binding_crate_name: &str,
        crate_name: &str,
        mod_cfg: &Option<Cfg>,
        item: &syn::ImplItemConst,
    ) {
        if crate_name != binding_crate_name {
            info!(
                "Skip {}::{} - (const's outside of the binding crate are not used).",
                crate_name, &item.ident
            );
            return;
        }

        let const_name = item.ident.to_string();

        match Constant::load_assoc(const_name.clone(), item, mod_cfg) {
            Ok(constant) => {
                info!("Take {}::{}.", crate_name, &item.ident);

                self.constants.try_insert(constant);
            }
            Err(msg) => {
                warn!("Skip {}::{} - ({})", crate_name, &item.ident, msg);
            }
        }
    }

    /// Loads a `const` declaration
    fn load_syn_const(
        &mut self,
        binding_crate_name: &str,
        crate_name: &str,
        mod_cfg: &Option<Cfg>,
        item: &syn::ItemConst,
    ) {
        if crate_name != binding_crate_name {
            info!(
                "Skip {}::{} - (const's outside of the binding crate are not used).",
                crate_name, &item.ident
            );
            return;
        }

        let const_name = item.ident.to_string();

        match Constant::load(const_name.clone(), item, mod_cfg) {
            Ok(constant) => {
                info!("Take {}::{}.", crate_name, &item.ident);

                self.constants.try_insert(constant);
            }
            Err(msg) => {
                warn!("Skip {}::{} - ({})", crate_name, &item.ident, msg);
            }
        }
    }

    /// Loads a `static` declaration
    fn load_syn_static(
        &mut self,
        binding_crate_name: &str,
        crate_name: &str,
        mod_cfg: &Option<Cfg>,
        item: &syn::ItemStatic,
    ) {
        if crate_name != binding_crate_name {
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
    fn load_syn_struct(&mut self, crate_name: &str, mod_cfg: &Option<Cfg>, item: &syn::ItemStruct) {
        match Struct::load(item, mod_cfg) {
            Ok(st) => {
                info!("Take {}::{}.", crate_name, &item.ident);

                self.structs.try_insert(st);
            }
            Err(msg) => {
                info!("Take {}::{} - opaque ({}).", crate_name, &item.ident, msg);
                self.opaque_items.try_insert(OpaqueItem::new(
                    item.ident.to_string(),
                    &item.generics,
                    &item.attrs,
                    mod_cfg,
                ));
            }
        }
    }

    /// Loads a `union` declaration
    fn load_syn_union(&mut self, crate_name: &str, mod_cfg: &Option<Cfg>, item: &syn::ItemUnion) {
        match Union::load(item, mod_cfg) {
            Ok(st) => {
                info!("Take {}::{}.", crate_name, &item.ident);

                self.unions.try_insert(st);
            }
            Err(msg) => {
                info!("Take {}::{} - opaque ({}).", crate_name, &item.ident, msg);
                self.opaque_items.try_insert(OpaqueItem::new(
                    item.ident.to_string(),
                    &item.generics,
                    &item.attrs,
                    mod_cfg,
                ));
            }
        }
    }

    /// Loads a `enum` declaration
    fn load_syn_enum(&mut self, crate_name: &str, mod_cfg: &Option<Cfg>, item: &syn::ItemEnum) {
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
                self.opaque_items.try_insert(OpaqueItem::new(
                    item.ident.to_string(),
                    &item.generics,
                    &item.attrs,
                    mod_cfg,
                ));
            }
        }
    }

    /// Loads a `type` declaration
    fn load_syn_ty(&mut self, crate_name: &str, mod_cfg: &Option<Cfg>, item: &syn::ItemType) {
        match Typedef::load(item, mod_cfg) {
            Ok(st) => {
                info!("Take {}::{}.", crate_name, &item.ident);

                self.typedefs.try_insert(st);
            }
            Err(msg) => {
                info!("Take {}::{} - opaque ({}).", crate_name, &item.ident, msg);
                self.opaque_items.try_insert(OpaqueItem::new(
                    item.ident.to_string(),
                    &item.generics,
                    &item.attrs,
                    mod_cfg,
                ));
            }
        }
    }
}
