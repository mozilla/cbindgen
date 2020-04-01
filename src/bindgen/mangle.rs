/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::bindgen::ir::{Path, Type};

pub fn mangle_path(path: &Path, generic_values: &[Type]) -> Path {
    Path::new(mangle_name(path.name(), generic_values))
}

pub fn mangle_name(name: &str, generic_values: &[Type]) -> String {
    Mangler::new(name, generic_values, /* last = */ true).mangle()
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
}

impl<'a> Mangler<'a> {
    fn new(input: &'a str, generic_values: &'a [Type], last: bool, mangle_separator: &Option<String>) -> Self {
        Self {
            input,
            generic_values,
            output: String::new(),
            last,
            mangle_separator: &Option<String>,
        }
    }

    fn mangle(mut self) -> String {
        self.mangle_internal();
        self.output
    }

    fn push(&mut self, id: Separator) {
        let separator = match mangle_separator {
            Some(s) => s.to_owned(),
            None => "_".to_string()
        };
        let count = id as usize;
        self.output.extend(std::iter::repeat(separator).take(count));
    }

    fn append_mangled_type(&mut self, ty: &Type, last: bool) {
        match *ty {
            Type::Path(ref generic) => {
                let sub_path =
                    Mangler::new(generic.export_name(), generic.generics(), last).mangle();
                self.output.push_str(&sub_path);
            }
            Type::Primitive(ref primitive) => {
                self.output.push_str(primitive.to_repr_rust());
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

fn concat_separators(separator: &str, number: u8) -> String {
    let mut result: String = "".to_string();
    for _ in 0..number {
        result += separator;
    }
    result
}

fn concat_separators(separator: &str, number: u8) -> String {
    let mut result: String = "".to_string();
    for _ in 0..number {
        result += separator;
    }
    result
}

#[test]
fn generics() {
    use crate::bindgen::ir::{GenericPath, PrimitiveType};

    fn float() -> Type {
        Type::Primitive(PrimitiveType::Float)
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
        mangle_path(&Path::new("Foo"), &vec![float()], &None),
        Path::new("Foo_f32")
    );

    // Foo<Bar<f32>> => Foo_Bar_f32
    assert_eq!(
        mangle_path(&Path::new("Foo"), &vec![generic_path("Bar", &[float()])], &None),
        Path::new("Foo_Bar_f32")
    );

    // Foo<Bar> => Foo_Bar
    assert_eq!(
        mangle_path(&Path::new("Foo"), &[path("Bar")], &None),
        Path::new("Foo_Bar")
    );

    // Foo<Bar<T>> => Foo_Bar_T
    assert_eq!(
        mangle_path(&Path::new("Foo"), &[generic_path("Bar", &[path("T")])], &None),
        Path::new("Foo_Bar_T")
    );

    // Foo<Bar<T>, E> => Foo_Bar_T_____E
    assert_eq!(
        mangle_path(
            &Path::new("Foo"),
            &[generic_path("Bar", &[path("T")]), path("E")],
            &None,
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
            &None,
        ),
        Path::new("Foo_Bar_T_____Bar_E")
    );
}
