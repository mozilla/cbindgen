#[repr(C)]
pub struct Foo {
    field: u32,
}

impl Foo {
    pub const FIELD_RELATED_CONSTANT: u32 = 0;
}

#[repr(C)]
pub struct Bar {
    field: u32,
}

impl Bar {
    pub const FIELD_RELATED_CONSTANT: u32 = 0;
}

#[no_mangle]
pub extern "C" fn root(a: Foo, b: Bar) {}
