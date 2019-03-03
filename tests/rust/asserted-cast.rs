/// cbindgen:prefix-with-name
#[repr(C, u8)]
pub enum H {
    Foo(i16),
    Bar { x: u8, y: i16 },
    Baz
}

/// cbindgen:prefix-with-name
#[repr(C, u8, u16)]
pub enum I {
    Foo(i16),
    Bar { x: u8, y: i16 },
    Baz
}

/// cbindgen:prefix-with-name
#[repr(C, u8)]
pub enum J {
    Foo(i16),
    Bar { x: u8, y: i16 },
    Baz
}

/// cbindgen:prefix-with-name
#[repr(u8)]
pub enum K {
    Foo(i16),
    Bar { x: u8, y: i16 },
    Baz
}

#[no_mangle]
pub extern "C" fn foo(
    h: H,
    i: I,
    j: J,
    k: K,
) {}
