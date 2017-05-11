use bindgen::items::*;

// This code is for translating Rust types into C declarations.
// See Section 6.7, Declarations, in the C standard for background.
// http://www.open-std.org/jtc1/sc22/wg14/www/docs/n1570.pdf
// This code can be done in a single recursive function, but it's
// not pretty and not worth it.

struct CDecl {
    type_qualifers: String,
    type_name: String,
    identifier: Option<String>,
    declarators: Vec<CDeclarator>
}
enum CDeclarator {
    Ptr(bool),
    Array(u64),
    Func(Vec<CDecl>),
}

impl CDecl {
    fn new() -> CDecl {
        CDecl {
            type_qualifers: String::new(),
            type_name: String::new(),
            identifier: None,
            declarators: Vec::new(),
        }
    }
    fn new_with_ident(ident: String) -> CDecl {
        CDecl {
            type_qualifers: String::new(),
            type_name: String::new(),
            identifier: Some(ident),
            declarators: Vec::new(),
        }
    }

    fn from_type(t: &Type) -> CDecl {
        let mut cdecl = CDecl::new();
        cdecl.build(t, false);
        cdecl
    }
    fn from_type_with_ident(ident: String, t: &Type) -> CDecl {
        let mut cdecl = CDecl::new_with_ident(ident);
        cdecl.build(t, false);
        cdecl
    }

    fn build(&mut self, t: &Type, is_const: bool) {
        match t {
            &Type::Path(ref p) => {
                if is_const {
                    assert!(self.type_qualifers.len() == 0);
                    self.type_qualifers = "const".to_owned();
                }

                assert!(self.type_name.len() == 0);
                self.type_name = p.clone();
            }
            &Type::Primitive(ref p) => {
                if is_const {
                    assert!(self.type_qualifers.len() == 0);
                    self.type_qualifers = "const".to_owned();
                }

                assert!(self.type_name.len() == 0);
                self.type_name = p.to_string();
            }

            &Type::ConstPtr(ref t)  => {
                self.declarators.push(CDeclarator::Ptr(is_const));
                self.build(t, true);
            }
            &Type::Ptr(ref t) => {
                self.declarators.push(CDeclarator::Ptr(is_const));
                self.build(t, false);
            }
            &Type::Array(ref t, sz) => {
                self.declarators.push(CDeclarator::Array(sz));
                self.build(t, false);
            }
            &Type::FuncPtr(ref ret, ref args) => {
                let args = args.iter().map(|x| CDecl::from_type(x)).collect();
                self.declarators.push(CDeclarator::Ptr(false));
                self.declarators.push(CDeclarator::Func(args));
                if let &Some(ref ret) = ret {
                    self.build(ret, false);
                } else {
                    self.build(&Type::Primitive(PrimitiveType::Void), false);
                }
            }
        }
    }

    fn to_string(&self) -> String {
        // Build the left side (the type-specifier and type-qualifier),
        // and then build the right side (the declarators), and then
        // merge the result.

        let left = if self.type_qualifers.len() != 0 {
            format!("{} {}",
                    self.type_qualifers,
                    self.type_name)
        } else {
            format!("{}",
                    self.type_name)
        };

        let mut right = if let Some(ref ident) = self.identifier {
            ident.to_owned()
        } else {
            String::new()
        };

        let mut last_was_pointer = false;
        for declarator in &self.declarators {
            match declarator {
                &CDeclarator::Ptr(ref is_const) => {
                    if *is_const {
                        right.insert_str(0, "*const ");
                    } else {
                        right.insert_str(0, "*");
                    }

                    last_was_pointer = true;
                },
                &CDeclarator::Array(sz) => {
                    if last_was_pointer {
                        right.insert_str(0, "(");
                        right.push_str(")");
                    }
                    right.push_str(&format!("[{}]", sz));

                    last_was_pointer = false;
                },
                &CDeclarator::Func(ref args) => {
                    if last_was_pointer {
                        right.insert_str(0, "(");
                        right.push_str(")");
                    }

                    let args = args.iter()
                                   .map(|x| x.to_string())
                                   .collect::<Vec<_>>()
                                   .join(", ");
                    right.push_str(&format!("({})", args));

                    last_was_pointer = true;
                },
            }
        }

        if right.len() == 0 {
            return left;
        } else if !self.identifier.is_some() {
            return format!("{}{}", left, right);
        } else {
            return format!("{} {}", left, right);
        }
    }
}

pub fn to_cdecl(t: &Type) -> String {
    CDecl::from_type(t).to_string()
}
pub fn to_cdecl_with_ident(ident: String, t: &Type) -> String {
    CDecl::from_type_with_ident(ident, t).to_string()
}
