use std::env;
use std::fs::File;
use std::io::Read;
extern crate syn;
use syn::*;

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

fn is_c_abi(abi: &Option<Abi>) -> bool {
    abi == &Some(Abi::Named(String::from("C")))
}

fn map_path(p: &Path) -> String {
    let l = p.segments[0].ident.to_string();
    match l.as_ref() {
        "usize" => "size_t".to_string(),
        "u8" => "uint8_t".to_string(),
        "u32" => "uint32_t".to_string(),
        "f32" => "float".to_string(),
        "c_void" => "void".to_string(),
        _ => l,
    }
}

fn map_mut_ty(mut_ty: &MutTy) -> String {
    map_ty(&mut_ty.ty)
}

fn map_ty(ty: &Ty) -> String {
    match ty {
        &Ty::Path(_, ref p) => map_path(p),
        &Ty::Ptr(ref p) => format!("{}*", map_ty(&p.ty)),
        &Ty::Rptr(_, ref mut_ty) => format!("{}*", map_mut_ty(mut_ty)),
        _ => format!("unknown {:?}", ty),
    }

}

fn map_return_type(ret: &FunctionRetTy) -> String {
    match ret {
        &FunctionRetTy::Default => "void".to_string(),
        &FunctionRetTy::Ty(ref ty) => map_ty(ty),
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

fn main() {
    let p = env::args().nth(1).unwrap();
    let mut s = String::new();
    let mut f = File::open(p).unwrap();
    f.read_to_string(&mut s).unwrap();
    let krate = syn::parse_crate(&s).unwrap();
    for item in krate.items {
        match item.node {
            ItemKind::Fn(decl, _unsafe, _const, abi, _generic, _block) => {
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
            _ => {}
        }
    }

}
