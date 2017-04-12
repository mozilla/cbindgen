use std::io;
use std::io::Write;
use std::collections::BTreeMap;
use std::collections::HashSet;

use syn::*;
use rust_lib;

trait SynItemHelpers {
    fn has_attr(&self, target: MetaItem) -> bool;
    fn has_doc_attr_with(&self, value: &str) -> bool;

    fn is_no_mangle(&self) -> bool {
        self.has_attr(MetaItem::Word(Ident::new("no_mangle")))
    }
    fn is_repr_c(&self) -> bool {
        let repr_args = vec![NestedMetaItem::MetaItem(MetaItem::Word(Ident::new("C")))];
        self.has_attr(MetaItem::List(Ident::new("repr"), repr_args))
    }
    fn is_repr_u32(&self) -> bool {
        let repr_args = vec![NestedMetaItem::MetaItem(MetaItem::Word(Ident::new("u32")))];
        self.has_attr(MetaItem::List(Ident::new("repr"), repr_args))
    }

    fn is_wr_destructor_safe(&self) -> bool {
        self.has_doc_attr_with("wr-binding:destructor_safe")
    }
}
impl SynItemHelpers for Item {
    fn has_attr(&self, target: MetaItem) -> bool {
        return self.attrs
                   .iter()
                   .any(|ref attr| attr.style == AttrStyle::Outer && attr.value == target);
    }
    fn has_doc_attr_with(&self, target: &str) -> bool {
        self.attrs.iter().any(|ref attr| {
                     if attr.style == AttrStyle::Outer && attr.is_sugared_doc {
                         if let MetaItem::NameValue(_, Lit::Str(ref comment, _)) = attr.value {
                             return comment.contains(target);
                         }
                     }
                     false
                 })
    }
}

trait SynAbiHelpers {
    fn is_c(&self) -> bool;
}
impl SynAbiHelpers for Option<Abi> {
    fn is_c(&self) -> bool {
        self == &Some(Abi::Named(String::from("C")))
    }
}

trait SynFnRetTyHelpers {
    fn as_type(&self) -> ConvertResult<Option<Type>>;
}
impl SynFnRetTyHelpers for FunctionRetTy {
    fn as_type(&self) -> ConvertResult<Option<Type>> {
        match self {
            &FunctionRetTy::Default => Ok(None),
            &FunctionRetTy::Ty(ref t) => Ok(Some(try!(Type::convert(t)))),
        }
    }
}

trait SynFnArgHelpers {
    fn as_ident_and_type(&self) -> ConvertResult<(String, Type)>;
}
impl SynFnArgHelpers for FnArg {
    fn as_ident_and_type(&self) -> ConvertResult<(String, Type)> {
        match self {
            &FnArg::Captured(Pat::Ident(_, ref ident, _), ref ty) => {
                Ok((ident.to_string(), try!(Type::convert(ty))))
            }
            _ => Err(format!("unexpected param type")),
        }
    }
}

trait SynFieldHelpers {
    fn as_ident_and_type(&self) -> ConvertResult<(String, Type)>;
}
impl SynFieldHelpers for Field {
    fn as_ident_and_type(&self) -> ConvertResult<(String, Type)> {
        let ident = try!(self.ident.as_ref().ok_or(format!("missing ident"))).clone();
        let converted_ty = try!(Type::convert(&self.ty));

        Ok((ident.to_string(), converted_ty))
    }
}

// TODO: find a better place for these
fn map_path_name_to_primitive(path_name: &str) -> String {
    match path_name {
        "usize" => "size_t".to_string(),
        "u8" => "uint8_t".to_string(),
        "u32" => "uint32_t".to_string(),
        "u64" => "uint64_t".to_string(),
        "i8" => "int8_t".to_string(),
        "i32" => "int32_t".to_string(),
        "i64" => "int64_t".to_string(),
        "f32" => "float".to_string(),
        "f64" => "double".to_string(),
        "c_void" => "void".to_string(),
        "bool" => "bool".to_string(),
        _ => path_name.to_string(),
    }
}
fn convert_path_name_to_primitive(path_name: &str) -> Option<String> {
    match path_name {
        "usize" => Some("size_t".to_string()),
        "u8" => Some("uint8_t".to_string()),
        "u32" => Some("uint32_t".to_string()),
        "u64" => Some("uint64_t".to_string()),
        "i8" => Some("int8_t".to_string()),
        "i32" => Some("int32_t".to_string()),
        "i64" => Some("int64_t".to_string()),
        "f32" => Some("float".to_string()),
        "f64" => Some("double".to_string()),
        "c_void" => Some("void".to_string()),
        "bool" => Some("bool".to_string()),
        _ => None,
    }
}
fn path_name_is_primitive(path_name: &str) -> bool {
    match path_name {
        "usize" => true,
        "u8" => true,
        "u32" => true,
        "u64" => true,
        "i8" => true,
        "i32" => true,
        "i64" => true,
        "f32" => true,
        "f64" => true,
        "c_void" => true,
        "bool" => true,
        _ => false,
    }
}

trait SynPathHelpers {
    fn convert_to_simple_single_segment(&self) -> Option<String>;
    fn convert_to_generic_single_segment(&self) -> Option<(String, Vec<String>)>;
}
impl SynPathHelpers for Path {
    fn convert_to_simple_single_segment(&self) -> Option<String> {
        if self.segments.len() != 1 {
            return None;
        }

        match &self.segments[0].parameters {
            &PathParameters::AngleBracketed(ref d) => {
                if !d.lifetimes.is_empty() ||
                   !d.types.is_empty() ||
                   !d.bindings.is_empty() {
                    return None;
                }
            }
            &PathParameters::Parenthesized(_) => {
                return None;
            }
        }

        let name = self.segments[0].ident.to_string();

        Some(name)
    }

    fn convert_to_generic_single_segment(&self) -> Option<(String, Vec<String>)> {
        if self.segments.len() != 1 {
            return None;
        }

        let generics = match &self.segments[0].parameters {
            &PathParameters::AngleBracketed(ref d) => {
                if !d.lifetimes.is_empty() ||
                   !d.bindings.is_empty() {
                    return None;
                }

                let mut generics = Vec::new();
                for ty in &d.types {
                    match ty {
                        &Ty::Path(_, ref p) => {
                            match p.convert_to_simple_single_segment() {
                                Some(path) => generics.push(path),
                                None => return None,
                            }
                        },
                        _ => { return None },
                    }
                }
                generics
            }
            &PathParameters::Parenthesized(_) => {
                return None;
            }
        };

        let name = self.segments[0].ident.to_string();

        Some((name, generics))
    }
}

type ConvertResult<T> = Result<T, String>;
pub type BuildResult<T> = Result<T, String>;

type PathRef = String;
#[derive(Debug, Clone)]
enum PathValue {
    Enum(Enum),
    Struct(Struct),
    OpaqueStruct(OpaqueStruct),
    Typedef(Typedef),
    Specialization(Specialization),
    Prebuilt(Prebuilt),
}
impl PathValue {
    fn name(&self) -> &String {
        match self {
            &PathValue::Enum(ref x) => { &x.name },
            &PathValue::Struct(ref x) => { &x.name },
            &PathValue::OpaqueStruct(ref x) => { &x.name },
            &PathValue::Typedef(ref x) => { &x.name },
            &PathValue::Specialization(ref x) => { &x.name },
            &PathValue::Prebuilt(ref x) => { &x.name },
        }
    }

    fn add_deps(&self, library: &Library, out: &mut Vec<PathValue>) {
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
    name: String,
    source: String,
}
impl Prebuilt {
    pub fn new(name: String, source: String) -> Prebuilt {
        Prebuilt {
            name: name,
            source: source,
        }
    }

    fn generate(&self) -> String {
        self.source.clone()
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

    pub fn load(crate_or_src: &str, prebuilts: Vec<Prebuilt>, ignore: HashSet<String>) -> Library {
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

    fn resolve_path(&self, p: &PathRef) -> Option<PathValue> {
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

    fn add_deps_for_path(&self, p: &PathRef, out: &mut Vec<PathValue>) {
        if let Some(value) = self.resolve_path(p) {
            value.add_deps(self, out);

            if !out.iter().any(|x| x.name() == value.name()) {
                out.push(value);
            }
        } else {
            writeln!(io::stderr(), "warning, can't find {}", p).unwrap();
        }
    }

    fn add_deps_for_path_deps(&self, p: &PathRef, out: &mut Vec<PathValue>) {
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

    pub fn generate(&self) -> String {
        let mut result = String::new();

        result.push_str(r###"/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

"###);
        result.push_str("/* THIS FILE IS GENERATED! DO NOT MODIFY MANUALLY! See https://github.com/jrmuizel/wr-binding! */");

        for item in &self.items {
            result.push_str(&match item {
                &PathValue::Enum(ref x) => x.generate(),
                &PathValue::Struct(ref x) => x.generate(),
                &PathValue::OpaqueStruct(ref x) => x.generate(),
                &PathValue::Typedef(ref x) => x.generate(),
                &PathValue::Specialization(_) => {
                    panic!("should not encounter a specialization in a built library")
                }
                &PathValue::Prebuilt(ref x) => x.generate(),
            });
            result.push_str("\n\n");
        }

        result.push_str("/* THIS FILE IS GENERATED! DO NOT MODIFY MANUALLY! See https://github.com/jrmuizel/wr-binding! */");
        result.push_str("\n");

        for function in &self.functions {
            result.push_str(&function.generate());
            result.push_str("\n\n");
        }

        result.push_str("/* THIS FILE IS GENERATED! DO NOT MODIFY MANUALLY! See https://github.com/jrmuizel/wr-binding! */");
        result.push_str("\n");

        result
    }
}

#[derive(Debug, Clone)]
struct Struct {
    name: String,
    fields: Vec<(String, Type)>,
    generic_params: Vec<PathRef>,
}

impl Struct {
    fn convert(name: String, decl: &VariantData, generics: &Generics) -> ConvertResult<Struct> {
        let fields = match decl {
            &VariantData::Struct(ref fields) => {
                fields.iter()
                      .map(|x| x.as_ident_and_type().ok())
                      .collect::<Vec<_>>()
            }
            &VariantData::Tuple(ref fields) => {
                let mut out = Vec::new();
                let mut current = 0;
                for field in fields {
                    out.push(Some((format!("m{}", current),
                                   try!(Type::convert(&field.ty)))));
                    current += 1;
                }
                out
            }
            &VariantData::Unit => {
                vec![]
            }
        };

        let generic_params = generics.ty_params.iter()
                                               .map(|x| x.ident.to_string())
                                               .collect::<Vec<_>>();

        if fields.iter().all(|x| x.is_some()) {
            Ok(Struct {
                name: name,
                fields: fields.iter().filter_map(|x| x.clone()).collect(),
                generic_params: generic_params,
            })
        } else {
            Err(format!("one of the fields failed to convert"))
        }
    }

    fn add_deps(&self, library: &Library, out: &mut Vec<PathValue>) {
        for &(_, ref ty) in &self.fields {
            ty.add_deps(library, out);
        }
    }

    fn generate(&self) -> String {
        let fields = self.fields.iter()
                           .map(|x| format!("  {};", x.1.generate_with_ident(&x.0)))
                           .collect::<Vec<_>>()
                           .join("\n");

        let op = format!("  bool operator==(const {}& aOther) const {{\n    return {};\n  }}",
            self.name,
            self.fields.iter()
                       .map(|x| format!("{} == aOther.{}", x.0, x.0))
                       .collect::<Vec<_>>()
                       .join(" &&\n      "));

        format!("struct {} {{\n{}\n\n{}\n}};", self.name, fields, op)
    }
}

#[derive(Debug, Clone)]
struct OpaqueStruct {
    name: PathRef,
}

impl OpaqueStruct {
    fn new(name: String) -> OpaqueStruct {
        OpaqueStruct {
            name: name,
        }
    }

    fn generate(&self) -> String {
        format!("struct {};", self.name)
    }
}

#[derive(Debug, Clone)]
struct Enum {
    name: String,
    values: Vec<(String, u64)>,
}

impl Enum {
    fn convert(name: String, variants: &Vec<Variant>) -> ConvertResult<Enum> {
        let mut values = Vec::new();
        let mut current = 0;

        for variant in variants {
            match variant.data {
                VariantData::Unit => {
                    match variant.discriminant {
                        Some(ConstExpr::Lit(Lit::Int(i, _))) => {
                            current = i;
                        }
                        Some(_) => {
                            return Err(format!("unsupported discriminant"));
                        }
                        None => { /* okay, we just use current */ }
                    }

                    values.push((variant.ident.to_string(), current));
                    current = current + 1;
                }
                _ => {
                    return Err(format!("unsupported variant"));
                }
            }
        }

        Ok(Enum {
            name: name,
            values: values,
        })
    }

    fn generate(&self) -> String {
        let mut values = self.values.iter()
                                    .map(|x| format!("  {} = {},", x.0, x.1))
                                    .collect::<Vec<_>>()
                                    .join("\n");
        values.push_str("\n\n  Sentinel /* this must be last for serialization purposes. */");

        format!("enum class {} : uint32_t {{\n{}\n}};",
                self.name,
                values)
    }
}

#[derive(Debug, Clone)]
struct Function {
    name: String,
    wr_destructor_safe: bool,
    return_ty: Option<Type>,
    args: Vec<(String, Type)>,
}

impl Function {
    fn convert(name: String, wr_destructor_safe: bool, decl: &FnDecl) -> ConvertResult<Function> {
        let args = decl.inputs.iter()
                              .map(|x| x.as_ident_and_type().ok())
                              .collect::<Vec<_>>();
        let ret = try!(decl.output.as_type());

        if args.iter().all(|x| x.is_some()) {
            Ok(Function {
                name: name,
                wr_destructor_safe: wr_destructor_safe,
                return_ty: ret,
                args: args.iter().filter_map(|x| x.clone()).collect(),
            })
        } else {
            Err(format!("one of the params failed to convert"))
        }
    }

    fn add_deps(&self, library: &Library, out: &mut Vec<PathValue>) {
        if let &Some(ref ty) = &self.return_ty {
            ty.add_deps(library, out);
        }
        for &(_, ref arg) in &self.args {
            arg.add_deps(library, out);
        }
    }

    fn generate(&self) -> String {
        format!("WR_INLINE {}\n{}({})\n{};",
                self.return_ty.as_ref().map_or(format!("void"), |x| { x.generate() }),
                self.name,
                self.args.iter()
                         .map(|x| {
                            format!("{}", x.1.generate_with_ident(&x.0))
                         })
                         .collect::<Vec<_>>()
                         .join(",\n    "),
                if self.wr_destructor_safe {
                    "WR_DESTRUCTOR_SAFE_FUNC"
                } else {
                    "WR_FUNC"
                })
    }
}

#[derive(Debug, Clone)]
struct Specialization {
    name: String,
    aliased: PathRef,
    generic_values: Vec<PathRef>,
}
impl Specialization {
    fn convert(name: String, ty: &Ty) -> ConvertResult<Specialization> {
        match ty {
            &Ty::Path(ref _q, ref p) => {
                let (path, generics) = try!(p.convert_to_generic_single_segment().ok_or("not a generic single segment"));

                if path_name_is_primitive(&path) {
                    return Err(format!("can't specialize a primitive"));
                }

                Ok(Specialization {
                    name: name,
                    aliased: path,
                    generic_values: generics.iter()
                                            .map(|x| map_path_name_to_primitive(x))
                                            .collect(),
                })
            }
            _ => {
                Err(format!("not a path"))
            }
        }
    }

    fn add_deps(&self, library: &Library, out: &mut Vec<PathValue>) {
        if !path_name_is_primitive(&self.aliased) {
            library.add_deps_for_path_deps(&self.aliased, out);
        }
        for value in &self.generic_values {
            if !path_name_is_primitive(value) {
                library.add_deps_for_path(value, out);
            }
        }
    }

    fn specialize(&self, library: &Library) -> ConvertResult<PathValue> {
        match library.resolve_path(&self.aliased) {
            Some(aliased) => {
                match aliased {
                    PathValue::OpaqueStruct(_) => {
                        Ok(PathValue::OpaqueStruct(OpaqueStruct {
                            name: self.name.clone(),
                        }))
                    }
                    PathValue::Struct(aliased) => {
                        if self.generic_values.len() !=
                           aliased.generic_params.len() {
                            return Err(format!("incomplete specialization"));
                        }

                        let mappings = aliased.generic_params.iter()
                                                             .zip(self.generic_values.iter())
                                                             .collect::<Vec<_>>();
                        Ok(PathValue::Struct(Struct {
                            name: self.name.clone(),
                            fields: aliased.fields.iter()
                                                  .map(|x| (x.0.clone(), x.1.specialize(&mappings)))
                                                  .collect(),
                            generic_params: vec![],
                        }))
                    }
                    PathValue::Enum(aliased) => {
                        Ok(PathValue::Enum(Enum {
                            name: self.name.clone(),
                            values: aliased.values.clone(),
                        }))
                    }
                    PathValue::Prebuilt(aliased) => {
                        Ok(PathValue::Prebuilt(Prebuilt {
                            // assume that the prebuilt has the right name
                            name: aliased.name,
                            source: aliased.source,
                        }))
                    }
                    _ => Err(format!("unknown type to specialize"))
                }
            }
            None => {
                Err(format!("couldn't find aliased type"))
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Typedef {
    name: String,
    aliased: Type,
}
impl Typedef {
    fn convert(name: String, ty: &Ty) -> ConvertResult<Typedef> {
        Ok(Typedef {
            name: name,
            aliased: try!(Type::convert(ty)),
        })
    }

    fn add_deps(&self, library: &Library, out: &mut Vec<PathValue>) {
        self.aliased.add_deps(library, out);
    }

    fn generate(&self) -> String {
        format!("typedef {};",
                self.aliased.generate_with_ident(&self.name))
    }
}

#[derive(Debug, Clone)]
enum Type {
    ConstPtr(Box<Type>),
    Ptr(Box<Type>),
    Path(PathRef),
    Primitive(String),
    Array(Box<Type>, u64),
    FuncPtr(Option<Box<Type>>, Vec<Type>),
}
impl Type {
    fn convert(ty: &Ty) -> ConvertResult<Type> {
        match ty {
            &Ty::Rptr(_, ref mut_ty) => {
                let converted = try!(Type::convert(&mut_ty.ty));

                Ok(match mut_ty.mutability {
                    Mutability::Mutable => Type::Ptr(Box::new(converted)),
                    Mutability::Immutable => Type::ConstPtr(Box::new(converted)),
                })
            }
            &Ty::Ptr(ref mut_ty) => {
                let converted = try!(Type::convert(&mut_ty.ty));

                Ok(match mut_ty.mutability {
                    Mutability::Mutable => Type::Ptr(Box::new(converted)),
                    Mutability::Immutable => Type::ConstPtr(Box::new(converted)),
                })
            }
            &Ty::Path(_, ref p) => {
                match p.convert_to_simple_single_segment() {
                    Some(p) => {
                        if let Some(prim) = convert_path_name_to_primitive(&p) {
                            Ok(Type::Primitive(prim))
                        } else {
                            Ok(Type::Path(p))
                        }
                    }
                    None => Err(format!("not a simple single segment")),
                }
            }
            &Ty::Array(ref ty, ConstExpr::Lit(Lit::Int(sz, _))) => {
                let converted = try!(Type::convert(ty));

                Ok(Type::Array(Box::new(converted), sz))
            },
            &Ty::BareFn(ref f) => {
                let args = f.inputs.iter()
                                   .map(|x| Type::convert(&x.ty).ok())
                                   .collect::<Vec<_>>();
                let ret = try!(f.output.as_type());

                if args.iter().all(|x| x.is_some()) {
                    Ok(Type::FuncPtr(
                        ret.map(|x| Box::new(x)),
                        args.iter().filter_map(|x| x.clone()).collect(),
                    ))
                } else {
                    Err(format!("one of the params failed to convert"))
                }
            }
            _ => Err(format!("unexpected type")),
        }
    }

    fn add_deps(&self, library: &Library, out: &mut Vec<PathValue>) {
        match self {
            &Type::ConstPtr(ref t) => {
                t.add_deps(library, out);
            }
            &Type::Ptr(ref t) => {
                t.add_deps(library, out);
            }
            &Type::Path(ref p) => {
                library.add_deps_for_path(p, out);
            }
            &Type::Primitive(_) => { }
            &Type::Array(ref t, _) => {
                t.add_deps(library, out);
            }
            &Type::FuncPtr(ref opt_ret, ref args) => {
                if let Some(ref ty) = opt_ret.as_ref() {
                    ty.add_deps(library, out);
                }
                for arg in args {
                    arg.add_deps(library, out);
                }
            }
        }
    }

    fn specialize(&self, mappings: &Vec<(&String, &String)>) -> Type {
        match self {
            &Type::ConstPtr(ref t) => {
                Type::ConstPtr(Box::new(t.specialize(mappings)))
            }
            &Type::Ptr(ref t) => {
                Type::Ptr(Box::new(t.specialize(mappings)))
            }
            &Type::Path(ref p) => {
                let mut p = p.clone();

                for &(param, value) in mappings {
                    if p == *param {
                        p = value.clone();
                        break;
                    }
                }

                Type::Path(p)
            }
            &Type::Primitive(ref p) => {
                Type::Primitive(p.clone())
            }
            &Type::Array(ref t, ref sz) => {
                Type::Array(Box::new(t.specialize(mappings)), *sz)
            }
            &Type::FuncPtr(ref opt_ret, ref args) => {
                Type::FuncPtr(opt_ret.as_ref().map(|x| Box::new(x.specialize(mappings))),
                              args.iter()
                                  .map(|x| x.specialize(mappings))
                                  .collect())
            }
        }
    }

    fn generate(&self) -> String {
        match self {
            &Type::ConstPtr(ref t) => {
                format!("const {}*", t.generate())
            }
            &Type::Ptr(ref t) => {
                format!("{}*", t.generate())
            }
            &Type::Path(ref p) => {
                p.clone()
            }
            &Type::Primitive(ref p) => {
                p.clone()
            }
            &Type::Array(ref t, ref sz) => {
                format!("{}[{}]", t.generate(), sz)
            }
            &Type::FuncPtr(ref opt_ret, ref args) => {
                format!("{} (*)({})",
                        opt_ret.as_ref().map_or(format!("void"), |x| { x.generate() }),
                        args.iter()
                            .map(|x| { format!("{}", x.generate()) })
                            .collect::<Vec<_>>()
                            .join(", "))
            }
        }
    }

    fn generate_with_ident(&self, ident: &str) -> String {
        match self {
            &Type::ConstPtr(ref t) => {
                format!("const {} *{}", t.generate(), ident)
            }
            &Type::Ptr(ref t) => {
                format!("{} *{}", t.generate(), ident)
            }
            &Type::Path(ref p) => {
                format!("{} {}", p, ident)
            }
            &Type::Primitive(ref p) => {
                format!("{} {}", p, ident)
            }
            &Type::Array(ref t, ref sz) => {
                format!("{} {}[{}]", t.generate(), ident, sz)
            }
            &Type::FuncPtr(ref opt_ret, ref args) => {
                format!("{} (*{})({})",
                        opt_ret.as_ref().map_or(format!("void"), |x| { x.generate() }),
                        ident,
                        args.iter()
                            .map(|x| { format!("{}", x.generate()) })
                            .collect::<Vec<_>>()
                            .join(", "))
            }
        }
    }
}
