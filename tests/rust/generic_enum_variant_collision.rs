use std::os::raw::c_void;

#[repr(C)]
pub enum ResultA<T> {
    Ok(T),
    Err(*mut c_void),
}

#[repr(C)]
pub enum ResultB<T> {
    Ok(T),
    Err(*mut c_void),
}

#[repr(C)]
pub struct Payload {
    pub x: i32,
}

#[no_mangle]
pub extern "C" fn use_a(_a: ResultA<Payload>) {}

#[no_mangle]
pub extern "C" fn use_b(_b: ResultB<Payload>) {}
