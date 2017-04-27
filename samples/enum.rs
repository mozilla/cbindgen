enum Opaque {
    Foo(i32),
    Bar
}

#[repr(u32)]
enum A {
    x = 0,
    y = 2,
    z,
    w = 5,
}

#[repr(u16)]
enum B {
    x = 0,
    y = 2,
    z,
    w = 5,
}

#[repr(u8)]
enum C {
    x = 0,
    y = 2,
    z,
    w = 5,
}

#[no_mangle]
extern "C" fn root(x: *mut Opaque, y: A, z: B, w: C)
{

}
