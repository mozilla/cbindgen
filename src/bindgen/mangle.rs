/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::bindgen::ir::{Path, Type};
use convert_case::{Case, Casing};
use std::borrow::Cow;
use std::ops::Deref;

pub fn mangle_path(
    path: &Path,
    generic_values: &[Type],
    remove_underscores: bool,
    pascal_case_primitives: bool,
) -> Path {
    Path::new(mangle_name(
        path.name(),
        generic_values,
        remove_underscores,
        pascal_case_primitives,
    ))
}

pub fn mangle_name(
    name: &str,
    generic_values: &[Type],
    remove_underscores: bool,
    pascal_case_primitives: bool,
) -> String {
    Mangler::new(
        name,
        generic_values,
        /* last = */ true,
        remove_underscores,
        pascal_case_primitives,
    )
    .mangle()
}

enum Separator {
    OpeningAngleBracket = 1,
    Comma,
    ClosingAngleBracket,
    BeginMutPtr,
    BeginConstPtr,
}

struct Mangler<'a> {
    input: &'a str,
    generic_values: &'a [Type],
    output: String,
    last: bool,
    remove_underscores: bool,
    pascal_case_primitives: bool,
}

impl<'a> Mangler<'a> {
    fn new(
        input: &'a str,
        generic_values: &'a [Type],
        last: bool,
        remove_underscores: bool,
        pascal_case_primitives: bool,
    ) -> Self {
        Self {
            input,
            generic_values,
            output: String::new(),
            last,
            remove_underscores,
            pascal_case_primitives,
        }
    }

    fn mangle(mut self) -> String {
        self.mangle_internal();
        self.output
    }

    fn push(&mut self, id: Separator) {
        let count = id as usize;
        let separator = if self.remove_underscores { "" } else { "_" };
        self.output.extend(std::iter::repeat(separator).take(count));
    }

    fn append_mangled_type(&mut self, ty: &Type, last: bool) {
        match *ty {
            Type::Path(ref generic) => {
                let sub_path = Mangler::new(
                    generic.export_name(),
                    generic.generics(),
                    last,
                    self.remove_underscores,
                    self.pascal_case_primitives,
                )
                .mangle();
                self.output.push_str(&sub_path);
            }
            Type::Primitive(ref primitive) => {
                let mut primitive_string = Cow::Borrowed(primitive.to_repr_rust());
                if self.pascal_case_primitives {
                    primitive_string = Cow::Owned(primitive_string.to_case(Case::Pascal));
                }
                self.output.push_str(primitive_string.deref());
            }
            Type::Ptr {
                ref ty, is_const, ..
            } => {
                self.push(if is_const {
                    Separator::BeginConstPtr
                } else {
                    Separator::BeginMutPtr
                });
                self.append_mangled_type(&**ty, last);
            }
            Type::Array(..) | Type::FuncPtr(..) => {
                unimplemented!(
                    "Unable to mangle generic parameter {:?} for '{}'",
                    ty,
                    self.input
                );
            }
        }
    }

    fn mangle_internal(&mut self) {
        debug_assert!(self.output.is_empty());
        self.output = self.input.to_owned();
        if self.generic_values.is_empty() {
            return;
        }

        self.push(Separator::OpeningAngleBracket);
        for (i, ty) in self.generic_values.iter().enumerate() {
            if i != 0 {
                self.push(Separator::Comma);
            }
            let last = self.last && i == self.generic_values.len() - 1;
            self.append_mangled_type(ty, last);
        }

        // Skip writing the trailing '>' mangling when possible
        if !self.last {
            self.push(Separator::ClosingAngleBracket)
        }
    }
}

#[test]
fn generics() {
    use crate::bindgen::ir::{GenericPath, PrimitiveType};

    fn float() -> Type {
        Type::Primitive(PrimitiveType::Float)
    }

    fn c_char() -> Type {
        Type::Primitive(PrimitiveType::Char)
    }

    fn path(path: &str) -> Type {
        generic_path(path, &vec![])
    }

    fn generic_path(path: &str, generics: &[Type]) -> Type {
        let path = Path::new(path);
        let generic_path = GenericPath::new(path, generics.to_owned());
        Type::Path(generic_path)
    }

    // Foo<f32> => Foo_f32
    assert_eq!(
        mangle_path(&Path::new("Foo"), &vec![float()], false, false),
        Path::new("Foo_f32")
    );

    // Foo<Bar<f32>> => Foo_Bar_f32
    assert_eq!(
        mangle_path(
            &Path::new("Foo"),
            &vec![generic_path("Bar", &[float()])],
            false,
            false,
        ),
        Path::new("Foo_Bar_f32")
    );

    // Foo<Bar> => Foo_Bar
    assert_eq!(
        mangle_path(&Path::new("Foo"), &[path("Bar")], false, false),
        Path::new("Foo_Bar")
    );

    // Foo<Bar> => FooBar
    assert_eq!(
        mangle_path(&Path::new("Foo"), &[path("Bar")], true, false),
        Path::new("FooBar")
    );

    // Foo<Bar<f32>> => FooBarF32
    assert_eq!(
        mangle_path(
            &Path::new("Foo"),
            &vec![generic_path("Bar", &[float()])],
            true,
            true,
        ),
        Path::new("FooBarF32")
    );

    // Foo<Bar<c_char>> => FooBarCChar
    assert_eq!(
        mangle_path(
            &Path::new("Foo"),
            &vec![generic_path("Bar", &[c_char()])],
            true,
            true,
        ),
        Path::new("FooBarCChar")
    );

    // Foo<Bar<T>> => Foo_Bar_T
    assert_eq!(
        mangle_path(
            &Path::new("Foo"),
            &[generic_path("Bar", &[path("T")])],
            false,
            false,
        ),
        Path::new("Foo_Bar_T")
    );

    // Foo<Bar<T>, E> => Foo_Bar_T_____E
    assert_eq!(
        mangle_path(
            &Path::new("Foo"),
            &[generic_path("Bar", &[path("T")]), path("E")],
            false,
            false,
        ),
        Path::new("Foo_Bar_T_____E")
    );

    // Foo<Bar<T>, Bar<E>> => Foo_Bar_T_____Bar_E
    assert_eq!(
        mangle_path(
            &Path::new("Foo"),
            &[
                generic_path("Bar", &[path("T")]),
                generic_path("Bar", &[path("E")]),
            ],
            false,
            false,
        ),
        Path::new("Foo_Bar_T_____Bar_E")
    );

    // Foo<Bar<T>, E> => FooBarTE
    assert_eq!(
        mangle_path(
            &Path::new("Foo"),
            &[generic_path("Bar", &[path("T")]), path("E")],
            true,
            false,
        ),
        Path::new("FooBarTE")
    );

    // Foo<Bar<T>, Bar<E>> => FooBarTBarE
    assert_eq!(
        mangle_path(
            &Path::new("Foo"),
            &[
                generic_path("Bar", &[path("T")]),
                generic_path("Bar", &[path("E")]),
            ],
            true,
            false,
        ),
        Path::new("FooBarTBarE")
    );
}
