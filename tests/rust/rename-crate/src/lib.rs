extern crate dependency as internal_name;

pub use internal_name::*;

#[no_mangle]
pub extern "C" fn root(a: Foo) {
}
