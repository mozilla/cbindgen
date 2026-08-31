use include_all_deps_dep::UsedDepStruct;

/// Not used by any exported item, but declared by the binding crate.
#[repr(C)]
pub struct UnusedLocalStruct {
    pub z: u32,
}

#[no_mangle]
pub unsafe extern "C" fn get_x(used: *const UsedDepStruct) -> u32 {
    (*used).x
}
