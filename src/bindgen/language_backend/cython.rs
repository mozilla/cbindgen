use crate::bindgen::language_backend::{LanguageBackend, NamespaceOperation};
use crate::bindgen::writer::SourceWriter;
use crate::bindgen::Config;
use std::io::Write;

pub struct CythonLanguageBackend {
    config: Config,
}

impl CythonLanguageBackend {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl LanguageBackend for CythonLanguageBackend {
    fn write_headers<W: Write>(&self, out: &mut SourceWriter<W>) {
        if let Some(ref f) = self.config.header {
            out.new_line_if_not_start();
            write!(out, "{}", f);
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
            && (self.config.cython.cimports.is_empty())
            && self.config.after_includes.is_none()
        {
            return;
        }

        out.new_line_if_not_start();

        if !self.config.no_includes {
            out.write("from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t");
            out.new_line();
            out.write("from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t");
            out.new_line();
            out.write("cdef extern from *");
            out.open_brace();
            out.write("ctypedef bint bool");
            out.new_line();
            out.write("ctypedef struct va_list");
            out.new_line();
            out.close_brace(false);
        }

        for (module, names) in &self.config.cython.cimports {
            write!(out, "from {} cimport {}", module, names.join(", "));
            out.new_line();
        }

        if let Some(ref line) = self.config.after_includes {
            write!(out, "{}", line);
            out.new_line();
        }
    }

    fn open_close_namespaces<W: Write>(&self, op: NamespaceOperation, out: &mut SourceWriter<W>) {
        if op == NamespaceOperation::Open {
            out.new_line();
            let header = self.config.cython.header.as_deref().unwrap_or("*");
            write!(out, "cdef extern from {}", header);
            out.open_brace();
        } else {
            out.close_brace(false);
        }
    }

    fn write_footers<W: Write>(&self, _out: &mut SourceWriter<W>) {}
}
