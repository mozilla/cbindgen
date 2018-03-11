/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;

use bindgen::config::{Config};
use bindgen::ir::{Function, Type};
use bindgen::writer::{ListType, SourceWriter};

// This code is for translating Rust types into C declarations.
// See Section 6.7, Declarations, in the C standard for background.
// http://www.open-std.org/jtc1/sc22/wg14/www/docs/n1570.pdf

enum CDeclarator {
    Ptr(bool),
    Array(String),
    Func(Vec<(Option<String>, CDecl)>, bool),
}

impl CDeclarator {
    fn is_ptr(&self) -> bool {
        match self {
            &CDeclarator::Ptr(..) => true,
            &CDeclarator::Func(..) => true,
            _ => false,
        }
    }
}

struct CDecl {
    type_qualifers: String,
    type_name: String,
    type_generic_args: Vec<Type>,
    declarators: Vec<CDeclarator>,
}

impl CDecl {
    fn new() -> CDecl {
        CDecl {
            type_qualifers: String::new(),
            type_name: String::new(),
            type_generic_args: Vec::new(),
            declarators: Vec::new(),
        }
    }

    fn from_type(t: &Type) -> CDecl {
        let mut cdecl = CDecl::new();
        cdecl.build_type(t, false);
        cdecl
    }
    fn from_func(f: &Function, layout_vertical: bool) -> CDecl {
        let mut cdecl = CDecl::new();
        cdecl.build_func(f, layout_vertical);
        cdecl
    }

    fn build_func(&mut self, f: &Function, layout_vertical: bool) {
        let args = f.args
            .iter()
            .map(|&(ref arg_name, ref arg_ty)| (Some(arg_name.clone()), CDecl::from_type(arg_ty)))
            .collect();
        self.declarators
            .push(CDeclarator::Func(args, layout_vertical));
        self.build_type(&f.ret, false);
    }

    fn build_type(&mut self, t: &Type, is_const: bool) {
        match t {
            &Type::Path(ref path) => {
                if is_const {
                    assert!(self.type_qualifers.len() == 0, "error generating cdecl for {:?}", t);
                    self.type_qualifers = "const".to_owned();
                }

                assert!(self.type_name.len() == 0, "error generating cdecl for {:?}", t);
                self.type_name = path.name.clone();
                assert!(self.type_generic_args.len() == 0, "error generating cdecl for {:?}", t);
                self.type_generic_args = path.generics.clone();
            }
            &Type::Primitive(ref p) => {
                if is_const {
                    assert!(self.type_qualifers.len() == 0, "error generating cdecl for {:?}", t);
                    self.type_qualifers = "const".to_owned();
                }

                assert!(self.type_name.len() == 0, "error generating cdecl for {:?}", t);
                self.type_name = p.to_string();
            }

            &Type::ConstPtr(ref t) => {
                self.declarators.push(CDeclarator::Ptr(is_const));
                self.build_type(t, true);
            }
            &Type::Ptr(ref t) => {
                self.declarators.push(CDeclarator::Ptr(is_const));
                self.build_type(t, false);
            }
            &Type::Array(ref t, ref constant) => {
                self.declarators.push(CDeclarator::Array(constant.clone()));
                self.build_type(t, false);
            }
            &Type::FuncPtr(ref ret, ref args) => {
                let args = args.iter().map(|x| (None, CDecl::from_type(x))).collect();
                self.declarators.push(CDeclarator::Ptr(false));
                self.declarators.push(CDeclarator::Func(args, false));
                self.build_type(ret, false);
            }
        }
    }

    fn write<F: Write>(
        &self,
        out: &mut SourceWriter<F>,
        ident: Option<&str>,
        void_prototype: bool,
        config: &Config
    ) {
        // Write the type-specifier and type-qualifier first
        if self.type_qualifers.len() != 0 {
            write!(out, "{} ", self.type_qualifers);
        }

        if config.export.typedef_struct(&self.type_name) {
            write!(out, "{}", self.type_name);
        } else {
            write!(out, "struct {}", self.type_name);
        }

        if !self.type_generic_args.is_empty() {
            out.write("<");
            out.write_horizontal_source_list(&self.type_generic_args, ListType::Join(", "));
            out.write(">");
        }

        // When we have an identifier, put a space between the type and the declarators
        if ident.is_some() {
            out.write(" ");
        }

        // Write the left part of declarators before the identifier
        let mut iter_rev = self.declarators.iter().rev().peekable();

        while let Some(declarator) = iter_rev.next() {
            let next_is_pointer = iter_rev.peek().map_or(false, |x| x.is_ptr());

            match declarator {
                &CDeclarator::Ptr(ref is_const) => if *is_const {
                    out.write("*const ");
                } else {
                    out.write("*");
                },
                &CDeclarator::Array(..) => if next_is_pointer {
                    out.write("(");
                },
                &CDeclarator::Func(..) => if next_is_pointer {
                    out.write("(");
                },
            }
        }

        // Write the identifier
        if let Some(ident) = ident {
            write!(out, "{}", ident);
        }

        // Write the right part of declarators after the identifier
        let mut iter = self.declarators.iter();
        let mut last_was_pointer = false;

        while let Some(declarator) = iter.next() {
            match declarator {
                &CDeclarator::Ptr(..) => {
                    last_was_pointer = true;
                }
                &CDeclarator::Array(ref constant) => {
                    if last_was_pointer {
                        out.write(")");
                    }
                    write!(out, "[{}]", constant);

                    last_was_pointer = false;
                }
                &CDeclarator::Func(ref args, layout_vertical) => {
                    if last_was_pointer {
                        out.write(")");
                    }

                    out.write("(");
                    if args.is_empty() && void_prototype {
                        out.write("void");
                    }
                    if layout_vertical {
                        let align_length = out.line_length_for_align();
                        out.push_set_spaces(align_length);
                        for (i, &(ref arg_ident, ref arg_ty)) in args.iter().enumerate() {
                            if i != 0 {
                                out.write(",");
                                out.new_line();
                            }

                            // Convert &Option<String> to Option<&str>
                            let arg_ident = arg_ident.as_ref().map(|x| x.as_ref());

                            arg_ty.write(out, arg_ident, void_prototype, config);
                        }
                        out.pop_tab();
                    } else {
                        for (i, &(ref arg_ident, ref arg_ty)) in args.iter().enumerate() {
                            if i != 0 {
                                out.write(", ");
                            }

                            // Convert &Option<String> to Option<&str>
                            let arg_ident = arg_ident.as_ref().map(|x| x.as_ref());

                            arg_ty.write(out, arg_ident, void_prototype, config);
                        }
                    }
                    out.write(")");

                    last_was_pointer = true;
                }
            }
        }
    }
}

pub fn write_func<F: Write>(
    out: &mut SourceWriter<F>,
    f: &Function,
    layout_vertical: bool,
    void_prototype: bool,
    config: &Config
) {
    &CDecl::from_func(f, layout_vertical).write(out, Some(&f.name), void_prototype, &config);
}

pub fn write_field<F: Write>(out: &mut SourceWriter<F>, t: &Type, ident: &str, config: &Config) {
    &CDecl::from_type(t).write(out, Some(ident), false, &config);
}

pub fn write_type<F: Write>(out: &mut SourceWriter<F>, t: &Type, config: &Config) {
    &CDecl::from_type(t).write(out, None, false, &config);
}
