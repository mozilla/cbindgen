/// cbindgen:ignore
#[no_mangle]
pub extern "C" fn root() {}

/// cbindgen:ignore
///
/// Something else.
#[no_mangle]
pub extern "C" fn another_root() {}

#[repr(C)]
pub struct OneFieldIgnored {
    x: i32,
    /// cbindgen:ignore
    y: i64,
    z: i32,
}

#[repr(C)]
pub struct AllFieldsIgnored {
    /// cbindgen:ignore
    inner: i32,
}

#[no_mangle]
pub extern "C" fn no_ignore_root(one: OneFieldIgnored, all: AllFieldsIgnored) {}
