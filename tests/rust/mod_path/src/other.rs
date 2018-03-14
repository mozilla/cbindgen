#[repr(C)]
pub struct ExportMe {
    val: u64
}

#[repr(C)]
pub struct DoNotExportMe {
    val: u64
}

pub const EXPORT_ME_TOO: u8 = 0x2a;

#[no_mangle]
pub unsafe extern "C" fn export_me(val: *mut ExportMe) { }
