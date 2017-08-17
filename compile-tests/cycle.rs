#[repr(C)]
pub struct Foo {
    a: i32,
    b: *const Bar,
}

#[repr(C)]
pub struct Bar {
    a: *mut Foo,
}

#[no_mangle]
pub extern "C" fn foo(f: Foo) {}
