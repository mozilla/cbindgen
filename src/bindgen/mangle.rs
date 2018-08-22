/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use bindgen::ir::{Path, Type};

pub fn mangle_path(path: &Path, generic_values: &[Type]) -> Path {
    internal_mangle_path(path, generic_values, true)
}

pub fn mangle_name(name: &str, generic_values: &[Type]) -> String {
    internal_mangle_name(name, generic_values, true)
}

fn internal_mangle_path(path: &Path, generic_values: &[Type], last_in_parent: bool) -> Path {
    let name = path.name();
    let mangled_name = internal_mangle_name(name, generic_values, last_in_parent);
    Path::new(mangled_name)
}

fn internal_mangle_name(name: &str, generic_values: &[Type], last_in_parent: bool) -> String {
    if generic_values.is_empty() {
        return name.to_owned();
    }

    let mut mangled = name.to_owned();

    mangled.push_str("_"); // <
    for (i, ty) in generic_values.iter().enumerate() {
        if i != 0 {
            mangled.push_str("__"); // ,
        }

        let is_last = i == generic_values.len() - 1;
        match ty {
            &Type::Path(ref generic) => {
                mangled.push_str(&internal_mangle_name(
                    generic.export_name(),
                    generic.generics(),
                    last_in_parent && is_last,
                ));
            }
            &Type::Primitive(ref primitive) => {
                mangled.push_str(primitive.to_repr_rust());
            }
            &Type::ConstPtr(..) | &Type::Ptr(..) | &Type::Array(..) | &Type::FuncPtr(..) => {
                unimplemented!()
            }
        }

        // Skip writing the trailing '>' mangling when possible
        if is_last && !last_in_parent {
            mangled.push_str("___"); // >
        }
    }

    mangled
}

#[test]
fn generics() {
    use bindgen::ir::{Generic, PrimitiveType};

    fn float() -> Type {
        Type::Primitive(PrimitiveType::Float)
    }

    fn path(path: &str) -> Type {
        generic_path(path, &vec![])
    }

    fn generic_path(path: &str, generics: &[Type]) -> Type {
        let path = Path::new(path);
        let generic = Generic::new(path, generics.to_owned());
        Type::Path(generic)
    }

    // Foo<f32> => Foo_f32
    assert_eq!(
        mangle_path(&Path::new("Foo"), &vec![float()]),
        Path::new("Foo_f32")
    );

    // Foo<Bar<f32>> => Foo_Bar_f32
    assert_eq!(
        mangle_path(&Path::new("Foo"), &vec![generic_path("Bar", &[float()])]),
        Path::new("Foo_Bar_f32")
    );

    // Foo<Bar> => Foo_Bar
    assert_eq!(
        mangle_path(&Path::new("Foo"), &[path("Bar")]),
        Path::new("Foo_Bar")
    );

    // Foo<Bar<T>> => Foo_Bar_T
    assert_eq!(
        mangle_path(&Path::new("Foo"), &[generic_path("Bar", &[path("T")])]),
        Path::new("Foo_Bar_T")
    );

    // Foo<Bar<T>, E> => Foo_Bar_T_____E
    assert_eq!(
        mangle_path(
            &Path::new("Foo"),
            &[generic_path("Bar", &[path("T")]), path("E")]
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
            ]
        ),
        Path::new("Foo_Bar_T_____Bar_E")
    );
}

#[test]
#[should_panic(expected = "name 'foo_bar' contains an underscore")]
fn invalid() {
    use bindgen::ir::PrimitiveType;

    // foo_bar<u32> => foo_bar_f32
    let t = Type::Primitive(PrimitiveType::UInt32);
    assert_eq!(
        mangle_path(&Path::new("foo_bar"), &vec![t]),
        Path::new("foo_bar_u32")
    );
}
