#[no_mangle]
pub extern "cmse-nonsecure-entry" fn foo() {}

#[no_mangle]
pub extern "cmse-nonsecure-call" fn bar() {}
