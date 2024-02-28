pub mod inner {
    /// Inner docs should not be exported.
    pub struct Inner;
}

pub struct Outer {
    inner: inner::Inner,
}

#[no_mangle]
pub extern "C" fn root(a: Outer)
{ }
