#[repr(C)]
pub struct HasBitfields {
    #[cfg(not(feature = "cbindgen"))]
    foo_and_bar: u64,

    #[cfg(feature = "cbindgen")]
    /// cbindgen:bitfield=8
    foo: u64,
    #[cfg(feature = "cbindgen")]
    /// cbindgen:bitfield=56
    bar: u64,
}

#[no_mangle]
pub extern "C" fn root(_: &HasBitfields) {}
