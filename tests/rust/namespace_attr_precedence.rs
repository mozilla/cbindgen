use std::os::raw::c_char;

/// A function without namespace annotation - should use global namespace
#[no_mangle]
pub extern "C" fn uses_global_namespace() {}

/// A function with per-item namespace - should override global namespace
/// cbindgen:namespace=ffi::bar
#[no_mangle]
pub extern "C" fn uses_item_namespace(a: *const c_char) {}

/// Another function without namespace annotation - should use global namespace
#[no_mangle]
pub extern "C" fn also_uses_global_namespace() {}
