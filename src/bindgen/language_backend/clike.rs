use crate::bindgen::language_backend::{LanguageBackend, NamespaceOperation};
use crate::bindgen::writer::SourceWriter;
use crate::bindgen::{Config, Language};
use std::io::Write;

pub struct CLikeLanguageBackend {
    config: Config,
}

impl CLikeLanguageBackend {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl LanguageBackend for CLikeLanguageBackend {
    fn write_headers<W: Write>(&self, out: &mut SourceWriter<W>) {
        if let Some(ref f) = self.config.header {
            out.new_line_if_not_start();
            write!(out, "{}", f);
            out.new_line();
        }
        if let Some(f) = self.config.include_guard() {
            out.new_line_if_not_start();
            write!(out, "#ifndef {}", f);
            out.new_line();
            write!(out, "#define {}", f);
            out.new_line();
        }
        if self.config.pragma_once {
            out.new_line_if_not_start();
            write!(out, "#pragma once");
            out.new_line();
        }
        if self.config.include_version {
            out.new_line_if_not_start();
            write!(
                out,
                "/* Generated with cbindgen:{} */",
                crate::bindgen::config::VERSION
            );
            out.new_line();
        }
        if let Some(ref f) = self.config.autogen_warning {
            out.new_line_if_not_start();
            write!(out, "{}", f);
            out.new_line();
        }

        if self.config.no_includes
            && self.config.sys_includes().is_empty()
            && self.config.includes().is_empty()
            && self.config.after_includes.is_none()
        {
            return;
        }

        out.new_line_if_not_start();

        if !self.config.no_includes {
            match self.config.language {
                Language::C => {
                    out.write("#include <stdarg.h>");
                    out.new_line();
                    out.write("#include <stdbool.h>");
                    out.new_line();
                    if self.config.usize_is_size_t {
                        out.write("#include <stddef.h>");
                        out.new_line();
                    }
                    out.write("#include <stdint.h>");
                    out.new_line();
                    out.write("#include <stdlib.h>");
                    out.new_line();
                }
                Language::Cxx => {
                    out.write("#include <cstdarg>");
                    out.new_line();
                    if self.config.usize_is_size_t {
                        out.write("#include <cstddef>");
                        out.new_line();
                    }
                    out.write("#include <cstdint>");
                    out.new_line();
                    out.write("#include <cstdlib>");
                    out.new_line();
                    out.write("#include <ostream>");
                    out.new_line();
                    out.write("#include <new>");
                    out.new_line();
                    if self.config.enumeration.cast_assert_name.is_none()
                        && (self.config.enumeration.derive_mut_casts
                            || self.config.enumeration.derive_const_casts)
                    {
                        out.write("#include <cassert>");
                        out.new_line();
                    }
                }
                _ => {}
            }
        }

        for include in self.config.sys_includes() {
            write!(out, "#include <{}>", include);
            out.new_line();
        }

        for include in self.config.includes() {
            write!(out, "#include \"{}\"", include);
            out.new_line();
        }

        if let Some(ref line) = self.config.after_includes {
            write!(out, "{}", line);
            out.new_line();
        }
    }

    fn open_close_namespaces<W: Write>(&self, op: NamespaceOperation, out: &mut SourceWriter<W>) {
        let mut namespaces =
            if self.config.language != Language::Cxx && !self.config.cpp_compatible_c() {
                vec![]
            } else {
                let mut ret = vec![];
                if let Some(ref namespace) = self.config.namespace {
                    ret.push(&**namespace);
                }
                if let Some(ref namespaces) = self.config.namespaces {
                    for namespace in namespaces {
                        ret.push(&**namespace);
                    }
                }
                ret
            };

        if namespaces.is_empty() {
            return;
        }

        if op == NamespaceOperation::Close {
            namespaces.reverse();
        }

        if self.config.cpp_compatible_c() {
            out.new_line_if_not_start();
            out.write("#ifdef __cplusplus");
        }

        for namespace in namespaces {
            out.new_line();
            match op {
                NamespaceOperation::Open => write!(out, "namespace {} {{", namespace),
                NamespaceOperation::Close => write!(out, "}} // namespace {}", namespace),
            }
        }

        out.new_line();
        if self.config.cpp_compatible_c() {
            out.write("#endif // __cplusplus");
            out.new_line();
        }
    }

    fn write_footers<W: Write>(&self, out: &mut SourceWriter<W>) {
        if let Some(f) = self.config.include_guard() {
            out.new_line_if_not_start();
            if self.config.language == Language::C {
                write!(out, "#endif /* {} */", f);
            } else {
                write!(out, "#endif // {}", f);
            }
            out.new_line();
        }
    }
}
