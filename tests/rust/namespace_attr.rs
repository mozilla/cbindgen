use std::os::raw::c_char;

/// A function without namespace attribute - uses global namespace
#[no_mangle]
pub extern "C" fn global_function() {}

/// A function with a single namespace
#[cbindgen_macro::namespace = "ffi"]
#[no_mangle]
pub extern "C" fn ffi_function() {}

/// A function with nested namespaces using :: separator
#[cbindgen_macro::namespace = "ffi::inner"]
#[no_mangle]
pub extern "C" fn nested_function(a: *const c_char) {}

/// Another function with the same namespace to test grouping
#[cbindgen_macro::namespace = "ffi::inner"]
#[no_mangle]
pub extern "C" fn another_nested_function() {}

/// A function with a different nested namespace
#[cbindgen_macro::namespace = "other::ns"]
#[no_mangle]
pub extern "C" fn other_namespace_function() {}
