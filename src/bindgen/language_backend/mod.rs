use crate::bindgen::ir::{
    Documentation, Enum, Function, ItemContainer, Literal, OpaqueItem, Static, Struct, Type,
    Typedef, Union,
};
use crate::bindgen::writer::SourceWriter;
use crate::bindgen::Bindings;

use std::io::Write;

mod clike;
mod cython;

pub use clike::CLikeLanguageBackend;
pub use cython::CythonLanguageBackend;

pub trait LanguageBackend: Sized {
    fn write_headers<W: Write>(&mut self, out: &mut SourceWriter<W>);

    fn open_namespaces<W: Write>(&mut self, out: &mut SourceWriter<W>);
    fn close_namespaces<W: Write>(&mut self, out: &mut SourceWriter<W>);
    fn write_footers<W: Write>(&mut self, out: &mut SourceWriter<W>);
    fn write_enum<W: Write>(&mut self, out: &mut SourceWriter<W>, e: &Enum);
    fn write_struct<W: Write>(&mut self, out: &mut SourceWriter<W>, s: &Struct);
    fn write_union<W: Write>(&mut self, out: &mut SourceWriter<W>, u: &Union);
    fn write_opaque_item<W: Write>(&mut self, out: &mut SourceWriter<W>, o: &OpaqueItem);
    fn write_type_def<W: Write>(&mut self, out: &mut SourceWriter<W>, t: &Typedef);
    fn write_static<W: Write>(&mut self, out: &mut SourceWriter<W>, s: &Static);
    fn write_function<W: Write>(&mut self, out: &mut SourceWriter<W>, f: &Function);
    fn write_type<W: Write>(&mut self, out: &mut SourceWriter<W>, t: &Type);
    fn write_documentation<W: Write>(&mut self, out: &mut SourceWriter<W>, d: &Documentation);
    fn write_literal<W: Write>(&mut self, out: &mut SourceWriter<W>, l: &Literal);

    fn write_bindings<W: Write>(&mut self, out: &mut SourceWriter<W>, b: &Bindings)
    where
        Self: Sized,
    {
        self.write_headers(out);

        self.open_namespaces(out);

        self.write_primitive_constants(out, b);

        self.write_items(out, b);

        self.write_non_primitive_constants(out, b);

        self.write_globals(out, b);

        self.write_functions(out, b);

        self.close_namespaces(out);

        self.write_footers(out);

        self.write_trailer(out, b);
    }

    fn write_primitive_constants<W: Write>(&mut self, out: &mut SourceWriter<W>, b: &Bindings)
    where
        Self: Sized,
    {
        for constant in &b.constants {
            if constant.uses_only_primitive_types() {
                out.new_line_if_not_start();
                constant.write(&b.config, self, out, None);
                out.new_line();
            }
        }
    }

    fn write_items<W: Write>(&mut self, out: &mut SourceWriter<W>, b: &Bindings) {
        for item in &b.items {
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
                ItemContainer::Enum(ref x) => self.write_enum(out, x),
                ItemContainer::Struct(ref x) => self.write_struct(out, x),
                ItemContainer::Union(ref x) => self.write_union(out, x),
                ItemContainer::OpaqueItem(ref x) => self.write_opaque_item(out, x),
                ItemContainer::Typedef(ref x) => self.write_type_def(out, x),
            }
            out.new_line();
        }
    }

    fn write_non_primitive_constants<W: Write>(&mut self, out: &mut SourceWriter<W>, b: &Bindings)
    where
        Self: Sized,
    {
        for constant in &b.constants {
            if !constant.uses_only_primitive_types() {
                out.new_line_if_not_start();
                constant.write(&b.config, self, out, None);
                out.new_line();
            }
        }
    }

    fn write_globals<W: Write>(&mut self, out: &mut SourceWriter<W>, b: &Bindings) {
        self.write_globals_default(out, b)
    }

    fn write_globals_default<W: Write>(&mut self, out: &mut SourceWriter<W>, b: &Bindings) {
        for global in &b.globals {
            out.new_line_if_not_start();
            self.write_static(out, global);
            out.new_line();
        }
    }

    fn write_functions<W: Write>(&mut self, out: &mut SourceWriter<W>, b: &Bindings) {
        self.write_functions_default(out, b)
    }

    fn write_functions_default<W: Write>(&mut self, out: &mut SourceWriter<W>, b: &Bindings) {
        for function in &b.functions {
            out.new_line_if_not_start();
            self.write_function(out, function);
            out.new_line();
        }
    }

    fn write_trailer<W: Write>(&mut self, out: &mut SourceWriter<W>, b: &Bindings) {
        if let Some(ref f) = b.config.trailer {
            out.new_line_if_not_start();
            write!(out, "{}", f);
            if !f.ends_with('\n') {
                out.new_line();
            }
        }
    }
}
