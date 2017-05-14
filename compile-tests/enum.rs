enum Opaque {
    Foo(i32),
    Bar
}

#[repr(u32)]
enum A {
    a1 = 0,
    a2 = 2,
    a3,
    a4 = 5,
}

#[repr(u16)]
enum B {
    b1 = 0,
    b2 = 2,
    b3,
    b4 = 5,
}

#[repr(u8)]
enum C {
    c1 = 0,
    c2 = 2,
    c3,
    c4 = 5,
}

#[no_mangle]
extern "C" fn root(x: *mut Opaque,
                   y: A,
                   z: B,
                   w: C)
{ }
