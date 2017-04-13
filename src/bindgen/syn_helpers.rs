use syn::*;

use bindgen::items::*;
use bindgen::library::*;

pub trait SynItemHelpers {
    fn has_attr(&self, target: MetaItem) -> bool;
    fn get_doc_attr(&self) -> String;

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
}
impl SynItemHelpers for Item {
    fn has_attr(&self, target: MetaItem) -> bool {
        return self.attrs
                   .iter()
                   .any(|ref attr| attr.style == AttrStyle::Outer && attr.value == target);
    }
    fn get_doc_attr(&self) -> String {
        let mut doc = String::new();
        for attr in &self.attrs {
            if attr.style == AttrStyle::Outer &&
               attr.is_sugared_doc {
                if let MetaItem::NameValue(_, Lit::Str(ref comment, _)) = attr.value {
                    doc.push_str(&comment);
                    doc.push('\n');
                }
            }
        }
        doc
    }
}

pub trait SynAbiHelpers {
    fn is_c(&self) -> bool;
}
impl SynAbiHelpers for Option<Abi> {
    fn is_c(&self) -> bool {
        self == &Some(Abi::Named(String::from("C")))
    }
}

pub trait SynFnRetTyHelpers {
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

pub trait SynFnArgHelpers {
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

pub trait SynFieldHelpers {
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
pub fn map_path_name_to_primitive(path_name: &str) -> String {
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
pub fn convert_path_name_to_primitive(path_name: &str) -> Option<String> {
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
pub fn path_name_is_primitive(path_name: &str) -> bool {
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

pub trait SynPathHelpers {
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
