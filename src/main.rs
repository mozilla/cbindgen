use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::env;
use std::fmt;
use std::io;
use std::io::Write;
use std::sync::Mutex;

extern crate syn;
use syn::*;

mod rust_lib;

#[derive(Debug, Clone)]
struct ConvertedType {
    /// The type converted to C (e.g. `uint32_t`)
    prefix: String,
    /// Stuff that might need to go after the identifier (e.g. `[3]` for an array).
    postfix: String,
    /// The types this type depends upon. For example a `prefix` of
    /// Foo<Bar> would have both `Foo` and `Bar` in the `deps`.
    deps: BTreeSet<String>,
}

impl ConvertedType {
    fn prepend_prefix(&self, str: &str) -> ConvertedType {
        let mut clone = self.clone();
        clone.prefix = String::from(str);
        clone.prefix.push_str(&self.prefix);
        clone
    }

    fn append_prefix(&self, str: &str) -> ConvertedType {
        let mut clone = self.clone();
        clone.prefix.push_str(str);
        clone
    }

    fn append_postfix(&self, str: &str) -> ConvertedType {
        let mut clone = self.clone();
        clone.postfix.push_str(str);
        clone
    }

    fn format_with_ident(&self, ident: &Ident) -> String {
        format!("{} {}{}", &self.prefix, &ident.to_string(), &self.postfix)
    }
}

impl From<String> for ConvertedType {
    fn from(str: String) -> ConvertedType {
        let mut initial_set = BTreeSet::new();
        initial_set.insert(str.clone());
        ConvertedType {
            prefix: str,
            postfix: String::new(),
            deps: initial_set,
        }
    }
}

impl fmt::Display for ConvertedType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", &self.prefix, &self.postfix)
    }
}

fn has_attribute(target: MetaItem, attrs: &Vec<Attribute>) -> bool {
    return attrs
               .iter()
               .any(|ref attr| attr.style == AttrStyle::Outer && attr.value == target);
}

fn has_no_mangle(attrs: &Vec<Attribute>) -> bool {
    has_attribute(MetaItem::Word(Ident::new("no_mangle")), attrs)
}

fn wr_func_body(attrs: &Vec<Attribute>) -> String {
    let destructor_safe = attrs.iter().any(|ref attr| {
        if attr.style == AttrStyle::Outer && attr.is_sugared_doc {
            if let MetaItem::NameValue(_, Lit::Str(ref comment, _)) = attr.value {
                return comment.contains("wr-binding:destructor_safe");
            }
        }
        false
    });
    if destructor_safe {
        String::from("WR_DESTRUCTOR_SAFE_FUNC")
    } else {
        String::from("WR_FUNC")
    }
}

fn is_repr_c(attrs: &Vec<Attribute>) -> bool {
    let repr_args = vec![NestedMetaItem::MetaItem(MetaItem::Word(Ident::new("C")))];
    has_attribute(MetaItem::List(Ident::new("repr"), repr_args), attrs)
}

fn is_repr_u32(attrs: &Vec<Attribute>) -> bool {
    let repr_args = vec![NestedMetaItem::MetaItem(MetaItem::Word(Ident::new("u32")))];
    has_attribute(MetaItem::List(Ident::new("repr"), repr_args), attrs)
}

fn is_c_abi(abi: &Option<Abi>) -> bool {
    abi == &Some(Abi::Named(String::from("C")))
}

fn is_primitive(path: &str) -> bool {
    match path {
        "usize" |
        "u8"  |
        "u32" |
        "u64" |
        "i8"  |
        "i32" |
        "i64" |
        "f32" |
        "f64" |
        "c_void" => true,
        _ => false,
    }
}

fn map_path(p: &Path) -> ConvertedType {
    let l = p.segments[0].ident.to_string();
    ConvertedType::from(match l.as_ref() {
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
        _ => l,
    })
}

fn map_mut_ty(mut_ty: &MutTy) -> ConvertedType {
    let mut mapped = map_ty(&mut_ty.ty);
    if mut_ty.mutability == Mutability::Immutable {
        mapped = mapped.prepend_prefix("const ");
    }
    mapped
}

fn map_ty(ty: &Ty) -> ConvertedType {
    match ty {
        &Ty::Path(_, ref p) => map_path(p),
        &Ty::Ptr(ref mut_ty) => map_mut_ty(mut_ty).append_prefix("*"),
        &Ty::Rptr(_, ref mut_ty) => map_mut_ty(mut_ty).append_prefix("*"),
        &Ty::Array(ref p, ConstExpr::Lit(Lit::Int(sz, _))) => map_ty(&p).append_postfix(&format!("[{}]", sz)),
        &Ty::BareFn(ref b) => map_fn(b),
        _ => ConvertedType::from(format!("unknown {:?}", ty)),
    }
}

fn map_return_type(ret: &FunctionRetTy) -> ConvertedType {
    match ret {
        &FunctionRetTy::Default => ConvertedType::from("void".to_string()),
        &FunctionRetTy::Ty(ref ty) => map_ty(ty),
    }
}

fn map_fn(ty: &BareFnTy) -> ConvertedType {
    let mut deps = BTreeSet::new();

    let ret = {
        let mut mapped = map_return_type(&ty.output);
        deps.append(&mut mapped.deps);
        format!("{}", mapped)
    };
    let args = ty.inputs.iter()
                        .map(|args| {
                            let mut mapped = map_ty(&args.ty);
                            deps.append(&mut mapped.deps);
                            format!("{}", mapped)
                        })
                        .collect::<Vec<_>>()
                        .join(", ");

    ConvertedType {
        prefix: format!("{} (*", ret),
        postfix: format!(")({})", args),
        deps: deps,
    }
}

fn map_arg(f: &FnArg, dep_set: &mut BTreeSet<String>) -> String {
    match f {
        &FnArg::Captured(Pat::Ident(_, ref ident, _), ref ty) => {
            let mut converted = map_ty(ty);
            dep_set.append(&mut converted.deps);
            converted.format_with_ident(ident)
        }
        _ => "unknown".to_string(),
    }
}

fn map_field(f: &Field, dep_set: &mut BTreeSet<String>) -> String {
    let mut ret = String::from("  ");
    let mut converted = map_ty(&f.ty);
    dep_set.append(&mut converted.deps);
    ret.push_str(&converted.format_with_ident(f.ident.as_ref().expect("Struct fields must have idents")));
    ret.push_str(";\n");
    ret
}

fn fold_enum_variants(accum: (String, i32), v: &Variant) -> (String, i32) {
    // `accum` contains the combined string of converted enum variants so far, to which
    // we will append the converted version of `v`. The other thing in `accum` is the
    // value of the previous variant. If `v` has an explicit value we keep that, otherwise
    // we increment the value of the previous variant to get the new one. This is all
    // so that we properly support enums with explicitly-specified and discontinuous
    // values.
    let mut ret = accum.0;
    ret.push_str("  ");
    ret.push_str(&v.ident.to_string());
    ret.push_str(" = ");
    let new_value = match &v.discriminant {
        &None => accum.1 + 1,
        &Some(ConstExpr::Lit(Lit::Int(ref specified_value, _))) => *specified_value as i32,
        &Some(_) => {
            // we don't handle this yet, so just put in something that will fail C compilation
            writeln!(io::stderr(), "warning, unsupported enum discriminant").unwrap();
            ret.push_str("???");
            accum.1 + 1
        }
    };
    ret.push_str(&new_value.to_string());
    ret.push_str(",\n");
    (ret, new_value)
}

/// Convenience struct to group stuff associated with a converted
/// item.
struct ConvertedItem {
    /// This holds the C code that needs to go into the header file.
    c_code: String,
    /// This holds the generic parameters in use by the C code that
    /// need to be fulfilled by a type specialization. This is only used
    /// for structs.
    ty_params: Vec<String>,
    /// This holds a set of identifiers that the C code depends on.
    /// So for example if a struct A contains another struct B, the
    /// `ConvertedItem` instance for A will have the string "B" in
    /// `deps`.
    deps: BTreeSet<String>,
}

impl ConvertedItem {
    fn new(c_code: String, ty_params: Vec<String>, deps: BTreeSet<String>) -> ConvertedItem {
        ConvertedItem {
            c_code: c_code,
            ty_params: ty_params,
            deps: deps,
        }
    }
}

/// An enum for representing the two ways we use type aliases
enum TypeAlias {
    Struct(StructAlias),
    Typedef(TypedefAlias),
}

/// A `struct` alias creates a template specialized and renamed type
/// The type this aliases will be copied and have its name changed
/// and generic parameters replaced by the alias specified ones
struct StructAlias {
    aliased_ident: String,
    ty_params: Vec<String>,
}

/// A `typedef` alias creates a C typedef that references a type
/// that will be generated elsewhere
struct TypedefAlias {
    aliased: ConvertedItem,
}

impl TypeAlias {
    fn new(ident: &str, ty: &Ty) -> TypeAlias {
        // If the type is a path and is not a primitive then create a `struct`
        // type alias so that we generate a specialization for this
        if let &Ty::Path(_, ref p) = ty {
            let l = p.segments[0].ident.to_string();

            if !is_primitive(&l) {
                let ty_params = if let PathParameters::AngleBracketed(ref d) = p.segments[0].parameters {
                    d.types.iter()
                           .map(|ty| {
                               map_ty(ty).to_string()
                           })
                           .collect::<Vec<String>>()
                } else {
                    vec![]
                };

                return TypeAlias::Struct(StructAlias {
                    aliased_ident: l,
                    ty_params: ty_params,
                });
            }
        }

        // For everything else, create a typedef
        let converted_type = map_ty(ty);
        return TypeAlias::Typedef(TypedefAlias {
            aliased: ConvertedItem::new(converted_type.format_with_ident(&Ident::from(ident)),
                                        vec![],
                                        converted_type.deps),
        });
    }

    fn deps(&self, results_ref: &ConversionResults) -> BTreeSet<String> {
        match self {
            &TypeAlias::Struct(ref a) => {
                let mut deps = BTreeSet::new();

                // A type alias depends on its type parameters
                // and anything it's aliased type depends on
                for ty in &a.ty_params {
                    deps.insert(ty.clone());
                }
                if let Some(aliased) = results_ref.ds.get(&a.aliased_ident) {
                    for ty in &aliased.deps {
                        deps.insert(ty.clone());
                    }
                }

                deps
            },
            &TypeAlias::Typedef(ref a) => a.aliased.deps.clone(),
        }
    }
}

/// A structure to store all the results of converting Rust stuff to C
/// stuff, so that we can pick and choose what we output.
struct ConversionResults {
    /// This holds the C function signatures of no_mangle rust functions,
    /// in the order that they were encountered in Rust.
    funcs: Vec<ConvertedItem>,
    /// This holds the C conversions of repr(C) structs and repr(u32) enums
    /// from Rust. The 'ds' in the name stands for 'data structures'. The
    /// key of the map is the name of the type.
    ds: BTreeMap<String, ConvertedItem>,
    /// This holds type aliases we find in Rust code. This way we can do
    /// something like:
    ///  type WrFoo = Foo;  // the type alias
    ///  fn wr_func(foo: WrFoo) { ... }
    /// where `Foo` is repr(C), and have the corresponding C struct
    /// generated with the name WrFoo so that it's not polluting the
    /// global namespace and everything works ok. In the above example
    /// we would store WrFoo -> Foo in `type_map` and use it when
    /// generating the C definition of `WrFoo`.
    type_map: BTreeMap<String, TypeAlias>,
}

/// Recursive function to collect the dependencies we need. Deps are collected
/// into `dst`, using the "database" of dependencies in `results_ref`, and
/// using `deps` as the starting set of dependencies we want.
fn collect_deps(dst: &mut Vec<String>, results_ref: &ConversionResults, deps: &BTreeSet<String>) {
    for dep in deps {
        results_ref.ds.get(dep).map(|converted| collect_deps(dst, results_ref, &converted.deps));
        results_ref.type_map.get(dep).map(|alias| collect_deps(dst, results_ref, &alias.deps(results_ref)));

        if !dst.contains(&dep.to_string()) {
            dst.push(dep.to_string());
        }
    }
}

fn main() {
    let p = env::args().nth(1).unwrap();

    let results = Mutex::new(ConversionResults {
        funcs: Vec::new(),
        ds: BTreeMap::new(),
        type_map: BTreeMap::new(),
    });

    rust_lib::parse(p, &|mod_name, items| {
        for item in items {
            match item.node {
                ItemKind::Fn(ref decl,
                             ref _unsafe,
                             ref _const,
                             ref abi,
                             ref _generic,
                             ref _block) => {
                    writeln!(io::stderr(), "processing function {}::{}", mod_name, &item.ident).unwrap();
                    if has_no_mangle(&item.attrs) && is_c_abi(&abi) {
                        let mut deps = BTreeSet::new();
                        let mut return_type = map_return_type(&decl.output);
                        deps.append(&mut return_type.deps);
                        let c_code =
                            format!("WR_INLINE {}\n{}({})\n{};\n",
                                    return_type,
                                    item.ident,
                                    decl.inputs
                                        .iter()
                                        .map(|f| map_arg(f, &mut deps))
                                        .collect::<Vec<_>>()
                                        .join(",\n    "),
                                    wr_func_body(&item.attrs));
                        results.lock().unwrap().funcs.push(ConvertedItem::new(c_code, vec![], deps));
                    }
                }
                ItemKind::Struct(ref variant,
                                 ref generics) => {
                    writeln!(io::stderr(), "processing struct {}::{}", mod_name, &item.ident).unwrap();
                    if is_repr_c(&item.attrs) {
                        if let &VariantData::Struct(ref fields) = variant {
                            let mut deps = BTreeSet::new();
                            let mut c_code = String::new();
                            let ty_params = generics.ty_params
                                                    .iter()
                                                    .map(|t| t.ident.to_string())
                                                    .collect::<Vec<_>>();

                            c_code.push_str(&(
                                format!("struct {} {{\n{}\n  bool operator==(const {}& aOther) const {{\n    return {};\n  }}\n}};\n",
                                        item.ident,
                                        fields.iter()
                                              .map(|f| map_field(f, &mut deps))
                                              .collect::<String>(),
                                        item.ident,
                                        fields.iter()
                                              .map(|f| format!("{} == aOther.{}",
                                                               f.ident.as_ref().unwrap(),
                                                               f.ident.as_ref().unwrap()))
                                              .collect::<Vec<_>>()
                                              .join(" &&\n      "))));
                            results.lock().unwrap().ds.insert(
                                item.ident.to_string(),
                                ConvertedItem::new(c_code, ty_params, deps));
                        }
                    }
                }
                ItemKind::Enum(ref variants, ref _generics) => {
                    writeln!(io::stderr(), "processing enum {}::{}", mod_name, &item.ident).unwrap();
                    if is_repr_u32(&item.attrs) {
                        let c_code = format!("enum class {}: uint32_t {{\n{}\n  Sentinel /* this must be last for serialization purposes. */\n}};\n",
                                             item.ident,
                                             variants.iter()
                                                     .fold((String::new(), -1), fold_enum_variants)
                                                     .0);
                        results.lock().unwrap().ds.insert(
                            item.ident.to_string(),
                            ConvertedItem::new(c_code, vec![], BTreeSet::new()));
                    }
                }
                ItemKind::Ty(ref ty, ref _generics) => {
                    let alias_name = item.ident.to_string();
                    let alias = TypeAlias::new(&alias_name, ty);

                    results.lock().unwrap().type_map.insert(alias_name, alias);
                }
                _ => {}
            }
        }
    });

    // Collect all the recursive type dependencies using the functions
    // as the roots. (i.e. all the structs/enums referred to by the function
    // return types and arguments, plus all the structs/enums referred to
    // by *those* and so on). We just skip any types that we don't have
    // instead of returning an error. This way we don't have to add special
    // handling for things like void and int32_t and whatnot.
    // Collect into a Vec to maintain dependency order - things that are depended
    // are always before the things that depend on them.
    let mut all_func_deps = Vec::new();
    let results_ref = &results.lock().unwrap();
    for converted in &results_ref.funcs {
        collect_deps(&mut all_func_deps, results_ref, &converted.deps);
    }

    // Showtime!
    println!(r###"/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
"###);
    for dep in all_func_deps {
        // Check for `dep` in the ds map. If we don't find it, look up `dep`
        // in the type map, and if we find an alias, look that up in the ds
        // map instead. If we find the alias, we need to print it out, but
        // replace the alias (`mapped`) with `dep`, because the function
        // signature uses `dep`.
        if let Some(converted) = results_ref.ds.get(&dep) {
            if !converted.ty_params.is_empty() {
                continue;
            }
            println!("/* THIS FILE IS GENERATED! DO NOT MODIFY MANUALLY! See https://github.com/jrmuizel/wr-binding! */");
            println!("{}", converted.c_code);
        } else if let Some(alias) = results_ref.type_map.get(&dep) {
            match alias {
                &TypeAlias::Struct(ref alias) => {
                    // Create a specialized version of the struct or enum
                    // with type parameters resolved and a new name
                    if let Some(aliased) = results_ref.ds.get(&alias.aliased_ident) {
                        let replaced_name = aliased.c_code.replace(&alias.aliased_ident,
                                                                   &dep);
                        let code = alias.ty_params.iter()
                                                  .zip(aliased.ty_params.iter())
                                                  .fold(replaced_name,
                                                        |code, (value, param)| {
                                                            code.replace(param, value)
                                                        });
                        println!("/* THIS FILE IS GENERATED! DO NOT MODIFY MANUALLY! See https://github.com/jrmuizel/wr-binding! */");
                        println!("{}", code);
                    }
                }
                &TypeAlias::Typedef(ref alias) => {
                    println!("/* THIS FILE IS GENERATED! DO NOT MODIFY MANUALLY! See https://github.com/jrmuizel/wr-binding! */");
                    println!("typedef {};\n", alias.aliased.c_code);
                }
            }
        }
    }
    for converted in &results_ref.funcs {
        println!("/* THIS FILE IS GENERATED! DO NOT MODIFY MANUALLY! See https://github.com/jrmuizel/wr-binding! */");
        println!("{}", converted.c_code);
    }
}
