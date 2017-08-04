/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use bindgen::ir::*;

pub fn mangle_path(name: &str, generic_values: &[Type]) -> String {
    assert!(!name.contains("_"));

    let mut out = String::from(name);
    if !generic_values.is_empty() {
        out.push_str("_"); // <
        for (i, ty) in generic_values.iter().enumerate() {
            if i != 0 {
                out.push_str("__"); // ,
            }
            append_type(ty, &mut out, mangle_path);
        }
        out.push_str("___"); // >
    }
    out
}

fn append_type(ty: &Type, out: &mut String, generic_handler: fn(name:&str, generic_values: &[Type]) -> String ) {
    match ty {
        &Type::Path(ref path, ref generic_values) => {
            out.push_str(&generic_handler(path, generic_values));
        }
        &Type::Primitive(ref primitive) => {
            out.push_str(primitive.to_repr_rust());
        }
        &Type::ConstPtr(..) => {
            unimplemented!();
        }
        &Type::Ptr(..) => {
            unimplemented!();
        }
        &Type::Array(..) => {
            unimplemented!();
        }
        &Type::FuncPtr(..) => {
            unimplemented!();
        }
    }
}

#[test]
fn mangle_test() {
    assert_eq!(mangle_path("Foo",
                           &vec![Type::Primitive(PrimitiveType::Float)]),
               "Foo_f32");

    assert_eq!(mangle_path("Foo",
                           &vec![Type::Path("Bar".to_owned(),
                                            vec![Type::Primitive(PrimitiveType::Float)])
                           ]),
               "Foo_Bar_f32___");
}
