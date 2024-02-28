pub mod inner {
    /// Inner docs should not be exported.
    pub struct Inner;
}

#[repr(C)]
pub struct Outer {
    inner: *mut inner::Inner,
}

#[no_mangle]
pub extern "C" fn root(a: Outer)
{ }
