use crate::bindgen::ir::{
    Documentation, Enum, Function, Literal, OpaqueItem, Static, Struct, Type, Typedef, Union,
};
use crate::bindgen::writer::SourceWriter;

use std::io::Write;

mod clike;
mod cython;

pub use clike::CLikeLanguageBackend;
pub use cython::CythonLanguageBackend;

#[derive(PartialEq, Eq)]
pub enum NamespaceOperation {
    Open,
    Close,
}

pub trait LanguageBackend {
    fn write_headers<W: Write>(&self, out: &mut SourceWriter<W>);
    fn open_close_namespaces<W: Write>(&self, op: NamespaceOperation, out: &mut SourceWriter<W>);
    fn write_footers<W: Write>(&self, out: &mut SourceWriter<W>);
    fn write_enum<W: Write>(&self, out: &mut SourceWriter<W>, e: &Enum);
    fn write_struct<W: Write>(&self, out: &mut SourceWriter<W>, s: &Struct);
    fn write_union<W: Write>(&self, out: &mut SourceWriter<W>, u: &Union);
    fn write_opaque_item<W: Write>(&self, out: &mut SourceWriter<W>, o: &OpaqueItem);
    fn write_type_def<W: Write>(&self, out: &mut SourceWriter<W>, t: &Typedef);
    fn write_static<W: Write>(&self, out: &mut SourceWriter<W>, s: &Static);
    fn write_function<W: Write>(&self, out: &mut SourceWriter<W>, f: &Function);
    fn write_type<W: Write>(&self, out: &mut SourceWriter<W>, t: &Type);
    fn write_documentation<W: Write>(&self, out: &mut SourceWriter<W>, d: &Documentation);
    fn write_literal<W: Write>(&self, out: &mut SourceWriter<W>, l: &Literal);
}
