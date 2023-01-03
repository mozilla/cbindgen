use std::ffi::VaList;

#[no_mangle]
pub unsafe extern "C" fn va_list_test(mut ap: VaList) -> int32_t {
    ap.arg()
}

#[no_mangle]
pub unsafe extern "C" fn va_list_test2(mut ap: ...) -> int32_t {
    ap.arg()
}
