#[repr(C)]
struct Foo {

}

#[cfg(feature = "extra_headers")]
#[no_mangle]
pub extern "C" fn extra_debug_fn() {
}

#[cfg(feature = "no_parse")]
pub extern "C" fn no_parse() {
    x;
}

#[cfg(feature = "cbindgen")]
#[no_mangle]
pub extern "C" fn cbindgen() {
}

#[no_mangle]
pub extern "C" fn root(a: Foo) {
}
