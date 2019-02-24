/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path;

use bindgen::config::{Config, Language};
use bindgen::ir::{
    Constant, Function, ItemContainer, ItemMap, Path as BindgenPath, Static, Struct,
};
use bindgen::writer::{Source, SourceWriter};

/// A bindings header that can be written.
pub struct Bindings {
    pub config: Config,
    /// The map from path to struct, used to lookup whether a given type is a
    /// transparent struct. This is needed to generate code for constants.
    struct_map: ItemMap<Struct>,
    globals: Vec<Static>,
    constants: Vec<Constant>,
    items: Vec<ItemContainer>,
    functions: Vec<Function>,
}

impl Bindings {
    pub(crate) fn new(
        config: Config,
        struct_map: ItemMap<Struct>,
        constants: Vec<Constant>,
        globals: Vec<Static>,
        items: Vec<ItemContainer>,
        functions: Vec<Function>,
    ) -> Bindings {
        Bindings {
            config,
            struct_map,
            globals,
            constants,
            items,
            functions,
        }
    }

    // FIXME(emilio): What to do when the configuration doesn't match?
    pub fn struct_is_transparent(&self, path: &BindgenPath) -> bool {
        let mut any = false;
        self.struct_map.for_items(path, |s| any |= s.is_transparent);
        any
    }

    pub fn write_to_file<P: AsRef<path::Path>>(&self, path: P) -> bool {
        // Don't compare files if we've never written this file before
        if !path.as_ref().is_file() {
            if let Some(parent) = path::Path::new(path.as_ref()).parent() {
                fs::create_dir_all(parent).unwrap();
            }
            self.write(File::create(path).unwrap());
            return true;
        }

        let mut new_file_contents = Vec::new();
        self.write(&mut new_file_contents);

        let mut old_file_contents = Vec::new();
        {
            let mut old_file = File::open(&path).unwrap();
            old_file.read_to_end(&mut old_file_contents).unwrap();
        }

        if old_file_contents != new_file_contents {
            let mut new_file = File::create(&path).unwrap();
            new_file.write_all(&new_file_contents).unwrap();
            true
        } else {
            false
        }
    }

    pub fn write_headers<F: Write>(&self, out: &mut SourceWriter<F>) {
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
            write!(
                out,
                "/* Generated with cbindgen:{} */",
                ::bindgen::config::VERSION
            );
            out.new_line();
        }
        if let Some(ref f) = self.config.autogen_warning {
            out.new_line_if_not_start();
            write!(out, "{}", f);
            out.new_line();
        }

        out.new_line_if_not_start();
        if !self.config.no_includes {
            if self.config.language == Language::C {
                out.write("#include <stdarg.h>");
                out.new_line();
                out.write("#include <stdbool.h>");
                out.new_line();
                out.write("#include <stdint.h>");
                out.new_line();
                out.write("#include <stdlib.h>");
                out.new_line();
            } else {
                out.write("#include <cstdarg>");
                out.new_line();
                out.write("#include <cstdint>");
                out.new_line();
                out.write("#include <cstdlib>");
                out.new_line();
            }
        }

        for include in &self.config.sys_includes {
            write!(out, "#include <{}>", include);
            out.new_line();
        }

        for include in &self.config.includes {
            write!(out, "#include \"{}\"", include);
            out.new_line();
        }
    }

    pub fn write<F: Write>(&self, file: F) {
        let mut out = SourceWriter::new(file, self);

        if !self.config.no_includes
            || !self.config.includes.is_empty()
            || !self.config.sys_includes.is_empty()
        {
            self.write_headers(&mut out);
        }

        if self.config.language == Language::Cxx {
            self.open_namespaces(&mut out);
        }

        for constant in &self.constants {
            if constant.ty.is_primitive_or_ptr_primitive() {
                out.new_line_if_not_start();
                constant.write(&self.config, &mut out, None);
                out.new_line();
            }
        }

        for item in &self.items {
            if item
                .deref()
                .annotations()
                .bool("no-export")
                .unwrap_or(false)
            {
                continue;
            }

            out.new_line_if_not_start();
            match *item {
                ItemContainer::Constant(..) => unreachable!(),
                ItemContainer::Static(..) => unreachable!(),
                ItemContainer::Enum(ref x) => x.write(&self.config, &mut out),
                ItemContainer::Struct(ref x) => x.write(&self.config, &mut out),
                ItemContainer::Union(ref x) => x.write(&self.config, &mut out),
                ItemContainer::OpaqueItem(ref x) => x.write(&self.config, &mut out),
                ItemContainer::Typedef(ref x) => x.write(&self.config, &mut out),
            }
            out.new_line();
        }

        for constant in &self.constants {
            if !constant.ty.is_primitive_or_ptr_primitive() {
                out.new_line_if_not_start();
                constant.write(&self.config, &mut out, None);
                out.new_line();
            }
        }

        if !self.functions.is_empty() || !self.globals.is_empty() {
            if self.config.language == Language::Cxx {
                out.new_line_if_not_start();
                out.write("extern \"C\" {");
                out.new_line();
            }

            for global in &self.globals {
                out.new_line_if_not_start();
                global.write(&self.config, &mut out);
                out.new_line();
            }

            for function in &self.functions {
                out.new_line_if_not_start();
                function.write(&self.config, &mut out);
                out.new_line();
            }

            if self.config.language == Language::Cxx {
                out.new_line_if_not_start();
                out.write("} // extern \"C\"");
                out.new_line();
            }
        }

        if self.config.language == Language::Cxx {
            self.close_namespaces(&mut out);
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
