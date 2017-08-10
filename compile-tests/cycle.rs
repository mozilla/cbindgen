
#[repr(C)]
pub struct A {
    b: *const B,
}

#[repr(C)]
pub struct B {
    a: *const A,
}

#[no_mangle]
pub extern "C" fn foo(a: A) {}
