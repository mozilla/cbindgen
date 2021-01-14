/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;

use crate::bindgen::declarationtyperesolver::DeclarationType;
use crate::bindgen::ir::{ArrayLength, Function, Type};
use crate::bindgen::writer::{ListType, SourceWriter};
use crate::bindgen::{Config, Language};

// This code is for translating Rust types into C declarations.
// See Section 6.7, Declarations, in the C standard for background.
// http://www.open-std.org/jtc1/sc22/wg14/www/docs/n1570.pdf

enum CDeclarator {
    Ptr {
        is_const: bool,
        is_nullable: bool,
        is_ref: bool,
    },
    Array(String),
    Func(Vec<(Option<String>, CDecl)>, bool),
}

impl CDeclarator {
    fn is_ptr(&self) -> bool {
        match self {
            CDeclarator::Ptr { .. } | CDeclarator::Func(..) => true,
            _ => false,
        }
    }
}

struct CDecl {
    type_qualifers: String,
    type_name: String,
    type_generic_args: Vec<Type>,
    declarators: Vec<CDeclarator>,
    type_ctype: Option<DeclarationType>,
}

impl CDecl {
    fn new() -> CDecl {
        CDecl {
            type_qualifers: String::new(),
            type_name: String::new(),
            type_generic_args: Vec::new(),
            declarators: Vec::new(),
            type_ctype: None,
        }
    }

    fn from_type(t: &Type, config: &Config) -> CDecl {
        let mut cdecl = CDecl::new();
        cdecl.build_type(t, false, config);
        cdecl
    }

    fn from_func_arg(t: &Type, array_length: Option<&str>, config: &Config) -> CDecl {
        let mut cdecl = CDecl::new();
        let length = match array_length {
            Some(l) => l,
            None => return CDecl::from_type(t, config),
        };
        let (ty, is_const) = match t {
            Type::Ptr { ty, is_const, .. } => (ty, is_const),
            _ => unreachable!(
                "Should never have an array length for a non pointer type {:?}",
                t
            ),
        };
        let ptr_as_array = Type::Array(ty.clone(), ArrayLength::Value(length.to_string()));
        cdecl.build_type(&ptr_as_array, *is_const, config);
        cdecl
    }

    fn from_func(f: &Function, layout_vertical: bool, config: &Config) -> CDecl {
        let mut cdecl = CDecl::new();
        cdecl.build_func(f, layout_vertical, config);
        cdecl
    }

    fn build_func(&mut self, f: &Function, layout_vertical: bool, config: &Config) {
        let args = f
            .args
            .iter()
            .map(|arg| {
                (
                    arg.name.clone(),
                    CDecl::from_func_arg(&arg.ty, arg.array_length.as_deref(), config),
                )
            })
            .collect();
        self.declarators
            .push(CDeclarator::Func(args, layout_vertical));
        self.build_type(&f.ret, false, config);
    }

    fn build_type(&mut self, t: &Type, is_const: bool, config: &Config) {
        match t {
            Type::Path(ref generic) => {
                if is_const {
                    assert!(
                        self.type_qualifers.is_empty(),
                        "error generating cdecl for {:?}",
                        t
                    );
                    self.type_qualifers = "const".to_owned();
                }

                assert!(
                    self.type_name.is_empty(),
                    "error generating cdecl for {:?}",
                    t
                );
                self.type_name = generic.export_name().to_owned();
                assert!(
                    self.type_generic_args.is_empty(),
                    "error generating cdecl for {:?}",
                    t
                );
                self.type_generic_args = generic.generics().to_owned();
                self.type_ctype = generic.ctype().cloned();
            }
            Type::Primitive(ref p) => {
                if is_const {
                    assert!(
                        self.type_qualifers.is_empty(),
                        "error generating cdecl for {:?}",
                        t
                    );
                    self.type_qualifers = "const".to_owned();
                }

                assert!(
                    self.type_name.is_empty(),
                    "error generating cdecl for {:?}",
                    t
                );
                self.type_name = p.to_repr_c(config).to_string();
            }
            Type::Ptr {
                ref ty,
                is_nullable,
                is_const: ptr_is_const,
                is_ref,
            } => {
                self.declarators.push(CDeclarator::Ptr {
                    is_const,
                    is_nullable: *is_nullable,
                    is_ref: *is_ref,
                });
                self.build_type(ty, *ptr_is_const, config);
            }
            Type::Array(ref t, ref constant) => {
                let len = constant.as_str().to_owned();
                self.declarators.push(CDeclarator::Array(len));
                self.build_type(t, is_const, config);
            }
            Type::FuncPtr {
                ref ret,
                ref args,
                is_nullable: _,
            } => {
                let args = args
                    .iter()
                    .map(|(ref name, ref ty)| (name.clone(), CDecl::from_type(ty, config)))
                    .collect();
                self.declarators.push(CDeclarator::Ptr {
                    is_const: false,
                    is_nullable: true,
                    is_ref: false,
                });
                self.declarators.push(CDeclarator::Func(args, false));
                self.build_type(ret, false, config);
            }
        }
    }

    fn write<F: Write>(&self, out: &mut SourceWriter<F>, ident: Option<&str>, config: &Config) {
        // Write the type-specifier and type-qualifier first
        if !self.type_qualifers.is_empty() {
            write!(out, "{} ", self.type_qualifers);
        }

        if config.language != Language::Cython {
            if let Some(ref ctype) = self.type_ctype {
                write!(out, "{} ", ctype.to_str());
            }
        }

        write!(out, "{}", self.type_name);

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

        #[allow(clippy::while_let_on_iterator)]
        while let Some(declarator) = iter_rev.next() {
            let next_is_pointer = iter_rev.peek().map_or(false, |x| x.is_ptr());

            match *declarator {
                CDeclarator::Ptr {
                    is_const,
                    is_nullable,
                    is_ref,
                } => {
                    out.write(if is_ref { "&" } else { "*" });
                    if is_const {
                        out.write("const ");
                    }
                    if !is_nullable && !is_ref && config.language != Language::Cython {
                        if let Some(attr) = &config.pointer.non_null_attribute {
                            write!(out, "{} ", attr);
                        }
                    }
                }
                CDeclarator::Array(..) => {
                    if next_is_pointer {
                        out.write("(");
                    }
                }
                CDeclarator::Func(..) => {
                    if next_is_pointer {
                        out.write("(");
                    }
                }
            }
        }

        // Write the identifier
        if let Some(ident) = ident {
            write!(out, "{}", ident);
        }

        // Write the right part of declarators after the identifier
        let mut iter = self.declarators.iter();
        let mut last_was_pointer = false;

        #[allow(clippy::while_let_on_iterator)]
        while let Some(declarator) = iter.next() {
            match *declarator {
                CDeclarator::Ptr { .. } => {
                    last_was_pointer = true;
                }
                CDeclarator::Array(ref constant) => {
                    if last_was_pointer {
                        out.write(")");
                    }
                    write!(out, "[{}]", constant);

                    last_was_pointer = false;
                }
                CDeclarator::Func(ref args, layout_vertical) => {
                    if last_was_pointer {
                        out.write(")");
                    }

                    out.write("(");
                    if args.is_empty() && config.language == Language::C {
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

                            arg_ty.write(out, arg_ident, config);
                        }
                        out.pop_tab();
                    } else {
                        for (i, &(ref arg_ident, ref arg_ty)) in args.iter().enumerate() {
                            if i != 0 {
                                out.write(", ");
                            }

                            // Convert &Option<String> to Option<&str>
                            let arg_ident = arg_ident.as_ref().map(|x| x.as_ref());

                            arg_ty.write(out, arg_ident, config);
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
    config: &Config,
) {
    CDecl::from_func(f, layout_vertical, config).write(out, Some(f.path().name()), config);
}

pub fn write_field<F: Write>(out: &mut SourceWriter<F>, t: &Type, ident: &str, config: &Config) {
    CDecl::from_type(t, config).write(out, Some(ident), config);
}

pub fn write_type<F: Write>(out: &mut SourceWriter<F>, t: &Type, config: &Config) {
    CDecl::from_type(t, config).write(out, None, config);
}
