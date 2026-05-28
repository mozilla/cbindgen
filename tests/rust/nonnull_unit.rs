use core::ptr::NonNull;

// Reproducer: generic argument is a ZST `()` which evaporates in parsing.
pub type MyId = NonNull<()>;

#[no_mangle]
pub extern "C" fn takes_id(id: MyId) {
    let _ = id;
}

#[no_mangle]
pub extern "C" fn takes_unit_ptr(id: *mut ()) {
    let _ = id;
}


