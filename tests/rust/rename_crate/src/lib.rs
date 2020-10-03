#![allow(unused_variables)]

extern crate dependency as internal_name;
extern crate renamed_dep;

pub use internal_name::*;
pub use renamed_dep::*;

#[no_mangle]
pub extern "C" fn root(a: Foo) {
}

#[no_mangle]
pub extern "C" fn renamed_func(a: RenamedTy) {
}


#[no_mangle]
pub extern "C" fn no_extern_func(a: ContainsNoExternTy) {
}
