use std::env;
use std::fmt;

extern crate syn;
use syn::*;

mod rust_lib;

#[derive(Debug)]
struct ConvertedType {
    /// The type converted to C (e.g. `uint32_t`)
    prefix: String,
    /// Stuff that might need to go after the identifier (e.g. `[3]` for an array).
    postfix: String,
}

impl ConvertedType {
    fn new<T: Into<String>>(pre: T, post: String) -> Self {
        ConvertedType {
            prefix: pre.into(),
            postfix: post
        }
    }
}

impl Into<String> for ConvertedType {
    fn into(self) -> String {
        let mut str = String::from(self.prefix);
        str.push_str(&self.postfix);
        str
    }
}

impl From<String> for ConvertedType {
    fn from(str: String) -> ConvertedType {
        ConvertedType {
            prefix: str,
            postfix: String::new()
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

fn is_c_abi(abi: &Option<Abi>) -> bool {
    abi == &Some(Abi::Named(String::from("C")))
}

fn map_path(p: &Path) -> String {
    let l = p.segments[0].ident.to_string();
    let mut c = match l.as_ref() {
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
    };
    if let PathParameters::AngleBracketed(ref d) = p.segments[0].parameters {
        let template_args = d.types
            .iter()
            .map(|ty| map_ty(ty).into())
            .collect::<Vec<String>>()
            .join(", ");
        if !template_args.is_empty() {
            c.push('<');
            c.push_str(&template_args);
            c.push('>');
        }
    }
    c
}

fn map_mut_ty(mut_ty: &MutTy) -> ConvertedType {
    map_ty(&mut_ty.ty)
}

fn map_ty(ty: &Ty) -> ConvertedType {
    match ty {
        &Ty::Path(_, ref p) => ConvertedType::from(map_path(p)),
        &Ty::Ptr(ref p) => ConvertedType::from(format!("{}*", map_ty(&p.ty))),
        &Ty::Rptr(_, ref mut_ty) => ConvertedType::from(format!("{}*", map_mut_ty(mut_ty))),
        &Ty::Array(ref p, ConstExpr::Lit(Lit::Int(sz, _))) => ConvertedType::new(map_ty(&p), format!("[{}]", sz)),
        _ => ConvertedType::from(format!("unknown {:?}", ty)),
    }
}

fn map_return_type(ret: &FunctionRetTy) -> String {
    match ret {
        &FunctionRetTy::Default => "void".to_string(),
        &FunctionRetTy::Ty(ref ty) => map_ty(ty).into(),
    }
}

fn map_pat(pat: &Pat) -> String {
    match pat {
        &Pat::Ident(_, ref ident, _) => ident.to_string(),
        _ => format!("unknown {:?}", pat),
    }

}

fn map_arg(f: &FnArg) -> String {
    match f {
        &FnArg::Captured(ref pat, ref ty) => format!("{} {}", map_ty(ty), map_pat(pat)),
        _ => "unknown".to_string(),
    }
}

fn map_field(f: &Field) -> String {
    let mut ret = String::from("  ");
    let converted = map_ty(&f.ty);
    ret.push_str(&converted.prefix);
    ret.push(' ');
    ret.push_str(&f.ident.as_ref().expect("Struct fields must have idents").to_string());
    ret.push_str(&converted.postfix);
    ret.push_str(";\n");
    ret
}

fn map_generic_param(t: &TyParam) -> String {
    let mut ret = String::from("typename ");
    ret.push_str(&t.ident.to_string());
    ret
}

fn main() {
    let p = env::args().nth(1).unwrap();

    rust_lib::parse(p, &|_, items| {
        for item in items {
            match item.node {
                ItemKind::Fn(ref decl,
                             ref _unsafe,
                             ref _const,
                             ref abi,
                             ref _generic,
                             ref _block) => {
                    if has_no_mangle(&item.attrs) && is_c_abi(&abi) {
                        println!("WR_INLINE {}\n{}({})\n{};\n",
                                 map_return_type(&decl.output),
                                 item.ident,
                                 decl.inputs
                                     .iter()
                                     .map(map_arg)
                                     .collect::<Vec<_>>()
                                     .join(", "),
                                 wr_func_body(&item.attrs));
                    }
                }
                ItemKind::Struct(ref variant,
                                 ref generics) => {
                    if is_repr_c(&item.attrs) {
                        if !generics.ty_params.is_empty() {
                            println!("template<{}>",
                                     generics.ty_params
                                         .iter()
                                         .map(map_generic_param)
                                         .collect::<Vec<_>>()
                                         .join(", "));
                        }
                        if let &VariantData::Struct(ref fields) = variant {
                            println!("struct {} {{\n{}}};\n",
                                     item.ident,
                                     fields
                                         .iter()
                                         .map(map_field)
                                         .collect::<String>());
                        }
                    }
                }
                _ => {}
            }
        }
    });
}
