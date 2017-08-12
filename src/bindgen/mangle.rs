/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use bindgen::ir::Type;

pub fn mangle_path(name: &str, generic_values: &[Type]) -> String {
    internal_mangle_path(name, generic_values, true)
}

fn internal_mangle_path(name: &str, generic_values: &[Type], last_in_parent: bool) -> String {
    assert!(!name.contains("_"));

    if generic_values.is_empty() {
        return String::from(name);
    }

    let mut out = String::from(name);

    out.push_str("_"); // <
    for (i, ty) in generic_values.iter().enumerate() {
        if i != 0 {
            out.push_str("__"); // ,
        }

        let is_last = i == generic_values.len() - 1;
        match ty {
            &Type::Path(ref path) => {
                out.push_str(&internal_mangle_path(&path.name,
                                                   &path.generics,
                                                   last_in_parent && is_last));
            }
            &Type::Primitive(ref primitive) => {
                out.push_str(primitive.to_repr_rust());
            }
            &Type::ConstPtr(..) |
            &Type::Ptr(..) |
            &Type::Array(..)|
            &Type::FuncPtr(..) => {
                unimplemented!()
            }
        }

        // Skip writing the trailing '>' mangling when possible
        if is_last && !last_in_parent {
            out.push_str("___"); // >
        }
    }

    out
}

#[test]
fn mangle_test() {
    use bindgen::ir::{GenericPath, PrimitiveType};

    fn float() -> Type {
        Type::Primitive(PrimitiveType::Float)
    }

    fn path(path: &str) -> Type {
        Type::Path(GenericPath::new(path.to_owned(), Vec::new()))
    }

    fn generic_path(path: &str, generics: &[Type]) -> Type {
        Type::Path(GenericPath::new(path.to_owned(), generics.to_owned()))
    }

    /* Foo<f32> => Foo_f32 */
    assert_eq!(mangle_path("Foo", &vec![float()]),
               "Foo_f32");

    /* Foo<Bar<f32>> => Foo_Bar_f32 */
    assert_eq!(mangle_path("Foo", &vec![generic_path("Bar", &[float()])]),
               "Foo_Bar_f32");

    /* Foo<Bar> => Foo_Bar */
    assert_eq!(mangle_path("Foo", &[path("Bar")]),
               "Foo_Bar");

    /* Foo<Bar<T>> => Foo_Bar_T */
    assert_eq!(mangle_path("Foo", &[generic_path("Bar", &[path("T")])]),
               "Foo_Bar_T");

    // /* Foo<Bar<T>, E> => Foo_Bar_T_____E */
    assert_eq!(mangle_path("Foo", &[generic_path("Bar", &[path("T")]),
                                    path("E")]),
               "Foo_Bar_T_____E");

    // /* Foo<Bar<T>, Bar<E>> => Foo_Bar_T_____Bar_E */
    assert_eq!(mangle_path("Foo", &[generic_path("Bar", &[path("T")]),
                                    generic_path("Bar", &[path("E")])]),
               "Foo_Bar_T_____Bar_E");
}
