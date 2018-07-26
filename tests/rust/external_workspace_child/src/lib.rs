extern crate dep;

#[no_mangle]
pub extern "C" fn consume_ext(_ext: dep::ExtType) {
}
