/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;
use std::fs::File;
use std::path;
use std::fs;

use bindgen::config::{Config, Language};
use bindgen::ir::{Constant, ItemContainer, Function, Static};
use bindgen::monomorph::TemplateSpecialization;
use bindgen::writer::{ListType, Source, SourceWriter};

/// A bindings header that can be written.
pub struct Bindings {
    config: Config,
    globals: Vec<Static>,
    constants: Vec<Constant>,
    items: Vec<ItemContainer>,
    functions: Vec<Function>,
    template_specializations: Vec<TemplateSpecialization>,
}

impl Bindings {
    pub(crate) fn new(config: Config,
                      constants: Vec<Constant>,
                      globals: Vec<Static>,
                      items: Vec<ItemContainer>,
                      functions: Vec<Function>,
                      template_specializations: Vec<TemplateSpecialization>) -> Bindings {
        Bindings {
            config: config,
            globals: globals,
            constants: constants,
            items: items,
            functions: functions,
            template_specializations: template_specializations,
        }
    }

    pub fn write_to_file(&self, path: &str) {
        if let Some(parent) = path::Path::new(path).parent() {
            fs::create_dir_all(parent).unwrap();
        }

        self.write(File::create(path).unwrap());
    }

    pub fn write<F: Write>(&self, file: F) {
        let mut out = SourceWriter::new(file, &self.config);

        if let Some(ref f) = self.config.header {
            out.new_line_if_not_start();
            write!(out, "{}", f);
            out.new_line();
        }
        if let Some(ref f) = self.config.include_guard {
            out.new_line_if_not_start();
            write!(out, "#ifndef {}", f);
            out.new_line();
            write!(out, "#define {}", f);
            out.new_line();
        }
        if self.config.include_version {
            out.new_line_if_not_start();
            write!(out, "/* Generated with cbindgen:{} */",
                        ::bindgen::config::VERSION);
            out.new_line();
        }
        if let Some(ref f) = self.config.autogen_warning {
            out.new_line_if_not_start();
            write!(out, "{}", f);
            out.new_line();
        }

        out.new_line_if_not_start();
        if self.config.language == Language::C {
            out.write("#include <stdint.h>");
            out.new_line();
            out.write("#include <stdlib.h>");
            out.new_line();
            out.write("#include <stdbool.h>");
        } else {
            out.write("#include <cstdint>");
            out.new_line();
            out.write("#include <cstdlib>");
        }
        out.new_line();

        if self.config.language == Language::Cxx {
            out.new_line_if_not_start();
            out.write("extern \"C\" {");
            out.new_line();

            self.open_namespaces(&mut out);
        }

        for constant in &self.constants {
            out.new_line_if_not_start();
            constant.write(&self.config, &mut out);
            out.new_line();
        }

        for item in &self.items {
            if item.deref().annotations().bool("no-export").unwrap_or(false) {
                continue;
            }

            out.new_line_if_not_start();
            match item {
                &ItemContainer::Constant(..) => unreachable!(),
                &ItemContainer::Static(..) => unreachable!(),
                &ItemContainer::Enum(ref x) => x.write(&self.config, &mut out),
                &ItemContainer::Struct(ref x) => x.write(&self.config, &mut out),
                &ItemContainer::Union(ref x) => x.write(&self.config, &mut out),
                &ItemContainer::OpaqueItem(ref x) => x.write(&self.config, &mut out),
                &ItemContainer::Typedef(ref x) => x.write(&self.config, &mut out),
                &ItemContainer::Specialization(_) => {
                    unreachable!("should not encounter a specialization in a generated library")
                }
            }
            out.new_line();
        }

        for global in &self.globals {
            out.new_line_if_not_start();
            global.write(&self.config, &mut out);
            out.new_line();
        }

        if let Some(ref f) = self.config.autogen_warning {
            out.new_line_if_not_start();
            write!(out, "{}", f);
            out.new_line();
        }

        for function in &self.functions {
            out.new_line_if_not_start();
            function.write(&self.config, &mut out);
            out.new_line();
        }

        if self.config.language == Language::Cxx {
            self.close_namespaces(&mut out);

            out.new_line_if_not_start();
            out.write("} // extern \"C\"");
            out.new_line();
        }

        if self.config.structure.generic_template_specialization &&
           self.config.language == Language::Cxx {
            self.open_namespaces(&mut out);
            for template in &self.template_specializations {
              out.new_line_if_not_start();
              out.write("template<");
              for (i, param) in template.generic.generic_params.iter().enumerate() {
                  if i != 0 {
                      out.write(", ")
                  }
                  write!(out, "typename {}", param);
              }
              out.write(">");
              out.new_line();
              write!(out, "struct {};", template.generic.name);
              out.new_line();

              for &(ref monomorph_path, ref generic_values) in &template.monomorphs {
                out.new_line();
                out.write("template<>");
                out.new_line();
                write!(out, "struct {}<", template.generic.name);
                out.write_horizontal_source_list(generic_values, ListType::Join(", "));
                write!(out, "> : public {}", monomorph_path);
                out.open_brace();
                out.close_brace(true);
                out.new_line();
              }
            }
            self.close_namespaces(&mut out);
        }

        if let Some(ref f) = self.config.autogen_warning {
            out.new_line_if_not_start();
            write!(out, "{}", f);
            out.new_line();
        }
        if let Some(ref f) = self.config.include_guard {
            out.new_line_if_not_start();
            if self.config.language == Language::C {
                write!(out, "#endif /* {} */", f);
            } else {
                write!(out, "#endif // {}", f);
            }
            out.new_line();
        }
        if let Some(ref f) = self.config.trailer {
            out.new_line_if_not_start();
            write!(out, "{}", f);
            out.new_line();
        }
    }

    pub(crate) fn open_namespaces<F: Write>(&self, out: &mut SourceWriter<F>) {
        let mut wrote_namespace: bool = false;
        if let Some(ref namespace) = self.config.namespace {
            wrote_namespace = true;

            out.new_line();
            write!(out, "namespace {} {{", namespace);
        }
        if let Some(ref namespaces) = self.config.namespaces {
            wrote_namespace = true;
            for namespace in namespaces {
                out.new_line();
                write!(out, "namespace {} {{", namespace);
            }
        }
        if wrote_namespace {
            out.new_line();
        }
    }

    pub(crate) fn close_namespaces<F: Write>(&self, out: &mut SourceWriter<F>) {
        let mut wrote_namespace: bool = false;
        if let Some(ref namespaces) = self.config.namespaces {
            wrote_namespace = true;

            for namespace in namespaces.iter().rev() {
                out.new_line_if_not_start();
                write!(out, "}} // namespace {}", namespace);
            }
        }
        if let Some(ref namespace) = self.config.namespace {
            wrote_namespace = true;

            out.new_line_if_not_start();
            write!(out, "}} // namespace {}", namespace);
        }
        if wrote_namespace {
            out.new_line();
        }
    }
}
