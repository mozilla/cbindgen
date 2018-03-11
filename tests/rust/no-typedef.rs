#[repr(C)]
pub struct simple {
    len: u64,
}

#[no_mangle]
pub extern "C" fn simple(simple: *const simple) -> *const simple {
    simple
}
