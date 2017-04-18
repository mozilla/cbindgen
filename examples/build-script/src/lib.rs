#![allow(dead_code)]
#![allow(unused_variables)]

pub struct Opaque {
    x: i32,
    y: f32,
}

#[repr(C)]
pub struct Normal {
    x: i32,
    y: f32,
}

#[no_mangle]
pub extern "C" fn root(x: *mut Opaque, y: Normal)
{

}
