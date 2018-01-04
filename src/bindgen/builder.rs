/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::path;

use bindgen::cargo::Cargo;
use bindgen::config::{Braces, Config, Language};
use bindgen::error::Error;
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
            std_types: true,
        }
    }

    #[allow(unused)]
    pub fn with_header<S: AsRef<str>>(mut self, header: S) -> Builder {
        self.config.header = Some(String::from(header.as_ref()));
        self
    }

    #[allow(unused)]
    pub fn with_trailer<S: AsRef<str>>(mut self, trailer: S) -> Builder {
        self.config.trailer = Some(String::from(trailer.as_ref()));
        self
    }

    #[allow(unused)]
    pub fn with_include_guard<S: AsRef<str>>(mut self, include_guard: S) -> Builder {
        self.config.include_guard = Some(String::from(include_guard.as_ref()));
        self
    }

    #[allow(unused)]
    pub fn with_autogen_warning<S: AsRef<str>>(mut self, autogen_warning: S) -> Builder {
        self.config.autogen_warning = Some(String::from(autogen_warning.as_ref()));
        self
    }

    #[allow(unused)]
    pub fn with_include_version(mut self, include_version: bool) -> Builder {
        self.config.include_version = include_version;
        self
    }

    #[allow(unused)]
    pub fn with_namespace<S: AsRef<str>>(mut self, namespace: S) -> Builder {
        self.config.namespace = Some(String::from(namespace.as_ref()));
        self
    }

    #[allow(unused)]
    pub fn with_namespaces<S: AsRef<str>>(mut self, namespaces: &[S]) -> Builder {
        self.config.namespaces = Some(
            namespaces
                .iter()
                .map(|x| String::from(x.as_ref()))
                .collect(),
        );
        self
    }

    #[allow(unused)]
    pub fn with_braces(mut self, braces: Braces) -> Builder {
        self.config.braces = braces;
        self
    }

    #[allow(unused)]
    pub fn with_line_length(mut self, line_length: usize) -> Builder {
        self.config.line_length = line_length;
        self
    }

    #[allow(unused)]
    pub fn with_tab_width(mut self, tab_width: usize) -> Builder {
        self.config.tab_width = tab_width;
        self
    }

    #[allow(unused)]
    pub fn with_language(mut self, language: Language) -> Builder {
        self.config.language = language;
        self
    }

    #[allow(unused)]
    pub fn with_parse_deps(mut self, parse_deps: bool) -> Builder {
        self.config.parse.parse_deps = parse_deps;
        self
    }

    #[allow(unused)]
    pub fn with_parse_include<S: AsRef<str>>(mut self, include: &[S]) -> Builder {
        self.config.parse.include =
            Some(include.iter().map(|x| String::from(x.as_ref())).collect());
        self
    }

    #[allow(unused)]
    pub fn with_parse_exclude<S: AsRef<str>>(mut self, exclude: &[S]) -> Builder {
        self.config.parse.exclude = exclude.iter().map(|x| String::from(x.as_ref())).collect();
        self
    }

    #[allow(unused)]
    pub fn with_parse_expand<S: AsRef<str>>(mut self, expand: &[S]) -> Builder {
        self.config.parse.expand = expand.iter().map(|x| String::from(x.as_ref())).collect();
        self
    }

    #[allow(unused)]
    pub fn with_documentation(mut self, documentation: bool) -> Builder {
        self.config.documentation = documentation;
        self
    }

    #[allow(unused)]
    pub fn with_target_os_define(mut self, platform: &str, preprocessor_define: &str) -> Builder {
        self.config.defines.insert(
            format!("target_os = {}", platform),
            preprocessor_define.to_owned(),
        );
        self
    }

    #[allow(unused)]
    pub fn with_define(mut self, key: &str, value: &str, preprocessor_define: &str) -> Builder {
        self.config.defines.insert(
            format!("{} = {}", key, value),
            preprocessor_define.to_owned(),
        );
        self
    }

    #[allow(unused)]
    pub fn with_config(mut self, config: Config) -> Builder {
        self.config = config;
        self
    }

    #[allow(unused)]
    pub fn with_std_types(mut self, std_types: bool) -> Builder {
        self.std_types = std_types;
        self
    }

    #[allow(unused)]
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
    pub fn with_crate_and_name<P: AsRef<path::Path>, S: AsRef<str>>(
        mut self,
        lib_dir: P,
        binding_lib_name: S,
    ) -> Builder {
        debug_assert!(self.lib.is_none());
        debug_assert!(self.lib_cargo.is_none());
        self.lib = Some((
            path::PathBuf::from(lib_dir.as_ref()),
            Some(String::from(binding_lib_name.as_ref())),
        ));
        self
    }

    #[allow(unused)]
    pub(crate) fn with_cargo(mut self, lib: Cargo) -> Builder {
        debug_assert!(self.lib.is_none());
        debug_assert!(self.lib_cargo.is_none());
        self.lib_cargo = Some(lib);
        self
    }

    pub fn generate(self) -> Result<Bindings, Error> {
        let mut result = Parse::new();

        if self.std_types {
            result.add_std_types();
        }

        for x in &self.srcs {
            result.extend_with(&parser::parse_src(x)?);
        }

        if let Some((lib_dir, binding_lib_name)) = self.lib.clone() {
            let cargo = if let Some(binding_lib_name) = binding_lib_name {
                Cargo::load(
                    &lib_dir,
                    Some(&binding_lib_name),
                    self.config.parse.parse_deps,
                )?
            } else {
                Cargo::load(&lib_dir, None, self.config.parse.parse_deps)?
            };

            result.extend_with(&parser::parse_lib(
                cargo,
                self.config.parse.parse_deps,
                &self.config.parse.include,
                &self.config.parse.exclude,
                &self.config.parse.expand,
            )?);
        } else if let Some(cargo) = self.lib_cargo.clone() {
            result.extend_with(&parser::parse_lib(
                cargo,
                self.config.parse.parse_deps,
                &self.config.parse.include,
                &self.config.parse.exclude,
                &self.config.parse.expand,
            )?);
        }

        Library::new(
            self.config,
            result.constants,
            result.globals,
            result.enums,
            result.structs,
            result.unions,
            result.opaque_items,
            result.typedefs,
            result.functions,
        ).generate()
    }
}
