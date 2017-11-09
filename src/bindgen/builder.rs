/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::path;

use bindgen::cargo::Cargo;
use bindgen::config::Config;
use bindgen::library::Library;
use bindgen::parser::{self, Parse};

#[derive(Debug, Clone)]
pub struct LibraryBuilder {
    config: Config,
    srcs: Vec<path::PathBuf>,
    lib: Option<Cargo>,
    std_types: bool,
}

impl LibraryBuilder {
    pub fn new() -> LibraryBuilder {
        LibraryBuilder {
            config: Config::default(),
            srcs: Vec::new(),
            lib: None,
            std_types: false,
        }
    }

    pub fn with_config(mut self, config: Config) -> LibraryBuilder {
        self.config = config;
        self
    }

    pub fn with_std_types(mut self) -> LibraryBuilder {
        self.std_types = true;
        self
    }

    pub fn with_src(mut self, src: &path::Path) -> LibraryBuilder {
        self.srcs.push(src.to_owned());
        self
    }

    pub fn with_crate(mut self, lib: Cargo) -> LibraryBuilder {
        debug_assert!(self.lib.is_none());
        self.lib = Some(lib);
        self
    }

    pub fn build(self) -> Result<Library, String> {
        let mut result = Parse::new();

        if self.std_types {
            result.add_std_types();
        }

        for x in &self.srcs {
            result.extend_with(&parser::parse_src(x)?);
        }

        if let Some(x) = self.lib.clone() {
            result.extend_with(&parser::parse_lib(x,
                                                  self.config.parse.parse_deps,
                                                  &self.config.parse.include,
                                                  &self.config.parse.exclude,
                                                  &self.config.parse.expand)?);
        }

        result.functions.sort_by(|x, y| x.name.cmp(&y.name));

        Ok(Library::new(self.config,
                        result.constants,
                        result.globals,
                        result.enums,
                        result.structs,
                        result.unions,
                        result.opaque_items,
                        result.typedefs,
                        result.specializations,
                        result.functions))
    }
}
