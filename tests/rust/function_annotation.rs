#[no_mangle]
/// cbindgen:function-prefix=TEST_MACRO
/// cbindgen:function-postfix=TEST_MACRO
/// cbindgen:function-ident-prefix=TEST_MACRO
/// cbindgen:function-arg-ident-prefix[input]=_Nonnull
/// cbindgen:function-arg-prefix[input]=_In_
pub extern "C" fn root(input: *const u64, input_size: u64) {}
