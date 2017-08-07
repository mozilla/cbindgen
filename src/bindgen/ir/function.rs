/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;

use syn;

use bindgen::annotation::*;
use bindgen::cdecl;
use bindgen::config::{Config, Layout};
use bindgen::ir::*;
use bindgen::library::*;
use bindgen::rename::*;
use bindgen::utilities::*;
use bindgen::writer::*;
use bindgen::mangle;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub annotations: AnnotationSet,
    pub ret: Type,
    pub args: Vec<(String, Type)>,
    pub extern_decl: bool,
    pub documentation: Documentation,
}

impl Function {
    pub fn load(name: String,
                annotations: AnnotationSet,
                decl: &syn::FnDecl,
                extern_decl: bool,
                doc: String) -> Result<Function, String>
    {
        let args = decl.inputs.iter()
                              .try_skip_map(|x| x.as_ident_and_type())?;
        let ret = decl.output.as_type()?;

        Ok(Function {
            name: name,
            annotations: annotations,
            ret: ret,
            args: args,
            extern_decl: extern_decl,
            documentation: Documentation::load(doc),
        })
    }

    pub fn add_member_function(&self, out: &mut MemberFunctions) {
        if let Some(&(_, ref ty)) = self.args.get(0) {
            match *ty {
                Type::ConstPtr(ref t) | Type::Ptr(ref t) => {
                    let t = match **t {
                        Type::Path(ref t, ref g) if g.is_empty() => {
                            Type::Path(t.to_owned(), Vec::new())
                        }
                        Type::Path(ref p, ref g) => {
                            Type::Path(mangle::mangle_path(p, g), Vec::new())
                        }
                        _ => return
                    };
                    out.entry(t)
                        .or_insert_with(Vec::new)
                        .push(self.clone())
                }
                _ => {}
            }
        }
    }

    pub fn add_deps(&self, library: &Library, out: &mut DependencyList) {
        self.ret.add_deps(library, out);
        for &(_, ref ty) in &self.args {
            ty.add_deps(library, out);
        }
    }

    pub fn add_monomorphs(&self, library: &Library, out: &mut Monomorphs) {
        self.ret.add_monomorphs(library, out);
        for &(_, ref ty) in &self.args {
            ty.add_monomorphs(library, out);
        }
    }

    pub fn add_specializations(&self, library: &Library, out: &mut SpecializationList) {
        self.ret.add_specializations(library, out);
        for &(_, ref ty) in &self.args {
            ty.add_specializations(library, out);
        }
    }

    pub fn rename_args(&mut self, config: &Config) {
        let rules = [self.annotations.parse_atom::<RenameRule>("rename-all"),
                     config.function.rename_args];

        if let Some(r) = find_first_some(&rules) {
            self.args = self.args.iter()
                                 .map(|x| (r.apply_to_snake_case(&x.0,
                                                                 IdentifierType::FunctionArg),
                                           x.1.clone()))
                                  .collect()
        }
    }

    pub fn mangle_paths(&mut self, monomorphs: &Monomorphs) {
        self.ret.mangle_paths(monomorphs);
        for &mut (_, ref mut ty) in &mut self.args {
            ty.mangle_paths(monomorphs);
        }
    }

    pub fn as_member(self) -> Self{
        Function {
            args: self.args[1..].to_owned(),
            ..self
        }
    }

    pub fn write_formated<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>, add_semicolon: bool) {
        fn write_1<W: Write>(func: &Function, config: &Config, out: &mut SourceWriter<W>, add_semicolon: bool) {
            let prefix = config.function.prefix(&func.annotations);
            let postfix = config.function.postfix(&func.annotations);

            func.documentation.write(config, out);
            if let Some(ref prefix) = prefix {
                out.write(prefix);
                out.write(" ");
            }
            cdecl::write_func(out, &func, false);
            if let Some(ref postfix) = postfix {
                out.write(" ");
                out.write(postfix);
            }
            if add_semicolon {
                out.write(";");
            }
        }

        fn write_2<W: Write>(func: &Function, config: &Config, out: &mut SourceWriter<W>, add_semicolon: bool) {
            let prefix = config.function.prefix(&func.annotations);
            let postfix = config.function.postfix(&func.annotations);

            func.documentation.write(config, out);
            if let Some(ref prefix) = prefix {
                out.write(prefix);
                out.new_line();
            }
            cdecl::write_func(out, &func, true);
            if let Some(ref postfix) = postfix {
                out.new_line();
                out.write(postfix);
            }
            if add_semicolon {
                out.write(";");
            }
        };

        let option_1 = out.measure(|out| write_1(self, config, out, add_semicolon));

        if (config.function.args == Layout::Auto && option_1 <= config.line_length) ||
           config.function.args == Layout::Horizontal {
            write_1(self, config, out, add_semicolon);
        } else {
            write_2(self, config, out, add_semicolon);
        }
    }
}

impl Source for Function {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        self.write_formated(config, out, true)
    }
}

pub trait SynFnArgHelpers {
    fn as_ident_and_type(&self) -> Result<Option<(String, Type)>, String>;
}

impl SynFnArgHelpers for syn::FnArg {
    fn as_ident_and_type(&self) -> Result<Option<(String, Type)>, String> {
        match self {
            &syn::FnArg::Captured(syn::Pat::Ident(_, ref ident, _), ref ty) => {
                if let Some(x) = Type::load(ty)? {
                    Ok(Some((ident.to_string(), x)))
                } else {
                    Ok(None)
                }
            }
            _ => Err(format!("parameter has unexpected type")),
        }
    }
}
