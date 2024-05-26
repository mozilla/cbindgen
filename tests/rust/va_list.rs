use std::ffi::VaList;

#[no_mangle]
pub unsafe extern "C" fn va_list_test(mut ap: VaList) -> int32_t {
    ap.arg()
}

#[no_mangle]
pub unsafe extern "C" fn my_snprintf(buf: *mut c_char, n: size_t, format: *const c_char, mut ap: ...) -> int32_t {
    0
}
#[no_mangle]
pub unsafe extern "C" fn my_vsnprintf(buf: *mut c_char, n: size_t, format: *const c_char, mut ap: VaList) -> int32_t {
    0
}
