use std::io::Write;

use bindgen::items::*;
use bindgen::writer::*;

// This code is for translating Rust types into C declarations.
// See Section 6.7, Declarations, in the C standard for background.
// http://www.open-std.org/jtc1/sc22/wg14/www/docs/n1570.pdf
// This code can be done in a single recursive function, but it's
// not pretty and not worth it.

struct CDecl {
    type_qualifers: String,
    type_name: String,
    declarators: Vec<CDeclarator>
}

enum CDeclarator {
    Ptr(bool),
    Array(u64),
    Func(Vec<(Option<String>, CDecl)>),
}

impl CDecl {
    fn new() -> CDecl {
        CDecl {
            type_qualifers: String::new(),
            type_name: String::new(),
            declarators: Vec::new(),
        }
    }

    fn from_type(t: &Type) -> CDecl {
        let mut cdecl = CDecl::new();
        cdecl.build_type(t, false);
        cdecl
    }
    fn from_func(f: &Function) -> CDecl {
        let mut cdecl = CDecl::new();
        cdecl.build_func(f);
        cdecl
    }

    fn build_func(&mut self, f: &Function) {
        let args = f.args.iter().map(|&(ref arg_name, ref arg_ty)| (Some(arg_name.clone()), CDecl::from_type(arg_ty))).collect();
        self.declarators.push(CDeclarator::Func(args));
        self.build_type(&f.ret, false);
    }

    fn build_type(&mut self, t: &Type, is_const: bool) {
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
                self.build_type(t, true);
            }
            &Type::Ptr(ref t) => {
                self.declarators.push(CDeclarator::Ptr(is_const));
                self.build_type(t, false);
            }
            &Type::Array(ref t, sz) => {
                self.declarators.push(CDeclarator::Array(sz));
                self.build_type(t, false);
            }
            &Type::FuncPtr(ref ret, ref args) => {
                let args = args.iter().map(|x| (None, CDecl::from_type(x))).collect();
                self.declarators.push(CDeclarator::Ptr(false));
                self.declarators.push(CDeclarator::Func(args));
                self.build_type(ret, false);
            }
        }
    }

    fn to_string(&self, ident: Option<&str>) -> String {
        // Build the left side (the type-specifier and type-qualifier),
        // and then build the right side (the declarators), and then
        // merge the result.

        let type_and_qualifier = if self.type_qualifers.len() != 0 {
            format!("{} {}",
                    self.type_qualifers,
                    self.type_name)
        } else {
            format!("{}",
                    self.type_name)
        };

        let mut left_declarators = String::new();
        let mut right_declarators = String::new();
        let mut last_was_pointer = false;
        for declarator in &self.declarators {
            match declarator {
                &CDeclarator::Ptr(ref is_const) => {
                    if *is_const {
                        left_declarators.insert_str(0, "*const ");
                    } else {
                        left_declarators.insert_str(0, "*");
                    }

                    last_was_pointer = true;
                },
                &CDeclarator::Array(sz) => {
                    if last_was_pointer {
                        left_declarators.insert_str(0, "(");
                        right_declarators.push_str(")");
                    }
                    right_declarators.push_str(&format!("[{}]", sz));

                    last_was_pointer = false;
                },
                &CDeclarator::Func(ref args) => {
                    if last_was_pointer {
                        left_declarators.insert_str(0, "(");
                        right_declarators.push_str(")");
                    }

                    right_declarators.push_str("(");
                    for (i, &(ref arg_ident, ref arg_ty)) in args.iter().enumerate() {
                        if i != 0 {
                            right_declarators.push_str(", ");
                        }

                        // This is gross, but needed to convert &Option<String> to Option<&str>
                        let arg_ident = arg_ident.as_ref().map(|x| x.as_ref());

                        right_declarators.push_str(&arg_ty.to_string(arg_ident));
                    }
                    right_declarators.push_str(")");

                    last_was_pointer = true;
                },
            }
        }

        match ident {
            Some(ident) => {
                format!("{} {}{}{}",
                        type_and_qualifier,
                        left_declarators,
                        ident,
                        right_declarators)
            }
            None => {
                format!("{}{}{}",
                        type_and_qualifier,
                        left_declarators,
                        right_declarators)
            }
        }
    }
}

pub fn write_func<F: Write>(out: &mut SourceWriter<F>, f: &Function) {
    out.write(&CDecl::from_func(f).to_string(Some(&f.name)));
}
pub fn write_type<F: Write>(out: &mut SourceWriter<F>, t: &Type, ident: &str) {
    out.write(&CDecl::from_type(t).to_string(Some(ident)));
}
