/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;
use std::fs::File;
use std::path;
use std::fs;

use bindgen::config::{Config, Language};
use bindgen::ir::{ItemContainer, Function};
use bindgen::monomorph::TemplateSpecialization;
use bindgen::writer::{ListType, Source, SourceWriter};

pub struct Bindings {
    config: Config,
    items: Vec<ItemContainer>,
    functions: Vec<Function>,
    template_specializations: Vec<TemplateSpecialization>,
}

impl Bindings {
    pub fn new(config: Config,
               items: Vec<ItemContainer>,
               functions: Vec<Function>,
               template_specializations: Vec<TemplateSpecialization>) -> Bindings {
        Bindings {
            config: config,
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
            out.write(&f);
            out.new_line();
        }
        if let Some(ref f) = self.config.include_guard {
            out.new_line_if_not_start();
            out.write(&format!("#ifndef {}", f));
            out.new_line();
            out.write(&format!("#define {}", f));
            out.new_line();
        }
        if self.config.include_version {
            out.new_line_if_not_start();
            out.write(&format!("/* Generated with cbindgen:{} */",
                      ::bindgen::config::VERSION));
            out.new_line();
        }
        if let Some(ref f) = self.config.autogen_warning {
            out.new_line_if_not_start();
            out.write(&f);
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

        for item in &self.items {
            out.new_line_if_not_start();
            match item {
                &ItemContainer::Enum(ref x) => x.write(&self.config, &mut out),
                &ItemContainer::Struct(ref x) => x.write(&self.config, &mut out),
                &ItemContainer::OpaqueItem(ref x) => x.write(&self.config, &mut out),
                &ItemContainer::Typedef(ref x) => x.write(&self.config, &mut out),
                &ItemContainer::Specialization(_) => {
                    unreachable!("should not encounter a specialization in a generated library")
                }
            }
            out.new_line();
        }

        if let Some(ref f) = self.config.autogen_warning {
            out.new_line_if_not_start();
            out.write(&f);
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
                  out.write("typename ");
                  out.write(param);
              }
              out.write(">");
              out.new_line();
              out.write(&format!("struct {};", template.generic.name));
              out.new_line();

              for &(ref monomorph_path, ref generic_values) in &template.monomorphs {
                out.new_line();
                out.write("template<>");
                out.new_line();
                out.write(&format!("struct {}<", template.generic.name));
                out.write_horizontal_source_list(generic_values, ListType::Join(", "));
                out.write(&format!("> : public {}", monomorph_path));
                out.open_brace();
                out.close_brace(true);
                out.new_line();
              }
            }
            self.close_namespaces(&mut out);
        }

        if let Some(ref f) = self.config.autogen_warning {
            out.new_line_if_not_start();
            out.write(&f);
            out.new_line();
        }
        if let Some(ref f) = self.config.include_guard {
            out.new_line_if_not_start();
            if self.config.language == Language::C {
                out.write(&format!("#endif /* {} */", f));
            } else {
                out.write(&format!("#endif // {}", f));
            }
            out.new_line();
        }
        if let Some(ref f) = self.config.trailer {
            out.new_line_if_not_start();
            out.write(&f);
            out.new_line();
        }
    }

    pub fn open_namespaces<F: Write>(&self, out: &mut SourceWriter<F>) {
        let mut wrote_namespace: bool = false;
        if let Some(ref namespace) = self.config.namespace {
            wrote_namespace = true;

            out.new_line();
            out.write("namespace ");
            out.write(namespace);
            out.write(" {");
        }
        if let Some(ref namespaces) = self.config.namespaces {
            wrote_namespace = true;
            for namespace in namespaces {
                out.new_line();
                out.write("namespace ");
                out.write(namespace);
                out.write(" {");
            }
        }
        if wrote_namespace {
            out.new_line();
        }
    }

    pub fn close_namespaces<F: Write>(&self, out: &mut SourceWriter<F>) {
        let mut wrote_namespace: bool = false;
        if let Some(ref namespaces) = self.config.namespaces {
            wrote_namespace = true;

            for namespace in namespaces.iter().rev() {
                out.new_line_if_not_start();
                out.write("} // namespace ");
                out.write(namespace);
            }
        }
        if let Some(ref namespace) = self.config.namespace {
            wrote_namespace = true;

            out.new_line_if_not_start();
            out.write("} // namespace ");
            out.write(namespace);
        }
        if wrote_namespace {
            out.new_line();
        }
    }
}
