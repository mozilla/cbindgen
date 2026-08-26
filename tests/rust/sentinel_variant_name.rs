#[repr(u8)]
pub enum A {
    A1,
    A2,
    A3,
}

/// cbindgen:sentinel-variant-name=Last
#[repr(u8)]
pub enum B {
    B1,
    B2,
    B3,
}

#[no_mangle]
pub extern "C" fn root(a: A, b: B) {}
