use std::ffi::VaList;

#[no_mangle]
pub unsafe extern "C" fn va_list_test(mut ap: VaList) -> int32_t {
    ap.arg()
}

#[no_mangle]
pub unsafe extern "C" fn va_list_test2(mut ap: ...) -> int32_t {
    ap.arg()
}

type VaListFnPtr = Option<unsafe extern "C" fn(VaList) -> int32_t>;
type VaListFnPtr2 = Option<unsafe extern "C" fn(...) -> int32_t>;

#[repr(C)]
struct Interface<T> {
    fn1: T,
}

#[no_mangle]
pub extern "C" fn va_list_fn_ptrs(
    fn1: Option<unsafe extern "C" fn(VaList) -> int32_t>,
    fn2: Option<unsafe extern "C" fn(...) -> int32_t>,
    fn3: VaListFnPtr,
    fn4: VaListFnPtr2,
    fn5: Interface<Option<unsafe extern "C" fn(VaList) -> int32_t>>,
    fn6: Interface<Option<unsafe extern "C" fn(...) -> int32_t>>,
) {
}
