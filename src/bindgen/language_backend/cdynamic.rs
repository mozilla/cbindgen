use std::{borrow::Cow, cell::RefCell, collections::HashMap, io::Write};

use crate::{
    bindgen::Config,
    bindgen::RenameRule,
    bindgen::{
        cdecl,
        ir::{Cfg, ConditionWrite as _, ToCondition as _, Type},
        rename::IdentifierType,
        writer::SourceWriter,
    },
};

use super::{CLikeLanguageBackend, LanguageBackend};

/* ---------------------------------------- Configuration --------------------------------------- */

/// # TODO
///
/// Should we elevate dynamic generation as part of [`crate::Language`] variant, and embed dynamic
/// configuration in it, since it inherits most of generation specifics from existing
/// implementation?
#[non_exhaustive]
#[derive(Default, Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct CDynamicBindingConfig {
    /// Default loader function name is `<api_struct_name>_load(..)`. Specifying this with will
    /// change the default name of generated loader method.
    ///
    /// # NOTE
    ///
    /// Since the loader method just inline static function which is conditionally activated by
    /// preprocessor macro definition, it's just okay to rename the generated
    pub loader_function_name_override: Option<String>,

    /// Symbol renaming lookup table. Symbols can be renamed freely, since the renaming symbols here
    /// is just name of identifiers which will be bound into the original symbol name.
    pub symbol_renames: HashMap<String, String>,

    /// Symbol rename rules which is applied as fallback, which don't have any specific
    /// `symbol_renames` rule
    pub symbol_rename_all: RenameRule,
}

impl CDynamicBindingConfig {
    fn rename_struct_field<'a>(&'a self, name: &'a str) -> Cow<str> {
        self.symbol_renames
            .get(name)
            .map(|x| Cow::Borrowed(x.as_str()))
            .unwrap_or_else(|| {
                self.symbol_rename_all
                    .apply(name, IdentifierType::StructMember)
            })
    }
}

/* -------------------------------------- Language Backend -------------------------------------- */

/// Differentiate loader macro name, since the user may not want to see other modules' loader
/// methods.
const DYN_TRAILER_TEMPLATE: &str = r##"
#ifdef INCLUDE_CBINDGEN_LOADER_{{API_STRUCT_NAME}}
#  ifndef CBINDGEN_LOADER_LOOKUP_INTERFACE_DEFINED
#  define CBINDGEN_LOADER_LOOKUP_INTERFACE_DEFINED
struct CBindgenSymbolLookupIface {
    void* module;
    void* (*find_symbol)(void* module, const char* symbol_name);
    void* (*opt_find_function)(void* module, const char* function_name);
};
#  endif

#  ifndef CBINDGEN_LOADER_{{API_STRUCT_NAME}}_DEFINED
#  define CBINDGEN_LOADER_{{API_STRUCT_NAME}}_DEFINED

inline int {{API_LOADER_FUNCTION_NAME}} (
    struct {{API_STRUCT_NAME}}* api,
    struct CBindgenSymbolLookupIface* module
) {
    int notfound = 0;
    void* mod = module->module;
    void* (*fsym)(void*, const char*) = module->find_symbol;
    void* (*ffunc)(void*, const char*) = module->opt_find_function;
    
    if (!ffunc) {
        ffunc = module->find_symbol;
    }
    
    {
{{API_LOADER_FUNCTION_BODY}}
    }
    
    return notfound;
}

#  endif
#endif
"##;

pub struct CDynamicBindingBackend {
    struct_name: String,
    config: CDynamicBindingConfig,
}

impl CDynamicBindingBackend {
    /// Create new backend for generating C dynamic
    #[allow(unused)] // Exposed for library users
    pub fn new<S>(api_struct_name: S, config: CDynamicBindingConfig) -> Self
    where
        S: Into<String>,
    {
        fn verify_ident(s: &str) {
            assert!(!s.is_empty() && s.chars().all(|x| x.is_ascii_alphanumeric() || x == '_'));
        }

        let ident: String = api_struct_name.into();
        verify_ident(&ident);

        if let Some(ref name) = config.loader_function_name_override {
            verify_ident(name);
        }

        Self {
            struct_name: ident,
            config,
        }
    }

    fn generate_api_struct<W: Write>(
        &mut self,
        out: &mut SourceWriter<W>,
        b: &crate::Bindings,
        inner: &mut CLikeLanguageBackend,
    ) {
        out.new_line();

        out.write_fmt(format_args!("struct {}", self.struct_name));
        out.open_brace();

        // This refcell is just workaround to avoid borrow checker complaint.
        let inner = RefCell::new(inner);

        out.write_vertical_source_list(
            self,
            &b.globals,
            crate::bindgen::writer::ListType::Cap(";"),
            |this, out, item| {
                Self::run_in_cond(out, &b.config, &item.cfg, |out| {
                    let ty = wrap_in_pointer(&item.ty, !item.mutable);

                    // TODO: support annotation?
                    inner
                        .borrow_mut()
                        .write_documentation(out, &item.documentation);

                    let ident = this
                        .config
                        .rename_struct_field(&item.export_name)
                        .into_owned();

                    cdecl::write_field(this, out, &ty, &ident, &b.config);
                });
            },
        );

        if !b.globals.is_empty() {
            out.new_line();
        }

        out.write_vertical_source_list(
            self,
            &b.functions,
            crate::bindgen::writer::ListType::Cap(";"),
            |this, out, item| {
                Self::run_in_cond(out, &b.config, &item.cfg, |out| {
                    let ty = make_func_ptr(item);

                    inner
                        .borrow_mut()
                        .write_documentation(out, &item.documentation);

                    let ident = this
                        .config
                        .rename_struct_field(item.path.name())
                        .into_owned();

                    cdecl::write_field(this, out, &ty, &ident, &b.config);
                });
            },
        );

        out.close_brace(true);

        out.new_line();
    }

    fn generate_loader_function_body<W: Write>(
        &mut self,
        out: &mut SourceWriter<W>,
        b: &crate::Bindings,
        _inner: &mut CLikeLanguageBackend,
    ) {
        let mut body = Vec::<u8>::with_capacity(256);
        let mut body_writer = SourceWriter::new(&mut body, b);
        body_writer.push_set_spaces(8);

        // api, notfound, mod, ffunc, fsym
        body_writer.write_vertical_source_list(
            self,
            &b.globals,
            crate::bindgen::writer::ListType::Cap(";"),
            |this, out, item| {
                Self::run_in_cond(out, &b.config, &item.cfg, |out| {
                    // Generates `api->NAME = (PTR_TYPE)fsym("SYMBOL_NAME");`
                    let ident = this
                        .config
                        .rename_struct_field(&item.export_name)
                        .into_owned();

                    out.write_fmt(format_args!("api->{} = (", ident));

                    let ty = wrap_in_pointer(&item.ty, !item.mutable);
                    cdecl::write_type(this, out, &ty, &b.config);

                    out.write_fmt(format_args!(")fsym(mod, \"{}\");", ident));
                })
            },
        );

        if !b.globals.is_empty() {
            body_writer.new_line();
            body_writer.new_line();
        }

        body_writer.write_vertical_source_list(
            self,
            &b.functions,
            crate::bindgen::writer::ListType::Cap(";"),
            |this, out, item| {
                // Generates `api->NAME = (PTR_TYPE)ffunc("SYMBOL_NAME");`
                Self::run_in_cond(out, &b.config, &item.cfg, |out| {
                    let ident = this
                        .config
                        .rename_struct_field(item.path.name())
                        .into_owned();

                    out.write_fmt(format_args!("api->{} = (", ident));

                    let ty = make_func_ptr(item);
                    cdecl::write_type(this, out, &ty, &b.config);

                    out.write_fmt(format_args!(")fsym(mod, \"{}\");", ident));
                })
            },
        );

        if !b.functions.is_empty() {
            body_writer.new_line();
            body_writer.new_line();
        }

        // Write verifier
        body_writer.write_vertical_source_list(
            self,
            &b.functions
                .iter()
                .map(|x| (x.path.name(), &x.cfg))
                .chain(b.globals.iter().map(|x| (x.export_name.as_str(), &x.cfg)))
                .collect::<Vec<_>>(),
            crate::bindgen::writer::ListType::Cap(";"),
            |this, out, (ident, cfg)| {
                let ident = this.config.rename_struct_field(ident);
                Self::run_in_cond(out, &b.config, cfg, |out| {
                    out.write_fmt(format_args!("notfound += (int)!api->{}", ident));
                })
            },
        );

        drop(body_writer);

        let loader_func_name = self
            .config
            .loader_function_name_override
            .clone()
            .unwrap_or_else(|| format!("{}_load", &self.struct_name));

        let payload = DYN_TRAILER_TEMPLATE
            .replace("{{API_STRUCT_NAME}}", &self.struct_name)
            .replace("{{API_LOADER_FUNCTION_NAME}}", &loader_func_name)
            .replace(
                "{{API_LOADER_FUNCTION_BODY}}",
                std::str::from_utf8(&body).unwrap(),
            );

        write!(out, "{payload}");
    }

    fn run_in_cond<W: Write>(
        out: &mut SourceWriter<W>,
        config: &Config,
        cfg: &Option<Cfg>,
        f: impl FnOnce(&mut SourceWriter<W>),
    ) {
        let condition = cfg.to_condition(config);
        condition.write_before(config, out);

        f(out);

        condition.write_after(config, out);
        if condition.is_some() {
            out.new_line();
        }
    }
}

impl LanguageBackend for CDynamicBindingBackend {
    fn write_bindings<W: std::io::prelude::Write>(
        &mut self,
        out: &mut SourceWriter<W>,
        b: &crate::Bindings,
    ) where
        Self: Sized,
    {
        if b.config.language != crate::bindgen::Language::C {
            panic!("This backend only supports C language generation")
        }

        let mut inner = CLikeLanguageBackend::new(&b.config);

        inner.write_headers(out);
        inner.open_namespaces(out);
        inner.write_primitive_constants(out, b);
        inner.write_non_primitive_constants(out, b);

        // These symbols are irrelevant with dynamically loaded symbols; Just forward to inner
        inner.write_items(out, b);

        // What we need to deal with are just statics & functions
        self.generate_api_struct(out, b, &mut inner);
        self.generate_loader_function_body(out, b, &mut inner);

        inner.close_namespaces(out);
        inner.write_footers(out);
        inner.write_trailer(out, b);
    }

    fn write_headers<W: std::io::prelude::Write>(&mut self, _: &mut SourceWriter<W>) {
        unimplemented!()
    }

    fn open_namespaces<W: std::io::prelude::Write>(&mut self, _: &mut SourceWriter<W>) {
        unimplemented!()
    }

    fn close_namespaces<W: std::io::prelude::Write>(&mut self, _: &mut SourceWriter<W>) {
        unimplemented!()
    }

    fn write_footers<W: std::io::prelude::Write>(&mut self, _: &mut SourceWriter<W>) {
        unimplemented!()
    }

    fn write_enum<W: std::io::prelude::Write>(
        &mut self,
        _out: &mut SourceWriter<W>,
        _e: &crate::bindgen::ir::Enum,
    ) {
        unimplemented!()
    }

    fn write_struct<W: std::io::prelude::Write>(
        &mut self,
        _out: &mut SourceWriter<W>,
        _s: &crate::bindgen::ir::Struct,
    ) {
        unimplemented!()
    }

    fn write_union<W: std::io::prelude::Write>(
        &mut self,
        _out: &mut SourceWriter<W>,
        _u: &crate::bindgen::ir::Union,
    ) {
        unimplemented!()
    }

    fn write_opaque_item<W: std::io::prelude::Write>(
        &mut self,
        _out: &mut SourceWriter<W>,
        _o: &crate::bindgen::ir::OpaqueItem,
    ) {
        unimplemented!()
    }

    fn write_type_def<W: std::io::prelude::Write>(
        &mut self,
        _out: &mut SourceWriter<W>,
        _t: &crate::bindgen::ir::Typedef,
    ) {
        unimplemented!()
    }

    fn write_static<W: std::io::prelude::Write>(
        &mut self,
        _: &mut SourceWriter<W>,
        _: &crate::bindgen::ir::Static,
    ) {
        unimplemented!("This must be specialised")
    }

    fn write_function<W: std::io::prelude::Write>(
        &mut self,
        _out: &mut SourceWriter<W>,
        _f: &crate::bindgen::ir::Function,
    ) {
        unimplemented!("This must be specialised")
    }

    fn write_type<W: std::io::prelude::Write>(
        &mut self,
        _out: &mut SourceWriter<W>,
        _t: &crate::bindgen::ir::Type,
    ) {
        unimplemented!()
    }

    fn write_documentation<W: std::io::prelude::Write>(
        &mut self,
        _out: &mut SourceWriter<W>,
        _d: &crate::bindgen::ir::Documentation,
    ) {
        unimplemented!()
    }

    fn write_literal<W: std::io::prelude::Write>(
        &mut self,
        _out: &mut SourceWriter<W>,
        _l: &crate::bindgen::ir::Literal,
    ) {
        unimplemented!()
    }
}

fn wrap_in_pointer(ty: &crate::bindgen::ir::Type, constness: bool) -> crate::bindgen::ir::Type {
    crate::bindgen::ir::Type::Ptr {
        ty: Box::new(ty.clone()),
        is_const: constness,
        is_nullable: false,
        is_ref: false,
    }
}

fn make_func_ptr(item: &crate::bindgen::ir::Function) -> crate::bindgen::ir::Type {
    Type::FuncPtr {
        ret: Box::new(item.ret.to_owned()),
        args: item.args.iter().cloned().map(|x| (x.name, x.ty)).collect(),
        is_nullable: true,
        never_return: item.never_return,
    }
}

#[cfg(test)]
#[test]
#[ignore]
fn quick_generate_result() {
    let source_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let test_files = std::fs::read_dir(source_dir.join("tests/rust")).unwrap();
    let temp_dir = source_dir.join("target/tmp-cdynamic");

    std::fs::remove_dir_all(&temp_dir).ok();
    std::fs::create_dir_all(&temp_dir).ok();

    for file in test_files.filter_map(|d| {
        d.ok()
            .map(|x| x.path())
            .filter(|x| x.is_file() && x.extension().is_some_and(|x| x.eq_ignore_ascii_case("rs")))
    }) {
        let mut config = crate::Config::default();
        let file_name = std::path::Path::new(file.file_name().unwrap());
        let out_file_name = temp_dir.join(file_name.with_extension("c"));
        config.language = crate::bindgen::Language::C;

        crate::Builder::new()
            .with_config(config)
            .with_src(&file)
            .generate()
            .unwrap()
            .write_with_backend(
                std::fs::File::options()
                    .write(true)
                    .truncate(true)
                    .create(true)
                    .open(out_file_name)
                    .unwrap(),
                &mut super::CDynamicBindingBackend::new("MyApiStruct", Default::default()),
            );
    }
}
