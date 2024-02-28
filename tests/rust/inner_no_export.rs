/// Inner docs should not be exported.
pub struct Inner {
    a: i32,
}

/// Outer docs should be exported.
#[repr(C)]
pub struct Outer {
    inner: *mut Inner,
}

#[no_mangle]
pub extern "C" fn root(a: *const Outer)
{ }
