//! Test that only the used calling conventions are emitted as predefines

#[no_mangle]
pub extern "C" fn test_c() {}

#[no_mangle]
pub extern "cdecl" fn test_cdecl() {}

#[no_mangle]
pub extern "stdcall" fn test_stdcall() {}
