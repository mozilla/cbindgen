/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;
use std::fs::File;
use std::path;
use std::fs;

use bindgen::config::{Config, Language};
use bindgen::ir::Item;
use bindgen::writer::{Source, SourceWriter};

pub struct Bindings {
    config: Config,
    items: Vec<Item>,
}

impl Bindings {
    pub fn new(config: Config,
               items: Vec<Item>) -> Bindings {
        Bindings {
            config,
            items,
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

            self.open_namespaces(&mut out);
        }


        if let Some(ref f) = self.config.autogen_warning {
            out.new_line_if_not_start();
            out.write(&f);
            out.new_line();
        }

        for item in &self.items {
            out.new_line_if_not_start();
            item.write(&self.config, &mut out);
            out.new_line();
        }

        if self.config.language == Language::Cxx {
            self.close_namespaces(&mut out);
        }

        if let Some(ref f) = self.config.autogen_warning {
            out.new_line_if_not_start();
            out.write(&f);
            out.new_line();
        }
        if let Some(ref f) = self.config.include_guard {
            out.new_line_if_not_start();
            out.write(&format!("#endif // {}", f));
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
