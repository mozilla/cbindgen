#[repr(C)]
pub enum CResult<T, E> {
    Ok(T),
    Err(E),
}

#[repr(C)]
pub enum COption<T> {
    Some(T),
    None,
}

#[repr(C)]
pub struct ErrorInfo {
    code: i32,
    message: *const u8,
}

#[no_mangle]
pub extern "C" fn process_result(r: CResult<u32, ErrorInfo>) -> COption<u32> {
    match r {
        CResult::Ok(val) => COption::Some(val),
        CResult::Err(_) => COption::None,
    }
}

#[no_mangle]
pub extern "C" fn process_str_result(r: CResult<*const u8, i32>) -> COption<*const u8> {
    match r {
        CResult::Ok(val) => COption::Some(val),
        CResult::Err(_) => COption::None,
    }
}

#[no_mangle]
pub extern "C" fn get_option_int() -> COption<i32> {
    COption::Some(42)
}

#[no_mangle]
pub extern "C" fn get_option_str() -> COption<*const u8> {
    COption::None
} 