/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use bindgen::ir::*;

pub fn mangle_path(name: &str, generic_values: &[Type]) -> String {
    assert!(!name.contains("_"));

    let mut out = String::from(name);
    if !generic_values.is_empty() {
        out.push_str("_"); // <
        let mut should_close = false;
        for (i, ty) in generic_values.iter().enumerate() {
            if i != 0 {
                out.push_str("__"); // ,
            }
            if should_close {
                out.push_str("___"); // >
            }
            should_close = append_type(ty, &mut out, mangle_path);
        }
    }
    out
}

fn append_type(ty: &Type, out: &mut String, generic_handler: fn(&str, &[Type]) -> String) -> bool {
    match ty {
        &Type::Path(ref path, ref generic_values) => {
            out.push_str(&generic_handler(path, generic_values));
            true
        }
        &Type::Primitive(ref primitive) => {
            out.push_str(primitive.to_repr_rust());
            false
        }
        &Type::ConstPtr(..) => {
            unimplemented!()
        }
        &Type::Ptr(..) => {
            unimplemented!()
        }
        &Type::Array(..) => {
            unimplemented!()
        }
        &Type::FuncPtr(..) => {
            unimplemented!()
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
               "Foo_Bar_f32");

    assert_eq!(mangle_path("Vec", &[Type::Path("u8".into(), Vec::new())]), "Vec_u8");
    assert_eq!(mangle_path("Vec", &[Type::Path("Vec".into(), vec!(Type::Primitive(PrimitiveType::UInt8)))]),
               "Vec_Vec_u8");
    assert_eq!(mangle_path("Foo", &[Type::Path("Bar".into(), vec![Type::Path("T".into(), Vec::new()),]),
                                    Type::Path("E".into(), Vec::new())]),
               "Foo_Bar_T_____E");
}
