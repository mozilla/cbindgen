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

    fn add_dep(&self, dep: String) -> ConvertedType {
        let mut clone = self.clone();
        clone.deps.insert(dep);
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
    if has_attribute(MetaItem::Word(Ident::new("destructor_safe")), attrs) {
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

fn map_path(p: &Path) -> ConvertedType {
    let l = p.segments[0].ident.to_string();
    let mut c = ConvertedType::from(match l.as_ref() {
        "usize" => "size_t".to_string(),
        "u8" => "uint8_t".to_string(),
        "u32" => "uint32_t".to_string(),
        "u64" => "uint64_t".to_string(),
        "i8" => "int8_t".to_string(),
        "i32" => "int32_t".to_string(),
        "i64" => "int64_t".to_string(),
        "f32" => "float".to_string(),
        "c_void" => "void".to_string(),
        _ => l,
    });
    if let PathParameters::AngleBracketed(ref d) = p.segments[0].parameters {
        let template_args = d.types
            .iter()
            .map(|ty| {
                let conv = map_ty(ty);
                c = c.add_dep(conv.to_string());
                conv.to_string()
            })
            .collect::<Vec<String>>()
            .join(", ");
        if !template_args.is_empty() {
            c = c.append_prefix(&format!("<{}>", &template_args));
        }
    }
    c
}

fn map_mut_ty(mut_ty: &MutTy) -> ConvertedType {
    map_ty(&mut_ty.ty)
}

fn map_ty(ty: &Ty) -> ConvertedType {
    match ty {
        &Ty::Path(_, ref p) => map_path(p),
        &Ty::Ptr(ref p) => map_ty(&p.ty).append_prefix("*"),
        &Ty::Rptr(_, ref mut_ty) => map_mut_ty(mut_ty).append_prefix("*"),
        &Ty::Array(ref p, ConstExpr::Lit(Lit::Int(sz, _))) => map_ty(&p).append_postfix(&format!("[{}]", sz)),
        _ => ConvertedType::from(format!("unknown {:?}", ty)),
    }
}

fn map_return_type(ret: &FunctionRetTy) -> ConvertedType {
    match ret {
        &FunctionRetTy::Default => ConvertedType::from("void".to_string()),
        &FunctionRetTy::Ty(ref ty) => map_ty(ty),
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

fn map_generic_param(t: &TyParam) -> String {
    let mut ret = String::from("typename ");
    ret.push_str(&t.ident.to_string());
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

struct ConvertedItem {
    /// This holds the C code that needs to go into the header file.
    c_code: String,
    /// This holds a set of identifiers that the C code depends on.
    /// So for example if a struct A contains another struct B, the
    /// `ConvertedItem` instance for A will have the string "B" in
    /// `deps`.
    deps: BTreeSet<String>,
}

impl ConvertedItem {
    fn new(c_code: String, deps: BTreeSet<String>) -> ConvertedItem {
        ConvertedItem {
            c_code: c_code,
            deps: deps,
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
}

fn collect_deps(dst: &mut BTreeSet<String>, results_ref: &ConversionResults, deps: &BTreeSet<String>) {
    for dep in deps {
        dst.insert(dep.to_string());
        results_ref.ds.get(dep).map(|converted| collect_deps(dst, results_ref, &converted.deps));
    }
}

fn main() {
    let p = env::args().nth(1).unwrap();

    let results = Mutex::new(ConversionResults {
        funcs: Vec::new(),
        ds: BTreeMap::new(),
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
                                        .join(", "),
                                    wr_func_body(&item.attrs));
                        results.lock().unwrap().funcs.push(ConvertedItem::new(c_code, deps));
                    }
                }
                ItemKind::Struct(ref variant,
                                 ref generics) => {
                    writeln!(io::stderr(), "processing struct {}::{}", mod_name, &item.ident).unwrap();
                    if is_repr_c(&item.attrs) {
                        if let &VariantData::Struct(ref fields) = variant {
                            let mut deps = BTreeSet::new();
                            let mut c_code = String::new();
                            if !generics.ty_params.is_empty() {
                                c_code.push_str(&(
                                    format!("template<{}>\n",
                                            generics.ty_params
                                                    .iter()
                                                    .map(map_generic_param)
                                                    .collect::<Vec<_>>()
                                                    .join(", "))));
                            }
                            c_code.push_str(&(
                                format!("struct {} {{\n{}}};\n",
                                        item.ident,
                                        fields.iter()
                                              .map(|f| map_field(f, &mut deps))
                                              .collect::<String>())));
                            results.lock().unwrap().ds.insert(
                                item.ident.to_string(),
                                ConvertedItem::new(c_code, deps));
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
                            ConvertedItem::new(c_code, BTreeSet::new()));
                    }
                }
                _ => {}
            }
        }
    });

    let mut all_func_deps = BTreeSet::new();
    let results_ref = &results.lock().unwrap();
    for converted in &results_ref.funcs {
        collect_deps(&mut all_func_deps, results_ref, &converted.deps);
    }
    for dep in all_func_deps {
        results_ref.ds.get(&dep).map(|converted| println!("{}", converted.c_code));
    }
    for converted in &results_ref.funcs {
        println!("{}", converted.c_code);
    }
}
