#[repr(C)]
pub struct Foo {
    bar: u64,
}

#[no_mangle]
pub extern "C" fn doit(_: &Foo) {}
