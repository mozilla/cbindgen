/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;

use crate::bindgen::config::Layout;
use crate::bindgen::declarationtyperesolver::DeclarationType;
use crate::bindgen::ir::{ConstExpr, Function, GenericArgument, Type};
use crate::bindgen::language_backend::LanguageBackend;
use crate::bindgen::writer::{ListType, SourceWriter};
use crate::bindgen::{Config, Language};

use super::ir::AnnotationValue;

/// Annotation that allows specifying a prefix for function arguments.
/// It can be used like this:
/// ```rust
/// /// cbindgen:function-arg-prefix[bar]=_In_
/// fn foo(bar: *const u64) {}
/// ```
///
/// This will generate the following code:
/// ```c
/// void root(_In_ uint64_t* input);
/// ```
pub const ANNOTATION_FUNCTION_ARG_PREFIX: &str = "function-arg-prefix";

/// Annotation that allows specifying a prefix for function arguments.
/// It can be used like this:
/// ```rust
/// /// cbindgen:function-arg-ident-prefix[bar]=_NonNull
/// fn foo(bar: *const u64) {}
/// ```
///
/// This will generate the following code:
/// ```c
/// void root(uint64_t* _NonNull input);
/// ```
pub const ANNOTATION_FUNCTION_ARG_IDENT_PREFIX: &str = "function-arg-ident-prefix";

/// Annotation that allows specifying a prefix for function declarations.
/// It can be used like this:
/// ```rust
/// /// cbindgen:function-prefix=TEST_MACRO
/// fn root(input: *const u64) {}
/// ```
///
/// This will generate the following code:
/// ```c
/// TEST_MACRO void root(uint64_t* input);
/// ```
pub const ANNOTATION_FUNCTION_PREFIX: &str = "function-prefix";

/// Annotation that allows specifying a prefix for function declarations.
/// It can be used like this:
/// ```rust
/// /// cbindgen:function-postfix=TEST_MACRO
/// fn root(input: *const u64) {}
/// ```
///
/// This will generate the following code:
/// ```c
/// void root(uint64_t* input) TEST_MACRO;
/// ```
pub const ANNOTATION_FUNCTION_POSTFIX: &str = "function-postfix";

/// Annotation that allows specifying a prefix for function declarations.
/// It can be used like this:
/// ```rust
/// /// cbindgen:function-ident-prefix=TEST_MACRO
/// fn root(input: *const u64) {}
/// ```
///
/// This will generate the following code:
/// ```c
/// void TEST_MACRO root(uint64_t* input) ;
/// ```
pub const ANNOTATION_FUNCTION_IDENT_PREFIX: &str = "function-ident-prefix";

// This code is for translating Rust types into C declarations.
// See Section 6.7, Declarations, in the C standard for background.
// http://www.open-std.org/jtc1/sc22/wg14/www/docs/n1570.pdf
#[derive(Debug)]
enum CDeclarator {
    Ptr {
        is_const: bool,
        is_nullable: bool,
        is_ref: bool,
    },
    Array(String),
    Func {
        args: Vec<CFuncArg>,
        layout: Layout,
        never_return: bool,
        postfix: Option<String>,
    },
}

impl CDeclarator {
    fn is_ptr(&self) -> bool {
        matches!(self, CDeclarator::Ptr { .. } | CDeclarator::Func { .. })
    }
}

#[derive(Debug)]
struct CFuncArg {
    ident: Option<String>,
    r#type: CDecl,
}

#[derive(Debug)]
struct CDecl {
    type_qualifers: String,
    type_name: String,
    type_generic_args: Vec<GenericArgument>,
    declarators: Vec<CDeclarator>,
    type_ctype: Option<DeclarationType>,
    deprecated: Option<String>,
    // Prefix that should be added before the declaration
    prefix: Option<String>,
    // Prefix that should be added before the
    // identifier but not as a part of the identifier
    ident_prefix: Option<String>,
}

impl CDecl {
    fn new() -> CDecl {
        CDecl {
            type_qualifers: String::new(),
            type_name: String::new(),
            type_generic_args: Vec::new(),
            declarators: Vec::new(),
            type_ctype: None,
            deprecated: None,
            prefix: None,
            ident_prefix: None,
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
        let ptr_as_array = Type::Array(ty.clone(), ConstExpr::Value(length.to_string()));
        cdecl.build_type(&ptr_as_array, *is_const, config);
        cdecl
    }

    fn from_func(f: &Function, layout: Layout, config: &Config) -> CDecl {
        let mut cdecl = CDecl::new();
        cdecl.build_func(f, layout, config);
        cdecl
    }

    fn build_func(&mut self, f: &Function, layout: Layout, config: &Config) {
        let arg_ident_prefixes = f
            .annotations
            .dict(ANNOTATION_FUNCTION_ARG_IDENT_PREFIX)
            .unwrap_or_default();
        let arg_prefixes = f
            .annotations
            .dict(ANNOTATION_FUNCTION_ARG_PREFIX)
            .unwrap_or_default();

        let args = f
            .args
            .iter()
            .map(|arg| {
                let ident_prefix = arg
                    .name
                    .as_ref()
                    .and_then(|name| arg_ident_prefixes.get(name).cloned());
                let ident_prefix = match ident_prefix {
                    Some(AnnotationValue::Atom(prefix)) => prefix,
                    _ => None,
                };

                let prefix = arg
                    .name
                    .as_ref()
                    .and_then(|name| arg_prefixes.get(name).cloned());

                let prefix = match prefix {
                    Some(AnnotationValue::Atom(prefix)) => prefix,
                    _ => None,
                };

                let mut arg = CFuncArg {
                    ident: arg.name.clone(),
                    r#type: CDecl::from_func_arg(&arg.ty, arg.array_length.as_deref(), config),
                };

                arg.r#type.ident_prefix = ident_prefix;
                arg.r#type.prefix = prefix;
                arg
            })
            .collect();

        self.declarators.push(CDeclarator::Func {
            args,
            layout,
            never_return: f.never_return,
            postfix: f.annotations.atom(ANNOTATION_FUNCTION_POSTFIX).flatten(),
        });
        self.ident_prefix = f
            .annotations
            .atom(ANNOTATION_FUNCTION_IDENT_PREFIX)
            .flatten();
        self.prefix = f.annotations.atom(ANNOTATION_FUNCTION_PREFIX).flatten();
        self.deprecated.clone_from(&f.annotations.deprecated);
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
                    "const".clone_into(&mut self.type_qualifers);
                }

                assert!(
                    self.type_name.is_empty(),
                    "error generating cdecl for {:?}",
                    t
                );
                generic.export_name().clone_into(&mut self.type_name);
                assert!(
                    self.type_generic_args.is_empty(),
                    "error generating cdecl for {:?}",
                    t
                );
                generic.generics().clone_into(&mut self.type_generic_args);
                self.type_ctype = generic.ctype().cloned();
            }
            Type::Primitive(ref p) => {
                if is_const {
                    assert!(
                        self.type_qualifers.is_empty(),
                        "error generating cdecl for {:?}",
                        t
                    );
                    "const".clone_into(&mut self.type_qualifers);
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
                never_return,
            } => {
                let args = args
                    .iter()
                    .map(|(ref name, ref ty)| CFuncArg {
                        ident: name.clone(),
                        r#type: CDecl::from_type(ty, config),
                    })
                    .collect();
                self.declarators.push(CDeclarator::Ptr {
                    is_const: false,
                    is_nullable: true,
                    is_ref: false,
                });
                self.declarators.push(CDeclarator::Func {
                    args,
                    layout: config.function.args,
                    never_return: *never_return,
                    postfix: None,
                });
                self.build_type(ret, false, config);
            }
        }
    }

    fn write<F: Write, LB: LanguageBackend>(
        &self,
        language_backend: &mut LB,
        out: &mut SourceWriter<F>,
        ident: Option<&str>,
        config: &Config,
    ) {
        if config.language != Language::Cython {
            if let Some(prefix) = &self.prefix {
                write!(out, "{} ", prefix);
            }
        }

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
            out.write_horizontal_source_list(
                language_backend,
                &self.type_generic_args,
                ListType::Join(", "),
                |language_backend, out, g| match *g {
                    GenericArgument::Type(ref ty) => language_backend.write_type(out, ty),
                    GenericArgument::Const(ref expr) => write!(out, "{}", expr.as_str()),
                },
            );
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
            let next_is_pointer = iter_rev.peek().is_some_and(|x| x.is_ptr());
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
                    if config.language != Language::Cython {
                        if !is_nullable && !is_ref {
                            if let Some(attr) = &config.pointer.non_null_attribute {
                                write!(out, "{} ", attr);
                            }
                        } else if is_nullable {
                            if let Some(attr) = &config.pointer.nullable_attribute {
                                write!(out, "{} ", attr);
                            }
                        }
                    }
                }
                CDeclarator::Array(..) => {
                    if next_is_pointer {
                        out.write("(");
                    }
                }
                CDeclarator::Func { .. } => {
                    if next_is_pointer {
                        out.write("(");
                    }
                }
            }
        }

        // Write the identifier
        if let Some(ident) = ident {
            if config.language != Language::Cython {
                if let Some(prefix) = &self.ident_prefix {
                    write!(out, "{} ", prefix);
                }
            }

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
                CDeclarator::Func {
                    ref args,
                    ref layout,
                    never_return,
                    ref postfix,
                    ..
                } => {
                    if last_was_pointer {
                        out.write(")");
                    }
                    out.write("(");
                    if args.is_empty() && config.language == Language::C {
                        out.write("void");
                    }

                    fn write_vertical<F: Write, LB: LanguageBackend>(
                        language_backend: &mut LB,
                        out: &mut SourceWriter<F>,
                        config: &Config,
                        args: &[CFuncArg],
                    ) {
                        let align_length = out.line_length_for_align();
                        out.push_set_spaces(align_length);
                        for (i, arg) in args.iter().enumerate() {
                            if i != 0 {
                                out.write(",");
                                out.new_line();
                            }

                            // Convert &Option<String> to Option<&str>
                            let arg_ident = arg.ident.as_ref().map(|x| x.as_ref());

                            arg.r#type.write(language_backend, out, arg_ident, config);
                        }
                        out.pop_tab();
                    }

                    fn write_horizontal<F: Write, LB: LanguageBackend>(
                        language_backend: &mut LB,
                        out: &mut SourceWriter<F>,
                        config: &Config,
                        args: &[CFuncArg],
                    ) {
                        for (i, arg) in args.iter().enumerate() {
                            if i != 0 {
                                out.write(", ");
                            }

                            // Convert &Option<String> to Option<&str>
                            let arg_ident = arg.ident.as_ref().map(|x| x.as_ref());

                            arg.r#type.write(language_backend, out, arg_ident, config);
                        }
                    }

                    match layout {
                        Layout::Vertical => write_vertical(language_backend, out, config, args),
                        Layout::Horizontal => write_horizontal(language_backend, out, config, args),
                        Layout::Auto => {
                            if !out.try_write(
                                |out| write_horizontal(language_backend, out, config, args),
                                config.line_length,
                            ) {
                                write_vertical(language_backend, out, config, args)
                            }
                        }
                    }
                    out.write(")");

                    if never_return && config.language != Language::Cython {
                        if let Some(ref no_return_attr) = config.function.no_return {
                            out.write_fmt(format_args!(" {}", no_return_attr));
                        }
                    }

                    if config.language != Language::Cython {
                        if let Some(attr) = postfix {
                            write!(out, " {}", attr);
                        }
                    }

                    last_was_pointer = true;
                }
            }
        }
    }
}

pub fn write_func<F: Write, LB: LanguageBackend>(
    language_backend: &mut LB,
    out: &mut SourceWriter<F>,
    f: &Function,
    layout: Layout,
    config: &Config,
) {
    CDecl::from_func(f, layout, config).write(language_backend, out, Some(f.path().name()), config);
}

pub fn write_field<F: Write, LB: LanguageBackend>(
    language_backend: &mut LB,
    out: &mut SourceWriter<F>,
    t: &Type,
    ident: &str,
    config: &Config,
) {
    CDecl::from_type(t, config).write(language_backend, out, Some(ident), config);
}

pub fn write_type<F: Write, LB: LanguageBackend>(
    language_backend: &mut LB,
    out: &mut SourceWriter<F>,
    t: &Type,
    config: &Config,
) {
    CDecl::from_type(t, config).write(language_backend, out, None, config);
}
