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
}
