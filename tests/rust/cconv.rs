//! Test that all calling conventions are emitted correctly

#[no_mangle]
pub extern "C" fn test_none() {}

#[no_mangle]
pub extern "C" fn test_c() {}

#[no_mangle]
pub extern "cdecl" fn test_cdecl() {}

#[no_mangle]
pub extern "stdcall" fn test_stdcall() {}

#[no_mangle]
pub extern "win64" fn test_win64() {}

#[no_mangle]
pub extern "sysv64" fn test_sysv64() {}

#[no_mangle]
pub extern "system" fn test_rust() {}

#[no_mangle]
// NOTE: Accepting on non-arm may be a bug https://github.com/rust-lang/rust/issues/57182
pub extern "aapcs" fn test_aapcs() {}

#[no_mangle]
pub extern "fastcall" fn test_fastcall() {}

#[no_mangle]
pub extern "thiscall" fn test_thiscall() {}

#[no_mangle]
pub extern "efiapi" fn test_efiapi() {}

#[no_mangle]
pub extern "C-unwind" fn test_c() {}

#[no_mangle]
pub extern "cdecl-unwind" fn test_cdecl() {}

#[no_mangle]
pub extern "stdcall-unwind" fn test_stdcall() {}

#[no_mangle]
pub extern "win64-unwind" fn test_win64() {}

#[no_mangle]
pub extern "sysv64-unwind" fn test_sysv64() {}

#[no_mangle]
pub extern "system-unwind" fn test_rust() {}

#[no_mangle]
// NOTE: Accepting on non-arm may be a bug https://github.com/rust-lang/rust/issues/57182
pub extern "aapcs-unwind" fn test_aapcs() {}

#[no_mangle]
pub extern "fastcall-unwind" fn test_fastcall() {}

#[no_mangle]
pub extern "thiscall-unwind" fn test_thiscall() {}
