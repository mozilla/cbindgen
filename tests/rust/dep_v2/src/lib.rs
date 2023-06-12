use dep_2_dep::dep_struct;

#[no_mangle]
pub unsafe extern "C" fn get_x(dep_struct: *const dep_struct) -> u32 {
    dep_struct.read().x
}
