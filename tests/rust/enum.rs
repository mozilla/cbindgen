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

#[repr(usize)]
enum D {
    d1 = 0,
    d2 = 2,
    d3,
    d4 = 5,
}

#[repr(isize)]
enum E {
    e1 = 0,
    e2 = 2,
    e3,
    e4 = 5,
}

#[repr(u8)]
enum F {
    Foo(i16),
    Bar { x: u8, y: i16 },
    Baz
}

#[no_mangle]
pub extern "C" fn root(
    o: *mut Opaque,
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F
) { }
