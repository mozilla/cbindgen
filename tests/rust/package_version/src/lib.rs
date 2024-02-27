#[repr(C)]
pub struct Foo {
    #[cfg(not(feature = "cbindgen"))]
    bar: u64,
}

#[no_mangle]
pub extern "C" fn doit(_: &Foo) {}
