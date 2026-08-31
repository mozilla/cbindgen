/// Reachable from an exported function, so it is emitted even though its crate
/// does not contribute top-level items.
#[repr(C)]
pub struct UsedDepStruct {
    pub x: u32,
}

/// Not reachable, and its crate is not listed in `parse.extra_bindings`, so
/// `export.include_all` must not pick it up.
#[repr(C)]
pub struct UnusedDepStruct {
    pub y: u32,
}

/// Same gate as the types above: never emitted from a non-binding crate.
#[no_mangle]
pub extern "C" fn dep_only_fn() -> u32 {
    0
}
