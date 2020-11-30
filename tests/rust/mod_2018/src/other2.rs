#[repr(C)]
pub struct ExportMe2 {
    val: u64
}

#[no_mangle]
pub unsafe extern "C" fn export_me_2(_: *mut ExportMe2) { }
