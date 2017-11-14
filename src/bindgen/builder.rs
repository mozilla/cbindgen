/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::path;

use bindgen::cargo::Cargo;
use bindgen::config::Config;
use bindgen::library::Library;
use bindgen::bindings::Bindings;
use bindgen::parser::{self, Parse};

/// A builder for generating a bindings header.
#[derive(Debug, Clone)]
pub struct Builder {
    config: Config,
    srcs: Vec<path::PathBuf>,
    lib: Option<(path::PathBuf, Option<String>)>,
    lib_cargo: Option<Cargo>,
    std_types: bool,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            config: Config::default(),
            srcs: Vec::new(),
            lib: None,
            lib_cargo: None,
            std_types: false,
        }
    }

    pub fn with_config(mut self, config: Config) -> Builder {
        self.config = config;
        self
    }

    pub fn with_std_types(mut self) -> Builder {
        self.std_types = true;
        self
    }

    pub fn with_src(mut self, src: &path::Path) -> Builder {
        self.srcs.push(src.to_owned());
        self
    }

    #[allow(unused)]
    pub fn with_crate<P: AsRef<path::Path>>(mut self, lib_dir: P) -> Builder {
        debug_assert!(self.lib.is_none());
        debug_assert!(self.lib_cargo.is_none());
        self.lib = Some((path::PathBuf::from(lib_dir.as_ref()), None));
        self
    }

    #[allow(unused)]
    pub fn with_crate_and_name<P: AsRef<path::Path>,
                               S: AsRef<str>>(mut self,
                                              lib_dir: P,
                                              binding_lib_name: S) -> Builder
    {
        debug_assert!(self.lib.is_none());
        debug_assert!(self.lib_cargo.is_none());
        self.lib = Some((path::PathBuf::from(lib_dir.as_ref()),
                         Some(String::from(binding_lib_name.as_ref()))));
        self
    }

    #[allow(unused)]
    pub(crate) fn with_cargo(mut self, lib: Cargo) -> Builder {
        debug_assert!(self.lib.is_none());
        debug_assert!(self.lib_cargo.is_none());
        self.lib_cargo = Some(lib);
        self
    }

    pub fn generate(self) -> Result<Bindings, String> {
        let mut result = Parse::new();

        if self.std_types {
            result.add_std_types();
        }

        for x in &self.srcs {
            result.extend_with(&parser::parse_src(x)?);
        }

        if let Some((lib_dir, binding_lib_name)) = self.lib.clone() {
            let cargo = if let Some(binding_lib_name) = binding_lib_name {
                Cargo::load(&lib_dir,
                            Some(&binding_lib_name),
                            self.config.parse.parse_deps)?
            } else {
                Cargo::load(&lib_dir,
                            None,
                            self.config.parse.parse_deps)?
            };

            result.extend_with(&parser::parse_lib(cargo,
                                                  self.config.parse.parse_deps,
                                                  &self.config.parse.include,
                                                  &self.config.parse.exclude,
                                                  &self.config.parse.expand)?);
        } else if let Some(cargo) = self.lib_cargo.clone() {
            result.extend_with(&parser::parse_lib(cargo,
                                                  self.config.parse.parse_deps,
                                                  &self.config.parse.include,
                                                  &self.config.parse.exclude,
                                                  &self.config.parse.expand)?);
        }

        Library::new(self.config,
                     result.constants,
                     result.globals,
                     result.enums,
                     result.structs,
                     result.unions,
                     result.opaque_items,
                     result.typedefs,
                     result.specializations,
                     result.functions).generate()
    }
}
