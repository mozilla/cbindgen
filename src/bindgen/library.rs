use std::io;
use std::io::Write;
use std::collections::BTreeMap;
use std::collections::HashSet;

use syn::*;

use rust_lib;
use bindgen::items::*;
use bindgen::syn_helpers::*;

pub type ConvertResult<T> = Result<T, String>;
pub type BuildResult<T> = Result<T, String>;

pub type PathRef = String;
#[derive(Debug, Clone)]
pub enum PathValue {
    Enum(Enum),
    Struct(Struct),
    OpaqueStruct(OpaqueStruct),
    Typedef(Typedef),
    Specialization(Specialization),
    Prebuilt(Prebuilt),
}
impl PathValue {
    pub fn name(&self) -> &String {
        match self {
            &PathValue::Enum(ref x) => { &x.name },
            &PathValue::Struct(ref x) => { &x.name },
            &PathValue::OpaqueStruct(ref x) => { &x.name },
            &PathValue::Typedef(ref x) => { &x.name },
            &PathValue::Specialization(ref x) => { &x.name },
            &PathValue::Prebuilt(ref x) => { &x.name },
        }
    }

    pub fn add_deps(&self, library: &Library, out: &mut Vec<PathValue>) {
        match self {
            &PathValue::Enum(_) => { },
            &PathValue::Struct(ref x) => { x.add_deps(library, out); },
            &PathValue::OpaqueStruct(_) => { },
            &PathValue::Typedef(ref x) => { x.add_deps(library, out); },
            &PathValue::Specialization(ref x) => { x.add_deps(library, out); },
            &PathValue::Prebuilt(_) => { },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Prebuilt {
    pub name: String,
    pub source: String,
}
impl Prebuilt {
    pub fn new(name: String, source: String) -> Prebuilt {
        Prebuilt {
            name: name,
            source: source,
        }
    }

    fn write<F: Write>(&self, out: &mut F) {
        write!(out, "{}", self.source).unwrap();
    }
}

/// A library collects all of the information needed to generate
/// bindings for a specified rust library. It is turned into a
/// BuiltLibrary, and in the process filters out unneeded information
/// and in the future will do validation.
#[derive(Debug, Clone)]
pub struct Library {
    enums: BTreeMap<String, Enum>,
    structs: BTreeMap<String, Struct>,
    opaque_structs: BTreeMap<String, OpaqueStruct>,
    typedefs: BTreeMap<String, Typedef>,
    specializations: BTreeMap<String, Specialization>,
    prebuilts: BTreeMap<String, Prebuilt>,
    functions: BTreeMap<String, Function>,
}

impl Library {
    fn blank() -> Library {
        Library {
            enums: BTreeMap::new(),
            structs: BTreeMap::new(),
            opaque_structs: BTreeMap::new(),
            typedefs: BTreeMap::new(),
            specializations: BTreeMap::new(),
            prebuilts: BTreeMap::new(),
            functions: BTreeMap::new(),
        }
    }

    pub fn load(crate_or_src: &str,
                prebuilts: Vec<Prebuilt>,
                ignore: HashSet<String>) -> Library
    {
        let mut library = Library::blank();

        rust_lib::parse(crate_or_src, &mut |mod_name, items| {
            for item in items {
                if ignore.contains(&item.ident.to_string()) {
                    continue;
                }

                match item.node {
                    ItemKind::Fn(ref decl,
                                 ref _unsafe,
                                 ref _const,
                                 ref abi,
                                 ref _generic,
                                 ref _block) => {
                        if item.is_no_mangle() && abi.is_c() {
                            match Function::convert(item.ident.to_string(), item.is_wr_destructor_safe(), decl) {
                                Ok(func) => {
                                    writeln!(io::stderr(), "processed function       {}::{}", mod_name, &item.ident).unwrap();

                                    library.functions.insert(func.name.clone(), func);
                                }
                                Err(msg) => {
                                    writeln!(io::stderr(), "skipping function        {}::{} - {}", mod_name, &item.ident, msg).unwrap();
                                },
                            }
                        }
                    }
                    ItemKind::Struct(ref variant,
                                     ref generics) => {
                        let struct_name = item.ident.to_string();

                        if item.is_repr_c() {
                            match Struct::convert(struct_name.clone(), variant, generics) {
                                Ok(st) => {
                                    writeln!(io::stderr(), "processed struct         {}::{}", mod_name, &item.ident).unwrap();
                                    library.structs.insert(struct_name,
                                                           st);
                                }
                                Err(msg) => {
                                    writeln!(io::stderr(), "processed opaque struct  {}::{} - {}", mod_name, &item.ident, msg).unwrap();
                                    library.opaque_structs.insert(struct_name.clone(),
                                                                  OpaqueStruct::new(struct_name));
                                }
                            }
                        } else {
                            writeln!(io::stderr(), "processed opaque struct  {}::{}  - not marked as repr(C)", mod_name, &item.ident).unwrap();
                            library.opaque_structs.insert(struct_name.clone(),
                                                          OpaqueStruct::new(struct_name));
                        }
                    }
                    ItemKind::Enum(ref variants, ref generics) => {
                        if !generics.lifetimes.is_empty() ||
                           !generics.ty_params.is_empty() ||
                           !generics.where_clause.predicates.is_empty() {
                            writeln!(io::stderr(), "skipping enum            {}::{} - has generics or lifetimes or where bounds", mod_name, &item.ident).unwrap();
                            continue;
                        }

                        if item.is_repr_u32() {
                            let enum_name = item.ident.to_string();

                            match Enum::convert(enum_name.clone(), variants) {
                                Ok(en) => {
                                    writeln!(io::stderr(), "processed enum           {}::{}", mod_name, &item.ident).unwrap();
                                    library.enums.insert(enum_name, en);
                                }
                                Err(msg) => {
                                    writeln!(io::stderr(), "skipping enum            {}::{} - {}", mod_name, &item.ident, msg).unwrap();
                                }
                            }
                        } else {
                            writeln!(io::stderr(), "skipping enum            {}::{} - not marked as repr(u32)", mod_name, &item.ident).unwrap();
                        }
                    }
                    ItemKind::Ty(ref ty, ref generics) => {
                        if !generics.lifetimes.is_empty() ||
                           !generics.ty_params.is_empty() ||
                           !generics.where_clause.predicates.is_empty() {
                            writeln!(io::stderr(), "skipping type alias      {}::{} - has generics or lifetimes or where bounds", mod_name, &item.ident).unwrap();
                            continue;
                        }

                        let alias_name = item.ident.to_string();

                        let fail1 = match Specialization::convert(alias_name.clone(), ty) {
                            Ok(spec) => {
                                writeln!(io::stderr(), "processed specialization {}::{}", mod_name, &item.ident).unwrap();
                                library.specializations.insert(alias_name, spec);
                                continue;
                            }
                            Err(msg) => msg,
                        };
                        let fail2 = match Typedef::convert(alias_name.clone(), ty) {
                            Ok(typedef) => {
                                writeln!(io::stderr(), "processed typedef        {}::{}", mod_name, &item.ident).unwrap();
                                library.typedefs.insert(alias_name, typedef);
                                continue;
                            }
                            Err(msg) => msg,
                        };
                        writeln!(io::stderr(), "skipping type alias      {}::{} - {} and {}", mod_name, &item.ident, fail1, fail2).unwrap();
                    }
                    _ => {}
                }
            }
        });

        for prebuilt in prebuilts {
            library.prebuilts.insert(prebuilt.name.clone(), prebuilt);
        }

        library
    }

    pub fn resolve_path(&self, p: &PathRef) -> Option<PathValue> {
        // Search the prebuilts first, allow them to override
        if let Some(x) = self.prebuilts.get(p) {
            return Some(PathValue::Prebuilt(x.clone()));
        }

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
            writeln!(io::stderr(), "warning, can't find {}", p).unwrap();
        }
    }

    pub fn add_deps_for_path_deps(&self, p: &PathRef, out: &mut Vec<PathValue>) {
        if let Some(value) = self.resolve_path(p) {
            value.add_deps(self, out);
        } else {
            writeln!(io::stderr(), "warning, can't find {}", p).unwrap();
        }
    }

    pub fn build(&self) -> BuildResult<BuiltLibrary> {
        let mut result = BuiltLibrary::blank();

        // Gather only the items that we need for this
        // `extern "c"` interface
        let mut deps = Vec::new();
        for (_, function) in &self.functions {
            function.add_deps(self, &mut deps);
        }

        // Copy the binding items in dependencies order
        // into the BuiltLibrary, specializing any type
        // aliases we encounter
        for dep in deps {
            match &dep {
                &PathValue::Struct(ref s) => {
                    if !s.generic_params.is_empty() {
                        continue;
                    }
                }
                &PathValue::Specialization(ref s) => {
                    match s.specialize(self) {
                        Ok(value) => {
                            result.items.push(value);
                        }
                        Err(msg) => {
                            writeln!(io::stderr(), "error specializing {} - {}", dep.name(), msg).unwrap();
                        }
                    }
                    continue;
                }
                _ => { }
            }
            result.items.push(dep);
        }

        result.functions = self.functions.iter()
                                         .map(|(_, function)| function.clone())
                                         .collect::<Vec<_>>();

        Ok(result)
    }
}

/// A BuiltLibrary represents a completed bindings file ready to be printed.
#[derive(Debug, Clone)]
pub struct BuiltLibrary {
    items: Vec<PathValue>,
    functions: Vec<Function>,
}

impl BuiltLibrary {
    fn blank() -> BuiltLibrary {
        BuiltLibrary {
            items: Vec::new(),
            functions: Vec::new(),
        }
    }

    pub fn write<F: Write>(&self, out: &mut F) {
        writeln!(out,
r###"/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
"###).unwrap();

        writeln!(out, "/* DO NOT MODIFY THIS MANUALLY! This file was generated using cbindgen. */\n").unwrap();

        for item in &self.items {
            match item {
                &PathValue::Enum(ref x) => x.write(out),
                &PathValue::Struct(ref x) => x.write(out),
                &PathValue::OpaqueStruct(ref x) => x.write(out),
                &PathValue::Typedef(ref x) => x.write(out),
                &PathValue::Specialization(_) => {
                    panic!("should not encounter a specialization in a built library")
                }
                &PathValue::Prebuilt(ref x) => x.write(out),
            }
            write!(out, "\n\n").unwrap();
        }

        writeln!(out, "/* DO NOT MODIFY THIS MANUALLY! This file was generated using cbindgen. */\n").unwrap();

        for function in &self.functions {
            function.write(out);
            write!(out, "\n\n").unwrap();
        }

        writeln!(out, "/* DO NOT MODIFY THIS MANUALLY! This file was generated using cbindgen. */").unwrap();
    }
}
