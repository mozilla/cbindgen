#[repr(C)]
struct A {
    buf: *mut B,
    len: usize,
}

#[repr(C)]
struct B {
    something: i32,
    nested: A,
}

#[no_mangle]
pub extern "C" fn root(foo: &B) {}
